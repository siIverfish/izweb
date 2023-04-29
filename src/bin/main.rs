use tictactoe::request::{Request, MethodRef};
use tictactoe::app::{View, App};
use tictactoe::response::Response;


fn index(_request: Request) -> Response {
    Response::html("index")
}

fn error_404(_request: Request) -> Response {
    Response::html("404")
}

fn route(request: &Request) -> View {
    match request.method.as_ref() {
        MethodRef::Get("/") => Box::new(index),
        _                   => Box::new(error_404),
    }
}

fn main() {
    App::build()
        .with_route_fn(route)
        .run()
}