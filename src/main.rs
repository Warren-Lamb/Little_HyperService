use std::fmt;
use std::sync::{Mutex,Arc};
use hyper::{Body,Error,Method,Request,Response,Server,StatusCode};
use hyper::service::service_fn;
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


type UserId = u64;
struct UserData;
type UserDb = Arc<Mutex<Slab<UserData>>>;

impl fmt::Display for UserData {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("{}")
	}
}

lazy_static! {
	static ref INDEX_PATH : Regex = Regex::new("^/(index\\.html?)?$").unwrap();
	static ref USER_PATH: Regex = Regex::new("^/user/((?P<user_id>\\d+?/?)?$").unwrap();
	static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
}

fn microservice_handler (req: Request<Body>, db: &UserDb) 
   -> impl Future<Item = Response<Body>,Error = Error> 
   {
   	let r = Response::new(INDEX.into());
   future::ok(r)	
   }

fn main() {
	let address = ([127,0,0,1],6000).into();
	let builder = Server::bind(&address);
	let db = Arc::new(Mutex::new(Slab::new()));
	let server = builder.serve(move || {
		let db = db.clone();
		service_fn(move |req| microservice_handler(req , &db))
		});
	let server = server.map_err(drop);
	hyper::rt::run(server);

	}

