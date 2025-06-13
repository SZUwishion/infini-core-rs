use crate::infiniDevice_t;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceType {
    CPU,
    CUDA,
}

/// 一个 InfiniCore 计算设备。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Device {
    /// 设备类型
    pub ty: infiniDevice_t,
    /// 设备 ID
    pub id: i32,
}

impl Device {
    pub fn new(ty: DeviceType, id: i32) -> Self {
        Self {
            ty: match ty {
                DeviceType::CPU => infiniDevice_t::INFINI_DEVICE_CPU,
                DeviceType::CUDA => infiniDevice_t::INFINI_DEVICE_NVIDIA,
            },
            id,
        }
    }

    pub fn default() -> Self {
        Self {
            ty: infiniDevice_t::INFINI_DEVICE_CPU,
            id: 0,
        }
    }

    pub fn set(&mut self, ty: DeviceType, id: i32){
        self.ty = match ty {
            DeviceType::CPU => infiniDevice_t::INFINI_DEVICE_CPU,
            DeviceType::CUDA => infiniDevice_t::INFINI_DEVICE_NVIDIA,
        };
        self.id = id;
    }

    pub fn get(&self) -> DeviceType {
        match self.ty {
            infiniDevice_t::INFINI_DEVICE_CPU => DeviceType::CPU,
            infiniDevice_t::INFINI_DEVICE_NVIDIA => DeviceType::CUDA,
            _ => panic!("Invalid device type"),
        }
    }

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
