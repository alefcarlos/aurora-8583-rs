pub trait TryExecute<T, E> {
    fn execute(&self) -> Result<T, E>;
}