use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub status: u16,
    pub message: String,
    // code: i32,
}

impl<'r> Responder<'r, 'static> for ErrorBody {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self);

        match body {
            Ok(value) => Response::build()
                .status(Status::new(self.status))
                .header(ContentType::JSON)
                .sized_body(value.len(), Cursor::new(value))
                .ok(),
            Err(err) => {
                println!("Unable to parse ErrorBody: {:#?}", err);
                Response::build().status(Status::InternalServerError).ok()
            }
        }
    }
}
