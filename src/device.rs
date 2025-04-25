use crate::infiniDevice_t;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Device {
    pub ty: infiniDevice_t,
    pub id: i32,
}

impl Device {
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtDeviceSynchronize())
    }

    #[inline]
    pub fn set_device(&self) {
        infini!(infinirtSetDevice(self.ty, self.id))
    }
}