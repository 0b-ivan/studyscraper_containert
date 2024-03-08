#[macro_use]
extern crate rocket;

use reqwest::header::{HeaderMap, HeaderValue};
use rocket::fs::FileServer;
use rocket::http::ContentType;
use rocket::response::Redirect;
use rocket::response::{self, Responder, Response};
use rocket::Request;
use rocket_dyn_templates::{context, Template};

enum MyResponse {
    Redirect(Redirect),
    Template(Template),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for MyResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            MyResponse::Redirect(redirect) => redirect.respond_to(req),
            MyResponse::Template(template) => {
                Response::build_from(template.respond_to(req).unwrap())
                    .header(ContentType::HTML)
                    .ok()
            }
        }
    }
}

#[get("/download/<url>")]
async fn download(url: &str) -> MyResponse {
    if !url.starts_with("https://www.studydrive.net/") {
        return MyResponse::Redirect(Redirect::to("/not_found"));
    }

    let doc_id = url
        .split('/')
        .last()
        .unwrap()
        .split('?')
        .collect::<Vec<_>>()[0];

    println!("doc_id={}", doc_id);

    let json = send_get_request(doc_id).await.unwrap();
    let data = json["data"].as_object().unwrap();
    if data["file_preview"].is_null() {
        println!("no token found");
        let json = send_get_request(doc_id).await.unwrap();
        let token = get_token().await.unwrap();
        let data = json["data"].as_object().unwrap();
        let name = data["filename"].as_str().unwrap();
        let ending = name.split('.').last().unwrap();
        let url = format!(
            "https://cdn.studydrive.net/d/prod/documents/{}/original/{}.{}?token={}",
            doc_id, doc_id, ending, token
        );
        return MyResponse::Template(Template::render("download", context! { url: url }));
    }
    let name = data["filename"].as_str().unwrap();
    let ending = name.split('.').last().unwrap();
    let preview = data["file_preview"].as_str().unwrap();
    let token = preview.split("token=").last().unwrap();

    let url = format!(
        "https://cdn.studydrive.net/d/prod/documents/{}/original/{}.{}?token={}",
        doc_id, doc_id, ending, token
    );
    return MyResponse::Template(Template::render("download", context! { url: url }));
}

async fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let doc_id = "1617040";
    let json = send_get_request(doc_id).await?;

    let data = json["data"].as_object().unwrap();
    let preview = data["file_preview"].as_str().unwrap();
    let token = preview.split("token=").last().unwrap();
    Ok(token.to_string())
}

async fn send_get_request(doc_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("https://www.studydrive.net/document/{}", doc_id);
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-Requested-With",
        HeaderValue::from_static("XMLHttpRequest"),
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .expect("Failed to send request");

    // handle the response as json
    let json = response
        .json::<serde_json::Value>()
        .await
        .expect("Failed to parse json");

    Ok(json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .configure(
            rocket::Config::figment()
                .merge(("address", "0.0.0.0"))
                .merge(("port", 8080)),
        )
        .mount("/", routes![download])
        .mount("/", FileServer::from("static"))
}
