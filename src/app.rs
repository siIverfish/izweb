use std::{error::Error, net::{TcpListener, TcpStream}, io::Read};

use crate::{request::Request, response::Response, threads::ThreadPool};
use crate::defaults::default_route_400;

// this is copied from request, todo types module or something
pub type SendableError = Box<dyn Error + Send + Sync + 'static>;

pub type View      = Box<dyn Fn(Request)        -> Response + Send + Sync + 'static>;
pub type ErrorView = Box<dyn Fn(Box<dyn Error>) -> Response + Send + Sync + 'static>;

pub type RouteFn      = Box<dyn Fn(&Request)        -> View>;
pub type ErrorRouteFn = Box<dyn Fn(&SendableError) -> ErrorView>;

pub struct AppBuilder {
    route_fn: Option<RouteFn>,
    route_400_fn: Option<ErrorRouteFn>,
    bind: Option<&'static str>,
    threadpool_size: Option<usize>,
}

pub struct App {
    route_fn: RouteFn,
    route_400_fn: ErrorRouteFn,
    bind: &'static str,
    threadpool_size: usize,
}

impl App {
    pub fn build() -> AppBuilder {
        AppBuilder { 
            route_fn: None, 
            route_400_fn: None, 
            bind: None, 
            threadpool_size: None
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let listener: TcpListener = 
            TcpListener::bind(self.bind)?;
    
        let pool: ThreadPool = ThreadPool::new(self.threadpool_size);
        
        for stream in listener.incoming() {
            let mut stream: TcpStream = stream?;

                
            let mut buffer  = [0; 1024];
            stream.read(&mut buffer)?;
            let request_string = String::from_utf8(buffer.to_vec())?;

            let request: Result<Request, Box<(dyn Error + Send + Sync + 'static)>> = Request::from_string(&request_string);

            match request {
                Ok(request)       => {
                    let view_fn: View = (self.route_fn)(&request);
                    pool.execute(move || {
                        view_fn(request).send(&mut stream).unwrap();
                    });
                },

                Err(error) => {
                    let view_fn: ErrorView = (self.route_400_fn)(&error);
                    pool.execute(move || {
                        view_fn(error).send(&mut stream).unwrap();
                    });
                },  
            };

        }
    
        Ok(())
    }

}

impl AppBuilder {
    pub fn with_route_fn<F>(&mut self, route_fn: F) -> &mut Self
        where
            F: Fn(&Request) -> View + 'static
    {
        self.route_fn = Some(Box::new(route_fn));
        self
    }

    pub fn bound_to(&mut self, bind: &'static str) -> &mut Self {
        self.bind = Some(bind);
        self
    }

    // fn with_threadpool_size(&mut self, )

    pub fn fill_defaults(&mut self) -> &mut Self {
        // define defaults
        let default: AppBuilder = AppBuilder {
            route_fn: None,
            route_400_fn: Some(Box::new(default_route_400)),
            bind: Some("127.0.0.1:7878"),
            threadpool_size: Some(4),
        };

        // fill defaults, TODO replace with macro or something
        // there must be a better way to do this
        if let Some(default_route_fn) = default.route_fn { self.route_fn = Some(default_route_fn); }
        if let Some(default_route_400_fn) = default.route_400_fn { self.route_400_fn = Some(default_route_400_fn); }
        if let Some(default_bind) = default.bind { self.bind = Some(default_bind); }
        if let Some(default_threadpool_size) = default.threadpool_size { self.threadpool_size = Some(default_threadpool_size); }

        self
    }

    pub fn build(&mut self) -> App {
        self.fill_defaults();

        App {
            route_fn:        self.route_fn       .take().expect("App contains route_fn"),
            route_400_fn:    self.route_400_fn   .take().expect("App contains route_404_fn"),
            bind:            self.bind           .take().expect("App contains bind address with port"),
            threadpool_size: self.threadpool_size.take().expect("App contains threadpool size"),
        }
    }

    pub fn run(&mut self) {
        self.build().run().unwrap()
    }
}