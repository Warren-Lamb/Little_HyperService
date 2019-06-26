use std::fmt;
use std::sync::{Mutex,Arc};
use hyper::Server;
use hyper::{Body,Error,Method,Request,Response,Serve,StatusCode};
use hyper::service::Service_fn;
use lazy_static::lazy_static;
use slab::Slab;
use futures::{future,Future};
use regex::Regex;

const INDEX: &str = r#"
<!doctype html>
<html>
  <head>
	<title> "Little HyperService"</title>
  </head>
  <body>
    <h2> "Rust microservice</h2>
  </body>
</html>
"#;



