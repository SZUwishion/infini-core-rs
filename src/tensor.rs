use crate::{AsRaw, bindings::infiniopTensorDescriptor_t, infiniDtype_t};
pub use digit_layout::{DigitLayout, types};
use std::ptr::null_mut;

/// 一个 InfiniCore 张量描述符。
#[repr(transparent)]
pub struct Tensor(infiniopTensorDescriptor_t);

fn data_layout(dt: DigitLayout) -> infiniDtype_t {
    match dt {
        types::I8 => infiniDtype_t::INFINI_DTYPE_I8,
        types::I16 => infiniDtype_t::INFINI_DTYPE_I16,
        types::I32 => infiniDtype_t::INFINI_DTYPE_I32,
        types::I64 => infiniDtype_t::INFINI_DTYPE_I64,
        types::U8 => infiniDtype_t::INFINI_DTYPE_U8,
        types::U16 => infiniDtype_t::INFINI_DTYPE_U16,
        types::U32 => infiniDtype_t::INFINI_DTYPE_U32,
        types::U64 => infiniDtype_t::INFINI_DTYPE_U64,
        types::F16 => infiniDtype_t::INFINI_DTYPE_F16,
        types::F32 => infiniDtype_t::INFINI_DTYPE_F32,
        types::F64 => infiniDtype_t::INFINI_DTYPE_F64,
        types::BF16 => infiniDtype_t::INFINI_DTYPE_BF16,
        _ => panic!("Unsupported data type: {:?}", dt),
    }
}

impl Tensor {
    /// 创建一个新的张量描述符。
    ///
    /// # Arguments
    ///
    /// * `dt`: 张量的数据类型，使用 `digit_layout::DigitLayout` 表示。
    /// * `shape`: 张量的形状（维度大小），一个包含 `usize` 的迭代器。
    /// * `strides`: 张量的步长（以字节为单位），一个包含 `isize` 的迭代器。
    ///   步长表示在每个维度上移动一个元素需要跳过的字节数。
    ///
    /// # Panics
    ///
    /// * 如果 `shape` 和 `strides` 的维度数量不匹配。
    /// * 如果 `dt` 是不支持的数据类型（由 `data_layout` 函数 panic）。
    /// * 如果底层的 `infiniopCreateTensorDescriptor` 调用失败（由 `infini!` 宏 panic）。
    pub fn new(
        dt: DigitLayout,
        shape: impl IntoIterator<Item = usize>,
        strides: impl IntoIterator<Item = isize>,
    ) -> Self {
        let ele = dt.nbytes() as isize;
        let shape: Vec<_> = shape.into_iter().map(|x| x as _).collect();
        let strides: Vec<_> = strides.into_iter().map(|x| (x / ele) as _).collect();
        let ndim = shape.len();
        assert_eq!(strides.len(), ndim);

        let mut ptr = null_mut();
        infini!(infiniopCreateTensorDescriptor(
            &mut ptr,
            ndim as _,
            shape.as_ptr(),
            strides.as_ptr(),
            data_layout(dt),
        ));
        Self(ptr)
    }
}

impl Drop for Tensor {
    fn drop(&mut self) {
        infini!(infiniopDestroyTensorDescriptor(self.0))
    }
}

unsafe impl Send for Tensor {}
unsafe impl Sync for Tensor {}

impl AsRaw for Tensor {
    type Raw = infiniopTensorDescriptor_t;
    #[inline]
    unsafe fn as_raw(&self) -> Self::Raw {
        self.0
    }
}
