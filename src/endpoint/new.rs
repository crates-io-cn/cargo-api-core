use actix_web::{put, web, HttpResponse, Responder};
use byteorder::{LittleEndian, ReadBytesExt};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use web::Buf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dep {
    name: String,
    vresion_req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Option<String>,
    register: Option<String>,
    explicit_name_in_toml: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Features {
    extras: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    name: String,
    vers: String,
    deps: Vec<Dep>,
    // features: Features,
    authors: Vec<String>,
    description: Option<String>,
    documentation: Option<String>,
    homepage: Option<String>,
    readme: Option<String>,
    readme_file: Option<String>,
    keywords: Vec<String>,
    categories: Vec<String>,
    license: Option<String>,
    license_file: Option<String>,
    repository: Option<String>,
    links: Option<String>,
    badges: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Warnings {
    invalid_categories: Vec<String>,
    invalid_badges: Vec<String>,
    other: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    warnings: Warnings
}

impl Default for Response {
    fn default() -> Self {
        Response {
            warnings: Warnings {
                invalid_categories: Vec::new(),
                invalid_badges: Vec::new(),
                other: Vec::new()
            }
        }
    }
}

#[put("/api/v1/crates/new")]
// pub async fn publish(req: web::Json<Request>) -> impl Responder {
pub async fn publish(mut body: web::Payload) -> impl Responder {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }

    let mut data = bytes.bytes();

    let length: usize = data.read_u32::<LittleEndian>().unwrap().try_into().unwrap();

    let json: Request = serde_json::from_slice(&data[..length]).unwrap();

    println!("Json {:?}!", json);
    HttpResponse::Ok().json(Response::default())
}
