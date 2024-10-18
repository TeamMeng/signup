use crate::{
    error::AppError,
    model::{Customer, ProfileDetails},
};
use axum::{http::StatusCode, Extension, Json};

pub async fn profile_handler(
    Extension(customer): Extension<Customer>,
) -> Result<(StatusCode, Json<ProfileDetails>), AppError> {
    let profile = customer.to_response();
    Ok((StatusCode::OK, Json(profile)))
}
