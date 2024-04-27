pub mod task;

pub trait Query<T> {
    async fn execute(&self) -> T;
}
