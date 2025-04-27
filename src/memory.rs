use crate::{AsRaw, Device, Stream};
use std::{
    alloc::Layout,
    mem::forget,
    ops::{Deref, DerefMut},
    os::raw::c_void,
    ptr::{NonNull, null_mut},
    slice::{from_raw_parts, from_raw_parts_mut},
};

/// 一个标记类型，表示设备内存中的一个字节。
///
/// 主要用于类型系统，以区分设备指针和主机指针，尤其是在 `memcpy` 操作中。
/// 它是一个零大小类型（ZST），不占用实际内存。
#[repr(transparent)]
pub struct DevByte(u8);

impl Device {
    /// 在设备之间同步复制内存。
    #[inline]
    pub fn memcpy_d2d(&self, dst: &mut [DevByte], src: &[DevByte]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        if len > 0 {
            infini!(infinirtMemcpy(
                dst,
                src,
                len,
                infinirtMemcpyKind_t::INFINIRT_MEMCPY_D2D
            ))
        }
    }

    /// 将主机内存同步复制到设备内存。
    #[inline]
    pub fn memcpy_h2d<T: Copy>(&self, dst: &mut [DevByte], src: &[T]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        if len > 0 {
            infini!(infinirtMemcpy(
                dst,
                src,
                len,
                infinirtMemcpyKind_t::INFINIRT_MEMCPY_H2D
            ))
        }
    }

    /// 将设备内存同步复制到主机内存。
    #[inline]
    pub fn memcpy_d2h<T: Copy>(&self, dst: &mut [T], src: &[DevByte]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        if len > 0 {
            infini!(infinirtMemcpy(
                dst,
                src,
                len,
                infinirtMemcpyKind_t::INFINIRT_MEMCPY_D2H
            ))
        }
    }
}

impl Stream {
    /// 在设备之间异步复制内存。
    ///
    /// 操作将在指定的流上排队。
    #[inline]
    pub fn memcpy_d2d(&self, dst: &mut [DevByte], src: &[DevByte]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        if len > 0 {
            infini!(infinirtMemcpyAsync(
                dst,
                src,
                len,
                infinirtMemcpyKind_t::INFINIRT_MEMCPY_D2D,
                self.as_raw()
            ))
        }
    }

    /// 将主机内存异步复制到设备内存。
    ///
    /// 操作将在指定的流上排队。
    #[inline]
    pub fn memcpy_h2d<T: Copy>(&self, dst: &mut [DevByte], src: &[T]) {
        let (dst, src, len) = memcpy_ptr(dst, src);
        if len > 0 {
            infini!(infinirtMemcpyAsync(
                dst,
                src,
                len,
                infinirtMemcpyKind_t::INFINIRT_MEMCPY_H2D,
                self.as_raw()
            ))
        }
    }
}

#[inline]
fn memcpy_ptr<T, U>(dst: &mut [T], src: &[U]) -> (*mut c_void, *const c_void, usize) {
    let len = size_of_val(dst);
    assert_eq!(len, size_of_val(src));
    (dst.as_mut_ptr().cast(), src.as_ptr().cast(), len)
}

/// 表示在设备上分配的一块内存区域（Blob）。
///
/// 负责管理设备内存的分配和释放。
/// 通过 `Deref` 和 `DerefMut` 提供对内存的切片访问（作为 `[DevByte]`）。
pub struct DevBlob {
    ptr: NonNull<DevByte>,
    len: usize,
}

impl Device {
    /// 在设备上同步分配指定类型的内存。
    pub fn malloc<T: Copy>(&self, len: usize) -> DevBlob {
        let layout = Layout::array::<T>(len).unwrap();
        let len = layout.size();

        DevBlob {
            ptr: if len == 0 {
                NonNull::dangling()
            } else {
                let mut ptr = null_mut();
                infini!(infinirtMalloc(&mut ptr, len));
                NonNull::new(ptr).unwrap().cast()
            },
            len,
        }
    }

    /// 从主机内存数据同步创建设备内存 Blob 并复制内容。
    pub fn from_host<T: Copy>(&self, data: &[T]) -> DevBlob {
        let src = data.as_ptr().cast();
        let len = size_of_val(data);

        DevBlob {
            ptr: if len == 0 {
                NonNull::dangling()
            } else {
                let mut ptr = null_mut();
                infini!(infinirtMalloc(&mut ptr, len));
                infini!(infinirtMemcpy(
                    ptr,
                    src,
                    len,
                    infinirtMemcpyKind_t::INFINIRT_MEMCPY_H2D
                ));
                NonNull::new(ptr).unwrap().cast()
            },
            len,
        }
    }
}

