use axum::extract::Path;
use axum::http::StatusCode;
use axum::debug_handler;
use axum::response::{IntoResponse, Json};
use axum::Extension;
use serde_json::json;
use sqlx::SqlitePool;


use tracing;

use crate::models::mark::{self};

#[utoipa::path(
    get,
    path = "/marks",
    responses(
        (status = 200, description = "Tüm lokasyonlar alındı", body = [Mark]),
        (status = 500, description = "Lokasyonların alınmasında dahili sunucu hatası", body = [Mark])
    )
)]
pub async fn all_marks(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let sql = "SELECT id, title, time, images, latitude, longitude FROM mark ".to_string();

    let result: Result<Vec<mark::Mark>, sqlx::Error> =
        sqlx::query_as::<_, mark::Mark>(&sql).fetch_all(&pool).await;

    match result {
        Ok(marks) => {
            let marks_with_images: Vec<mark::Mark> = marks
                .into_iter()
                .map(|mark| {
                    
                    mark
                })
                .collect();

            (StatusCode::OK, Json(marks_with_images))
        }
        Err(err) => {
            println!("error retrieving marks: {:?}", err);
            tracing::error!("error retrieving marks: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<mark::Mark>::new()),
            )
        }
    }
}

#[utoipa::path(
    post,
    path = "/marks/yeni",
    request_body = NewMark,
    responses(
        (status = 201, description = "Mark başarıyla oluşturuldu", body = [Mark]),
        (status = 500, description = "Mark oluşturulamadı", body = [Mark]),
    )
)]

#[debug_handler]
pub async fn new_mark(
    Extension(pool): Extension<SqlitePool>,
    Json(mark): Json<mark::NewMark>,
) -> impl IntoResponse {
    let sql = "INSERT INTO mark (title, time, images, latitude, longitude) VALUES ($1, $2, $3, $4, $5) RETURNING *";

    let result: Result<mark::Mark, sqlx::Error> = sqlx::query_as(sql)
        .bind(&mark.title)
        .bind(&mark.time)
        .bind(serde_json::to_string(&mark.images).unwrap_or_default())
        .bind(mark.latitude)
        .bind(mark.longitude)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(mark_with_id) => (
            StatusCode::CREATED,
            [("Location", format!("/marks/{:?}", mark_with_id.id))],
            Json(mark_with_id),
        ),
        Err(err) => {
            println!("error retrieving marks: {:?}", err);
            tracing::error!("could not create mark. error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("Location", "/marks/{id}".to_string())],
                Json(mark::Mark {
                    id: 0,
                    title: String::new(),
                    time: String::new(),
                    images: String::new(),
                    latitude: 0.0,
                    longitude: 0.0,
                }),
            )
        }
    }
}

#[utoipa::path(
    get,
    path = "/marks/{id}",
    responses(
        (status = 200, description = "Mark returned successfully", body = [Mark]),
        (status = 404, description = "Mark not found", body = [Mark]),
    ),
    params(
        ("id" = i64, Path, description = "Mark database id"),
    )
)]
pub async fn mark(
    Path(id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    let sql = "SELECT * FROM mark WHERE id = $1".to_string();

    let result: Result<mark::Mark, sqlx::Error> =
        sqlx::query_as(&sql).bind(id).fetch_one(&pool).await;

    match result {
        Ok(mark) => (StatusCode::OK, Json(mark)),
        Err(err) => {
            tracing::error!("could not find mark with id: {:?} error: {:?}", id, err);
            (
                StatusCode::NOT_FOUND,
                Json(mark::Mark {
                    id,
                    title: "".to_string(),
                    time: "".to_string(),
                    images: "".to_string(),
                    latitude: 0.0,
                    longitude: 0.0,
                }),
            )
        }
    }
}


#[utoipa::path(
    delete,
    path = "/marks/{id}",
    responses(
        (status = 200, description = "Mark was deleted"),
        (status = 404, description = "Mark was not found"),
    ),
    params(
        ("id" = i64, Path, description = "Mark database id")
    ),
)]
pub async fn delete_mark(
    Path(id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM mark WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
    {
        Ok(query_result) => match query_result.rows_affected() {
            1 => (StatusCode::OK, Json(json!({"msg": "Mark Deleted"}))),
            _ => (
                StatusCode::NOT_FOUND,
                Json(json!({"msg": "Mark not found"})),
            ),
        },
        Err(e) => {
            tracing::error!("could not delete mark with id: {:?} error: {:?}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"msg": "Internal Server Error"})),
            )
        }
    }
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
struct ErrorResponseBody {
    success: bool,
    message: String,
}