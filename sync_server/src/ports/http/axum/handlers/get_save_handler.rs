use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::domain::Save;
use crate::ports::http::axum::models::GetSaveResponse;
use crate::DynApplicationService;

pub(crate) async fn get_save_handler(
    Extension(application_service): Extension<DynApplicationService>,
    Path(save_id): Path<String>,
) -> impl IntoResponse {
    let save_id = Uuid::parse_str(save_id.as_str()).unwrap();
    let save = application_service.get_save(save_id).await.unwrap();
    (StatusCode::OK, Json(response_body(save)))
}

fn response_body(save: Save) -> GetSaveResponse {
    GetSaveResponse::new(
        save.id().value().to_string(),
        save.version().value(),
        save.game_name().value().to_string(),
        save.file_name().value().to_string(),
        save.data().value(),
    )
}
