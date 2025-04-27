use crate::{
    AsRaw, Device, Stream,
    bindings::{infinirtEvent_t, infinirtEventQuery, infinirtEventStatus_t as Status},
};
use std::ptr::null_mut;

/// 一个 InfiniTensor 事件。
#[repr(transparent)]
pub struct Event(infinirtEvent_t);

impl Device {
    /// 创建一个与当前设备上下文关联的新事件。
    pub fn event(&self) -> Event {
        let mut event = null_mut();
        infini!(infinirtEventCreate(&mut event));
        Event(event)
    }
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}

impl Drop for Event {
    fn drop(&mut self) {
        infini!(infinirtEventDestroy(self.0))
    }
}

impl AsRaw for Event {
    type Raw = infinirtEvent_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}

impl Event {
    /// 阻塞当前主机线程，直到此事件完成。
    ///
    /// 如果事件尚未被记录 (`Stream::record`)，行为未定义（可能立即返回或阻塞）。
    #[inline]
    pub fn synchronize(&self) {
        infini!(infinirtEventSynchronize(self.0))
    }

    /// 查询事件的状态，检查是否已完成。
    ///
    /// 这是一个非阻塞操作。
    #[inline]
    pub fn is_complete(&self) -> bool {
        let mut status = Status::INFINIRT_EVENT_COMPLETE;
        unsafe {
            infinirtEventQuery(self.0, &mut status);
        }
        match status {
            Status::INFINIRT_EVENT_COMPLETE => true,
            Status::INFINIRT_EVENT_NOT_READY => false,
        }
    }
}

impl Stream {
    /// 在计算流中记录一个事件。
    ///
    /// 当流执行到此点时，事件被视为发生。
    #[inline]
    pub fn record(&self, event: &mut Event) {
        infini!(infinirtEventRecord(event.0, self.as_raw()))
    }

    /// 使计算流等待一个事件。
    ///
    /// 流的执行将暂停，直到指定的事件完成。
    #[inline]
    pub fn wait(&self, event: &Event) {
        infini!(infinirtStreamWaitEvent(self.as_raw(), event.0))
    }
}
