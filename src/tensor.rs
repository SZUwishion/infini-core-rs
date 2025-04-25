use crate::{bindings::infiniopTensorDescriptor_t, infiniDtype_t, AsRaw};
use digit_layout::{DigitLayout, types};
use std::ptr::null_mut;

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