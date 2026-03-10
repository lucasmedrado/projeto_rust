use axum::extract::{FromRequest, Request};
use axum::{async_trait, Json};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where 
    T: DeserializeOwned + Validate,
    S: Send + Sync,
  {
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
      let Json(value) = Json::<T>::from_request(req, state)
      .await
      .map_err(|rejection| (StatusCode::BAD_REQUEST, rejection.to_string()))?;

      value.validate()
      .map_err(|errors| (StatusCode::UNPROCESSABLE_ENTITY, format!("{}", errors)))?;

      Ok(ValidatedJson(value))
    }
  }
    
