//! InfiniCore Rust 核心库绑定。
//!
//! 这个 crate 提供了对底层 InfiniCore C 库（infinirt 和 infiniop）的安全 Rust 封装。
#![cfg(infini)]
// #![deny(warnings, missing_docs)]

/// 包含从 C 库生成的原始绑定的模块。
#[macro_use]
/// 规范命名
#[allow(non_camel_case_types)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    /// 包装对底层 InfiniCore C 函数的调用。
    ///
    /// 它执行 `unsafe` 调用，并断言返回的状态码是 `INFINI_STATUS_SUCCESS`。
    /// 如果调用失败，程序将 panic。
    #[macro_export]
    macro_rules! infini {
        ($f:expr) => {{
            // 允许未使用的导入
            #[allow(unused_imports)]
            use $crate::bindings::*;
            // 允许未使用的 `unsafe` 和宏元变量
            #[allow(unused_unsafe, clippy::macro_metavars_in_unsafe)]
            let err = unsafe { $f };
            assert_eq!(err, infiniStatus_t::INFINI_STATUS_SUCCESS);
        }};
    }
}

use bindings::{infiniDevice_t, infiniDtype_t};

/// 初始化 InfiniCore 运行时环境
#[inline]
pub fn init() {
    infini!(infinirtInit());
}

/// infinirt
mod device;
mod event;
mod memory;
mod stream;

pub use device::Device;
pub use event::Event;
pub use memory::{DevBlob, DevByte, HostBlob};
pub use stream::Stream;

/// infiniop
mod descriptor;
mod handle;
mod tensor;

pub use descriptor::Descriptor;
pub use handle::Handle;
pub use tensor::Tensor;

/// 资源的原始形式的表示。通常来自底层库的定义。
pub trait AsRaw {
    /// 原始形式的类型。
    type Raw: Unpin + 'static;
    /// # Safety
    ///
    /// The caller must ensure that the returned item is dropped before the original item.
    unsafe fn as_raw(&self) -> Self::Raw;
}
