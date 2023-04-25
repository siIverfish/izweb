use std::net::{TcpListener, TcpStream};

use std::error::Error;

use std::io::prelude::*;


use tictactoe::request::{Request, Method};
use tictactoe::response::{Response, ContentType};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener: TcpListener = 
        TcpListener::bind("127.0.0.1:7878")?;
    
    for stream in listener.incoming() {
        let mut stream: TcpStream = stream?;
        
        handle_connection(&mut stream)?;
    }

    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer)?;

    let buffer = String::from_utf8(buffer.to_vec())?;

    let request = Request::from_string(&buffer);

    let response: Response = match request {
        Ok(request)       => route_request(request),
        Err(error) => route_404    (error),
    };

    response.send(stream)?;

    Ok(())
}

fn route_404 (error: Box<dyn Error>) -> Response {
    let status: String = String::from("400 BAD REQUEST");

    Response::new()
        .with_content(format!("{:?}", error))
        .with_content_type(ContentType::Custom(String::from("text/plain")))
        .with_status(status)
}

fn route_request(request: Request) -> Response {
    match request.method {
        Method::Get("/")        => Response::html("index"),
        Method::Get("/about")   => Response::html("about"),
        _                       => Response::html("404"),
    }
}