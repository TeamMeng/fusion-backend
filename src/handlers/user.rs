use super::Output;
use crate::{AppError, AppState, CreateUser, SigninUser};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.create_user(input).await?;

    Ok((
        StatusCode::OK,
        Json(Output::new(
            true,
            format!("user by {} by created", user.email),
        ))
        .into_response(),
    ))
}

pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.signin(input).await?;

    Ok((
        StatusCode::OK,
        Json(Output::new(
            true,
            format!("user by {} by login", user.email),
        ))
        .into_response(),
    ))
}
