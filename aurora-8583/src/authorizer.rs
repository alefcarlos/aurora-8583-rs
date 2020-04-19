pub trait TryAuthorizerTransaction<T> {
    type Error;

    fn try_authorize(&self) -> Result<T, Self::Error>;
}
