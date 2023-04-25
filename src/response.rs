use std::{fs, net::TcpStream, error::Error};
use std::io::prelude::*;

pub enum ContentType {
    Html,
    Javascript,
    CSS,
    Custom(String),
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        String::from(match self {
            ContentType::Html               => "text/html",
            ContentType::Javascript         => "text/javascript",
            ContentType::CSS                => "text/css",
            ContentType::Custom(s) => s
        })
    }
}

pub struct Response {
    status: Option<String>,
    content: Option<String>,
    content_type: Option<ContentType>
}

macro_rules! content_type_method {
    ( $name:tt, $content_type:expr ) => {
        pub fn $name(file_name: &str) -> Self {
            let path: String = 
                String::from( concat!("./frontend/", stringify!($name), "/") ) + 
                file_name + 
                concat!('.', stringify!($name));
    
            Response::from_file(&path)
                .with_content_type($content_type)
        }
    };
}

impl Response {
    pub fn new() -> Response {
        Response {
            content: None,
            status: None,
            content_type: None,
        }
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn from_file(file_name: &str) -> Response {
        println!("Reading from path: {}", file_name);
        let content_bytes: Vec<u8> = fs::read(file_name).unwrap();
        let content: String = String::from_utf8(content_bytes).unwrap();

        Response::new()
            .with_content(content) 
    }

    content_type_method!(html,       ContentType::Html      );
    content_type_method!(css,        ContentType::CSS       );
    content_type_method!(javascript, ContentType::Javascript);

    fn fill_default(&mut self) {
        if self.status.is_none() {
            self.status = Some( String::from("200 OK") );
        }
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let content: &String = self.content.as_ref().expect("response has content");
        let status: &String = self.status.as_ref().expect("response has status");
        let content_type: String = self.content_type.as_ref().expect("response has content_type").to_string();

        let content_len: usize = content.len();

        format!("HTTP/1.1 {}\r\nContent-Length: {content_len}\r\nContent-Type: {}\r\n\r\n{}", status, content_type, content)
            .as_bytes()
            .to_owned()
    }

    pub fn send(mut self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>>{
        self.fill_default();
        let bytes: Vec<u8> = self.to_bytes();
        stream.write(&bytes)?;
        stream.flush()?;

        Ok(())
    }
}