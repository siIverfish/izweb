use std::error::Error;

type SendableError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum Method {
    Get(String),
    Post(String),
}

impl Method {
    fn from_strings<'a>(method: &str, path: &'a str) -> Result<Method, SendableError> {
        match method {
            "GET"  => Ok(Method::Get (path.into())),
            "POST" => Ok(Method::Post(path.into())),
            _      => Err("did not match")?,
        }
    }

    pub fn as_ref<'a>(&'a self) -> MethodRef<'a> {
        match self {
            Method::Get(s) => MethodRef::Get(s),
            Method::Post(s) => MethodRef::Post(s),
        }
    }
}

pub enum MethodRef<'a> {
    Get(&'a str),
    Post(&'a str)
}


/// The object used to store data about incoming requests to the server.
/// 
/// The main loop of the server:
/// 
/// 1. Server gets request bytes
/// 2. Makes bytes into Request object
/// 3. Routing
/// 4. View code returns Response object
/// 5. Send response object
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub protocol: String,
    pub headers: Vec<(String, String)>
}

impl Request  {
    /// Constructs a Request object from a web request represented as an `&str`.
    /// e.g.
    /// 
    /// `request.txt`:
    /// GET /favicon.ico HTTP/1.1
    /// Host: 127.0.0.1:7878
    /// Connection: keep-alive
    /// sec-ch-ua: "Chromium";v="112", "Google Chrome";v="112", "Not:A-Brand";v="99"
    /// sec-ch-ua-mobile: ?0
    /// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36
    /// sec-ch-ua-platform: "Windows"
    /// Accept: image/avif,image/webp,image/apng,image/svg+xml,image/*,*/*;q=0.8
    /// Sec-Fetch-Site: same-origin
    /// Sec-Fetch-Mode: no-cors
    /// Sec-Fetch-Dest: image
    /// Referer: http://127.0.0.1:7878/
    /// Accept-Encoding: gzip, deflate, br
    /// Accept-Language: en-US,en;q=0.9
    /// 
    /// `main.rs`:
    /// (idk if this works)
    /// ```
    /// use std::fs;
    /// use tictactoe::request::Request;
    /// 
    /// let request_bytes: Vec<u8> = fs::read(file_name).unwrap();
    /// let request: &str = &String::from_utf8(content_bytes).unwrap();
    /// 
    /// let request = Request::from_string(request);
    /// 
    /// assert_eq!(request.method, Method::Get("/favicon.ico"));
    /// ```
    pub fn from_string<'a>(string: &'a str) -> Result<Request, SendableError> {
        let mut lines = string.lines();

        let mut first_line = lines.next().ok_or("failed to get first line")?.split(" ");

        let method   = first_line.next().ok_or("failed to get method"  )?;
        let path     = first_line.next().ok_or("failed to get path"    )?;
        let protocol = first_line.next().ok_or("failed to get protocol")?;
        
        // println!("method: {} | path: {} | protocol: {}", method, path, protocol);

        let method = Method::from_strings(method, path)?;
        let protocol = String::from(protocol);

        let mut headers: Vec<(String, String)> = Vec::new();


        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }

            // println!("line: {}", line);

            if let &[key, value] = &line.split(": ").collect::<Vec<&str>>()[..] {
                headers.push((key.into(), value.into()));
            } else {
                Err(format!("failed to parse previous line: '{}'", line))?
            }
        }

        Ok( Request { method, protocol, headers } )
    }
}