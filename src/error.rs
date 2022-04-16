use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Error(ErrorInternals);

#[derive(Debug)]
enum ErrorInternals {
    HttpError(actix_web::Error),
    Custom {
        status_code: StatusCode,
        message: String,
    },
    Other {
        status_code: StatusCode,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.0 {
            ErrorInternals::HttpError(source) => write!(f, "woof::Error: {}", source),
            ErrorInternals::Other { source, .. } => {
                write!(f, "woof::Error: {}", source)
            }
            ErrorInternals::Custom { message, .. } => {
                write!(f, "woof::Error: {}", message)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.0 {
            ErrorInternals::HttpError(source) => Some(source),
            ErrorInternals::Other { source, .. } => Some(source.as_ref()),
            ErrorInternals::Custom { .. } => None,
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match &self.0 {
            ErrorInternals::HttpError(error) => error.error_response(),
            ErrorInternals::Other {
                status_code,
                source,
            } => HttpResponse::build(*status_code).body(source.to_string()),
            ErrorInternals::Custom {
                status_code,
                message,
            } => HttpResponse::build(*status_code).body(message.clone()),
        }
    }
}

impl From<actix_web::Error> for Error {
    fn from(error: actix_web::Error) -> Self {
        Self(ErrorInternals::HttpError(error))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self(ErrorInternals::Other {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            source: Box::new(error),
        })
    }
}

impl From<sea_orm::error::DbErr> for Error {
    fn from(error: sea_orm::error::DbErr) -> Self {
        Self(ErrorInternals::Other {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            source: Box::new(error),
        })
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[allow(non_snake_case)]
pub fn MissingPathSegment(segment: impl Display) -> Error {
    Error(ErrorInternals::Custom {
        status_code: StatusCode::NOT_FOUND,
        message: format!("Missing path segment {segment}"),
    })
}

#[allow(non_snake_case)]
pub fn InvalidPathSegment(segment: impl Display) -> Error {
    Error(ErrorInternals::Custom {
        status_code: StatusCode::BAD_REQUEST,
        message: format!("Invalid path segment {segment}"),
    })
}
