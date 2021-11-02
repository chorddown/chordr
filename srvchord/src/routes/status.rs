use rocket::get;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![crate::routes::status::index,]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    version: &'static str,
    running: bool,
}

#[get("/")]
pub fn index() -> Json<Status> {
    Json(Status {
        version: env!("CARGO_PKG_VERSION"),
        running: true,
    })
}

#[cfg(test)]
mod test {
    use crate::test_helpers::{json_format, run_test_fn, JsonTemplateValue};
    use rocket::http::Status;

    #[test]
    fn test_index() {
        run_test_fn(|client, _conn| {
            let get_response = client.get("/status/").dispatch();
            assert_eq!(get_response.status(), Status::Ok);

            let response_body = get_response.into_string().unwrap();

            assert_eq!(
                response_body,
                json_format::<JsonTemplateValue>(
                    r#"{"version":"$","running":true}"#,
                    vec![env!("CARGO_PKG_VERSION").into(),],
                ),
            );
        })
    }
}
