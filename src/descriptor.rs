use crate::{AsRaw, bindings::infiniStatus_t};
use std::ptr::null_mut;

/// 一个通用的描述符包装器，用于管理底层 C 库分配的资源。
pub struct Descriptor<T> {
    ptr: *mut T,
    destroyer: unsafe extern "C" fn(*mut T) -> infiniStatus_t,
}

impl<T> Descriptor<T> {
    /// 创建一个新的 `Descriptor` 实例。
    ///
    /// 这个函数接收一个闭包 `f`，该闭包负责调用 C API 来创建资源并将指针写入提供的 `&mut *mut T`。
    /// 以及一个 `destroyer` 函数指针，该函数将在 `Descriptor` 被 `drop` 时用于释放资源。
    pub fn new(
        f: impl FnOnce(&mut *mut T),
        destroyer: unsafe extern "C" fn(*mut T) -> infiniStatus_t,
    ) -> Self {
        let mut ptr = null_mut();
        f(&mut ptr);
        Self { ptr, destroyer }
    }
}

impl<T> Drop for Descriptor<T> {
    fn drop(&mut self) {
        assert_eq!(
            unsafe { (self.destroyer)(self.ptr) },
            infiniStatus_t::INFINI_STATUS_SUCCESS
        )
    }
}

unsafe impl<T> Send for Descriptor<T> {}
unsafe impl<T> Sync for Descriptor<T> {}

impl<T: 'static> AsRaw for Descriptor<T> {
    type Raw = *mut T;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.ptr
    }
}
