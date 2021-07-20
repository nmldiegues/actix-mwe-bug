use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{self, Deserialize, Serialize};

use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::StatusCode;
use std::fmt;
use std::fmt::{Display, Formatter};

fn main() {
    let sys = actix_web::rt::System::new("mwe");
    HttpServer::new(move || {
        App::new()
            .app_data(web::PathConfig::default().error_handler(error_parsing))
            .configure(configure)
    })
        .keep_alive(5)
        .bind("0.0.0.0:37123")
        .expect("couldn't start userfacing HTTP server")
        .run();

    sys.run().expect("actix runtime terminated");
}

pub fn error_parsing<T: std::error::Error + 'static>(_err: T, _req: &HttpRequest) -> actix_web::Error {
    ResponseErr {}.into()
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResponseErr {}

impl Display for ResponseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("responseerr")
    }
}

impl actix_web::ResponseError for ResponseErr {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let resp_json = serde_json::to_string(self).unwrap();
        HttpResponse::build(StatusCode::BAD_REQUEST)
            .header(CONTENT_TYPE, "application/json")
            .body(resp_json)
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mypath/{unused1}/{unused2}").service(web::resource("").route(web::post().to(my_endpoint))),
    );
}

#[derive(Serialize, Deserialize)]
pub enum MyEnum {
    #[serde(rename = "my_enum_value")]
    MyEnumValue,
}

async fn my_endpoint(
    web::Path((_unused1, _unused2)): web::Path<(i64, MyEnum)>,
    _body: web::Json<String>,
) -> web::Json<String> {
    web::Json("response_ok".to_owned())
}
