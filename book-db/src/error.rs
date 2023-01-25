use mongodb::bson;
use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
  #[error("mongodb error:{0}")]
  MongoError(#[from] mongodb::error::Error),
  #[error("error during mongodb query: {0}")]
  MongoQueryError(mongodb::error::Error),
  #[error("could not access field in document: {0}")]
  MongoDataError(#[from] bson::document::ValueAccessError),
  #[error("invalid id used: {0}")]
  InvalidError(String)
}

#[derive(Serialize)]
struct ErrorResponse {
  message: String
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
  let code: StatusCode;
  let message: &str;

  if err.is_not_found() {
    code = StatusCode::NOT_FOUND;
    message = "Not found";
  } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
    code = StatusCode::BAD_REQUEST;
    message = "Invalid body";
  } else if let Some(e: &Error) = err.find::<Error>() {
    match e {
      _ => {
        eprintln!("Application error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal server error";
      }
    }
  } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
    code = StatusCode::METHOD_NOT_ALLOWED;
    message = "Mothod not allowed";
  } else {
    eprintln!("unhandled error: {:?}", err);
    code = StatusCode::INTERNAL_SERVER_ERROR;
    message = "Internal server error";
  }
  let json: json = reply::json(val: &ErrorResponse {
    message: message.into()
  });
  Ok(Box::new(reply::with_status(json, code)))
}