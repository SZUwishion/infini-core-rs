use crate::{AsRaw, Device, bindings::infinirtStream_t};
use std::ptr::null_mut;

/// 一个 InfiniCore 计算流。
#[repr(transparent)]
pub struct Stream(infinirtStream_t);

impl Device {
    /// 在此设备上创建一个新的计算流。
    pub fn stream(&self) -> Stream {
        let mut stream = null_mut();
        infini!(infinirtStreamCreate(&mut stream));
        Stream(stream)
    }
}

unsafe impl Send for Stream {}
unsafe impl Sync for Stream {}

impl Drop for Stream {
    fn drop(&mut self) {
        infini!(infinirtStreamDestroy(self.0))
    }
}

impl AsRaw for Stream {
    type Raw = infinirtStream_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl Stream {
    /// 等待此流中所有先前提交的任务完成。
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtStreamSynchronize(self.0))
    }

    /// 获取与当前 InfiniCore 上下文关联的设备。
    #[inline]
    pub fn get_device(&self) -> Device {
        let mut ty = crate::infiniDevice_t::INFINI_DEVICE_CPU;
        let mut id = 0;
        infini!(infinirtGetDevice(&mut ty, &mut id));
        Device { ty, id }
    }
}
