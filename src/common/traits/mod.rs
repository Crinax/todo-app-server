pub trait IntoInner<T> {
  fn into_inner(self) -> T;
}

pub trait AsInner<T> {
  fn as_inner(&self) -> T;
}
