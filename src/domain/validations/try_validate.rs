pub trait TryValidate<T, E> {
    fn try_validate(&self) -> Result<T, E>;
}