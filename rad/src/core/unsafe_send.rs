// Allows for some objects to be used across multiple threads even when they do not support the send trait
// Should be used with caution since objects should have the send trait instead
pub struct UnsafeSend<T>{
  pub inner: T,
}

unsafe impl<T> Send for UnsafeSend<T> { }

#[allow(dead_code)]
impl<T> UnsafeSend<T> {
  pub unsafe fn new(t: T) -> Self {
      Self{ inner: t }
  }
  pub fn into_inner(self) -> T {
      self.inner
  }
}