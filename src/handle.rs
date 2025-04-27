use crate::{AsRaw, bindings::infiniopHandle_t};
use std::ptr::null_mut;

/// 一个 InfiniCore 操作句柄 (`infiniopHandle_t`)。
#[repr(transparent)]
pub struct Handle(infiniopHandle_t);

impl Handle {
    /// 创建一个新的 InfiniCore 操作句柄。
    pub fn new() -> Self {
        let mut ptr = null_mut();
        infini!(infiniopCreateHandle(&mut ptr));
        Self(ptr)
    }
}

impl Default for Handle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        infini!(infiniopDestroyHandle(self.0))
    }
}

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl AsRaw for Handle {
    type Raw = infiniopHandle_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}
