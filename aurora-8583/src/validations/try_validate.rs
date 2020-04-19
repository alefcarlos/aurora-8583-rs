/// Creates a validation
/// 
/// # Example
/// `TryValidate<T>` can be implemented as follows:
/// 
/// ```
/// use aurora_8583::TryValidate;
/// 
/// struct GreaterThanZero(i32);
/// 
/// impl TryValidate<bool> for GreaterThanZero {
///     type Error = &'static str;
///     fn try_validate(&self) -> Result<bool, Self::Error> {
///         if self.0 <= 0 {
///             Err("GreaterThanZero only accepts value superior than zero!")
///         } else {
///             Ok(true)
///         }
///     }
/// }
/// 
/// let data = GreaterThanZero(1);
/// 
/// assert!(data.try_validate().is_ok());
/// ```
pub trait TryValidate<T> {
    type Error;
    fn try_validate(&self) -> Result<T, Self::Error>;
}