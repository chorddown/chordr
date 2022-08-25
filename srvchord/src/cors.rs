use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct Cors {
    allowed_origins: Vec<&'static str>,
}

impl Cors {
    pub fn new(allowed_origins: Vec<&'static str>) -> Self {
        Self { allowed_origins }
    }
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        match request.headers().get_one("origin") {
            Some(origin_header) if self.allowed_origins.contains(&origin_header) => {
                response.set_header(Header::new("Access-Control-Allow-Origin", origin_header));
                response.set_header(Header::new(
                    "Access-Control-Allow-Methods",
                    "GET, POST, OPTIONS",
                ));
                response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
                response.set_header(Header::new(
                    "Access-Control-Allow-Headers",
                    "Authorization, Accept, Content-Type",
                ));
            }
            Some(_) => {}
            None => {}
        }
    }
}
