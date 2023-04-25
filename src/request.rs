use std::error::Error;


#[derive(Debug)]
pub enum Method<'a> {
    Get(&'a str),
    Post(&'a str),
}

impl Method<'_> {
    fn from_strings<'a>(method: &str, path: &'a str) -> Result<Method<'a>, Box<dyn Error>> {
        match method {
            "GET"  => Ok(Method::Get (path.into())),
            "POST" => Ok(Method::Post(path.into())),
            _      => Err("did not match")?,
        }
    }
}

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method<'a>,
    pub protocol: String,
    pub headers: Vec<(String, String)>
}

impl Request<'_>  {
    pub fn from_string(string: &str) -> Result<Request, Box<dyn Error>> {
        let mut lines = string.split("\r\n");

        let mut first_line = lines.next().ok_or("failed to get first line")?.split(" ");

        let method   = first_line.next().ok_or("failed to get method"  )?;
        let path     = first_line.next().ok_or("failed to get path"    )?;
        let protocol = first_line.next().ok_or("failed to get protocol")?;
        
        println!("method: {} | path: {} | protocol: {}", method, path, protocol);

        let method = Method::from_strings(method, path)?;
        let protocol = String::from(protocol);

        let mut headers: Vec<(String, String)> = Vec::new();


        while let Some(line) = lines.next() {
            if line == "" {
                break;
            }
            
            println!("line: {}", line);

            if let &[key, value] = &line.split(": ").collect::<Vec<&str>>()[..] {
                headers.push((key.into(), value.into()));
            } else {
                Err(format!("failed to parse previous line: '{}'", line))?
            }
        }

        Ok(Request { method, protocol, headers })
    }
}