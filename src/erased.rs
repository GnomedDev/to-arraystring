use core::ops::Deref;

use arrayvec::ArrayString;

pub trait ArrayStringErased: Deref<Target = str> + 'static {}

impl<const CAP: usize> ArrayStringErased for ArrayString<CAP> {}
