use std::{env, fs};

use serde::Serialize;
use warp::multipart::FormData;
use bytes::BufMut;
use futures::{TryStreamExt, StreamExt};
use uuid::Uuid;

use crate::{db::DBConnection, error::{EmptyContentError, InternalServerError, InvalidFileError, InvalidSessionError}, validate_session_from_headers};

#[derive(Debug, Clone, Copy, Serialize)]
enum FileType {
    Image,
    Video,
    Audio
}

impl FileType {
    fn try_from(content_type: &str) -> Option<Self> {
        match content_type.split("/").next()? {
            "image" => Some(Self::Image),
            "video" => Some(Self::Video),
            "audio" => Some(Self::Audio),
            _ => None
        }
    }
}

struct PendingFile {
    content: Vec<u8>,
    file_type: FileType,
}

impl PendingFile {
    fn save(self) -> FileID {
        let file_id = Uuid::new_v4().to_string();

        fs::write(
            format!("{}/appendage/file/{}", env::var("STORAGE_URL").unwrap(), file_id),
            self.content
        ).unwrap();

        FileID { file_id, file_type: self.file_type }
    }
}

#[derive(Debug, Serialize)]
struct FileID {
    file_id: String,
    file_type: FileType,
}

#[derive(Debug, Serialize)]
struct Appendage {
    appendage_id: String,
    files: Vec<FileID>,
}

#[derive(Debug, Serialize)]
struct FileUploadResponse {
    appendage_id: String,
}

pub async fn file_upload(headers: warp::http::HeaderMap, connection: DBConnection, form: FormData) -> Result<impl warp::Reply, warp::Rejection> {

    let _user = validate_session_from_headers(&headers, connection).await.ok_or(InvalidSessionError)?;

    let mut parts = form.into_stream();

    let mut files = Vec::new();

    while let Some(Ok(part)) = parts.next().await {
        let file_type = FileType::try_from(
            part.content_type().ok_or(InvalidFileError)?
        ).ok_or(InvalidFileError)?;

        let content = part.stream()
            .try_fold(Vec::new(), |mut v, buffer| {
                v.put(buffer);
                async move { Ok(v) }
            })
            .await
            .map_err(|_| InvalidFileError)?;

        files.push(PendingFile { content, file_type });
    }

    let appendage = Appendage {
        appendage_id: Uuid::new_v4().to_string(),
        files: files.into_iter().map(PendingFile::save).collect()
    };

    let _ = fs::write(
        format!("{}/appendage/{}", env::var("STORAGE_URL").unwrap(), appendage.appendage_id),
        serde_json::to_string(&appendage).unwrap()
    );

    Ok(warp::reply::json(&FileUploadResponse { appendage_id: appendage.appendage_id }))
}

pub async fn update_profile_picture(headers: warp::http::HeaderMap, connection: DBConnection, form: FormData) -> Result<impl warp::Reply, warp::Rejection> {
    let user = validate_session_from_headers(&headers, connection.clone()).await.ok_or(InvalidSessionError)?;

    let mut parts = form.into_stream();

    let file = parts.next().await.ok_or(EmptyContentError)?.map_err(|_| InternalServerError)?;

    let content = file.stream()
        .try_fold(Vec::new(), |mut v, buffer| {
            v.put(buffer);
            async move { Ok(v) }
        })
        .await
        .map_err(|_| InvalidFileError)?;

    fs::write(
        format!("{}/profile_picture/{}", env::var("STORAGE_URL").unwrap(), user.get_username()),
        content
    ).map_err(|_| InternalServerError)?;

    Ok(warp::reply())
}