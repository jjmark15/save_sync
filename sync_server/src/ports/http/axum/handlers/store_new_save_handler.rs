use std::option::Option::Some;

use axum::extract::{ContentLengthLimit, Extension, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::ports::http::axum::error::MissingMandatoryDataError;
use crate::ports::http::axum::error_handling::HandleAppResult;
use crate::ports::http::axum::handlers::app_error::AppError;
use crate::ports::http::axum::models::{StoreNewSaveRequest, StoreNewSaveResponse};
use crate::DynApplicationService;

pub(crate) async fn store_new_save_handler(
    Extension(application_service): Extension<DynApplicationService>,
    ContentLengthLimit(multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 /* 250kb */
        },
    >,
) -> Result<impl IntoResponse, AppError> {
    let request = game_save_from_form(multipart).await?;
    application_service
        .store_new_save(
            request.game_name().to_string(),
            request.file_name().to_string(),
            request.save_data(),
        )
        .await
        .handle_app_result(|save_id| {
            (
                StatusCode::CREATED,
                Json(StoreNewSaveResponse::new(save_id.to_string())),
            )
        })
}

async fn game_save_from_form(
    mut multipart: Multipart,
) -> Result<StoreNewSaveRequest, MissingMandatoryDataError> {
    let mut game_name: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut save_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        if name == "game_name" {
            game_name = Some(String::from_utf8(field.bytes().await.unwrap().to_vec()).unwrap());
        } else if let Some(file_name_value) = field.file_name() {
            file_name = Some(file_name_value.to_string());
            save_data = Some(field.bytes().await.unwrap().to_vec());
        }
    }

    if file_name.is_none() || save_data.is_none() || game_name.is_none() {
        return Err(MissingMandatoryDataError::new());
    }

    Ok(StoreNewSaveRequest::new(
        game_name.unwrap(),
        save_data.unwrap(),
        file_name.unwrap(),
    ))
}
