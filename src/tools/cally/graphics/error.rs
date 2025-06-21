pub enum RendererError {
    OtherError(String),
}

impl From<String> for RendererError {
    fn from(error: String) -> Self {
        RendererError::OtherError(error)
    }
}
