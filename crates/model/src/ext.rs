#[derive(Debug, thiserror::Error, PartialEq, derive_getters::Getters)]
#[error("{error}: json={json:?}")]
pub struct JsonCtxError<E> {
    error: E,
    json: Box<serde_json::Value>,
}

pub(crate) trait WithJsonContextExt<T, E> {
    fn with_json(self, json: &serde_json::Value) -> Result<T, JsonCtxError<E>>;
}
impl<E, T> WithJsonContextExt<T, E> for Result<T, E> {
    fn with_json(self, json: &serde_json::Value) -> Result<T, JsonCtxError<E>> {
        self.map_err(|err| JsonCtxError {
            error: err,
            json: json.clone().into(),
        })
    }
}

pub(crate) trait FromJsonExt<T, E>
where
    Result<T, E>: WithJsonContextExt<T, E>,
{
    fn from_json_with_ctx(value: &serde_json::Value) -> Result<T, JsonCtxError<E>> {
        Self::from_json_without_ctx(value).with_json(value)
    }
    fn from_json_without_ctx(value: &serde_json::Value) -> Result<T, E>;
}
