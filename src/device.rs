use crate::infiniDevice_t;

/// 一个 InfiniCore 计算设备。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Device {
    /// 设备类型
    pub ty: infiniDevice_t,
    /// 设备 ID
    pub id: i32,
}

impl Device {
    /// 同步当前设备
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtDeviceSynchronize())
    }

    /// 设置当前活动的 InfiniCore 设备。
    #[inline]
    pub fn set_device(&self) {
        infini!(infinirtSetDevice(self.ty, self.id))
    }
}