impl Stream {
    /// 在设备上异步分配指定类型的内存。
    ///
    /// 分配操作将在指定的流上排队。
    pub fn malloc<T: Copy>(&self, len: usize) -> DevBlob {
        let layout = Layout::array::<T>(len).unwrap();
        let len = layout.size();

        DevBlob {
            ptr: if len == 0 {
                NonNull::dangling()
            } else {
                let raw = unsafe { self.as_raw() };
                let mut ptr = null_mut();
                infini!(infinirtMallocAsync(&mut ptr, len, raw));
                NonNull::new(ptr).unwrap().cast()
            },
            len,
        }
    }

    /// 从主机内存数据异步创建设备内存 Blob 并复制内容。
    ///
    /// 分配和复制操作将在指定的流上排队。
    pub fn from_host<T: Copy>(&self, data: &[T]) -> DevBlob {
        let src = data.as_ptr().cast();
        let len = size_of_val(data);

        DevBlob {
            ptr: if len == 0 {
                NonNull::dangling()
            } else {
                let raw = unsafe { self.as_raw() };
                let mut ptr = null_mut();
                infini!(infinirtMallocAsync(&mut ptr, len, raw));
                infini!(infinirtMemcpyAsync(
                    ptr,
                    src,
                    len,
                    infinirtMemcpyKind_t::INFINIRT_MEMCPY_H2D,
                    raw
                ));
                NonNull::new(ptr).unwrap().cast()
            },
            len,
        }
    }

    /// 在指定的流上异步释放设备内存 Blob。
    pub fn free(&self, blob: DevBlob) {
        if blob.len == 0 {
            return;
        }

        let &DevBlob { ptr, .. } = &blob;
        forget(blob);

        infini!(infinirtFreeAsync(ptr.as_ptr().cast(), self.as_raw()))
    }
}

impl Drop for DevBlob {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }

        infini!(infinirtFree(self.ptr.as_ptr().cast(),))
    }
}

unsafe impl Send for DevBlob {}
unsafe impl Sync for DevBlob {}

impl AsRaw for DevBlob {
    type Raw = *mut DevByte;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.ptr.as_ptr()
    }
}

impl Deref for DevBlob {
    type Target = [DevByte];
    #[inline]
    fn deref(&self) -> &Self::Target {
        if self.len == 0 {
            &[]
        } else {
            unsafe { from_raw_parts(self.ptr.as_ptr(), self.len) }
        }
    }
}

impl DerefMut for DevBlob {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
        }
    }
}

/// 表示在主机上分配的一块内存区域（Blob），通常用于与设备进行高效交互（例如，锁页内存）。
///
/// 负责管理主机端特殊内存（如锁页内存）的分配和释放。
/// 通过 `Deref` 和 `DerefMut` 提供对内存的切片访问（作为 `[u8]`）。
pub struct HostBlob {
    ptr: NonNull<u8>,
    len: usize,
}

impl Device {
    /// 在主机上同步分配指定类型的“固定”（pinned）或“主机映射”（host-mapped）内存。
    pub fn malloc_host<T: Copy>(&self, len: usize) -> HostBlob {
        let layout = Layout::array::<T>(len).unwrap();
        let len = layout.size();

        HostBlob {
            ptr: if len == 0 {
                NonNull::dangling()
            } else {
                let mut ptr = null_mut();
                infini!(infinirtMallocHost(&mut ptr, len));
                NonNull::new(ptr).unwrap().cast()
            },
            len,
        }
    }
}

impl Drop for HostBlob {
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }

        infini!(infinirtFreeHost(self.ptr.as_ptr().cast(),))
    }
}

unsafe impl Send for HostBlob {}
unsafe impl Sync for HostBlob {}

impl AsRaw for HostBlob {
    type Raw = *mut u8;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.ptr.as_ptr()
    }
}

impl Deref for HostBlob {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        if self.len == 0 {
            &[]
        } else {
            unsafe { from_raw_parts(self.ptr.as_ptr(), self.len) }
        }
    }
}

impl DerefMut for HostBlob {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.len == 0 {
            &mut []
        } else {
            unsafe { from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
        }
    }
}
