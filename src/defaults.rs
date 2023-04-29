use std::error::Error;

use crate::response::{Response, ContentType};
use crate::app::{ErrorView, SendableError};

fn default_view_400(error: Box<dyn Error + 'static>) -> Response {
    let status: String = String::from("400 BAD REQUEST");

    Response::new()
        .with_content(format!("Error: {:?}", error))
        .with_content_type(ContentType::Custom(String::from("text/plain")))
        .with_status(status)
}

pub fn default_route_400 (_error: &SendableError) -> ErrorView {
    Box::new(default_view_400)
}