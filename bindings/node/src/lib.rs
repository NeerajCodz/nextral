use napi::bindgen_prelude::{Error, Result};
use napi_derive::napi;

#[napi]
pub fn lexical_score(text: String, query: String) -> Result<f64> {
    nextral::scoring::try_lexical_score(&text, &query)
        .map(|score| score as f64)
        .map_err(|error| Error::from_reason(error.to_string()))
}
