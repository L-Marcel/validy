pub struct Valid<T>(pub T);
#[cfg(feature = "axum_multipart")]
pub struct ValidMultipart<T>(pub T);
