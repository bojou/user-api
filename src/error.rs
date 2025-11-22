use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::Request;
use std::io::Cursor;

pub struct ExceptionHandler {
    status: Status,
    message: String,
}

impl ExceptionHandler {
    pub fn new(status: Status, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(Status::Conflict, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(Status::BadRequest, message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(Status::InternalServerError, message)
    }

    pub fn handle(error: Box<dyn std::error::Error>) -> Self {
        let error_msg = error.to_string();

        if error_msg.contains("duplicate key") || error_msg.contains("unique constraint") {
            Self::conflict("Resource already exists")
        } else if error_msg.contains("not-null constraint") || error_msg.contains("check constraint") {
            Self::bad_request(format!("Invalid data: {}", error))
        } else {
            Self::internal_error("Database error occurred")
        }
    }
}

impl<'r> Responder<'r, 'static> for ExceptionHandler {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .sized_body(self.message.len(), Cursor::new(self.message))
            .ok()
    }
}
