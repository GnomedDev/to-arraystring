use core::ops::Deref;

use arrayvec::ArrayString;

#[allow(clippy::module_name_repetitions)]
pub trait ArrayStringErased: Deref<Target = str> + 'static {}

impl<const CAP: usize> ArrayStringErased for ArrayString<CAP> {}
