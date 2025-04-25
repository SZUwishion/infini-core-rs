use crate::{bindings::infinirtStream_t, AsRaw, Device};
use std::{ffi::c_void, ptr::null_mut};

#[repr(transparent)]
pub struct Stream(infinirtStream_t);

impl Device {
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
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtStreamSynchronize(self.0))
    }

    #[inline]
    pub fn get_device(&self) -> Device {
        let mut ty = crate::infiniDevice_t::INFINI_DEVICE_CPU;
        let mut id = 0;
        infini!(infinirtGetDevice(&mut ty, &mut id));
        Device { ty, id }
    }
}