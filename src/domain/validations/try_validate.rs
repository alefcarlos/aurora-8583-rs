pub trait TryValidate<T, E> {
    fn validate(&self) -> Result<T, E>;
}