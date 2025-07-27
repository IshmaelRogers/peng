use actix_files::Files;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct CourseMeta {
    id: String,
    title: String,
}

fn load_courses() -> Vec<CourseMeta> {
    let mut list = Vec::new();
    if let Ok(entries) = fs::read_dir("courses") {
        for e in entries.flatten() {
            let path = e.path().join("course.toml");
            if let Ok(data) = fs::read_to_string(path) {
                if let Ok(meta) = toml::from_str::<CourseMeta>(&data) {
                    list.push(meta);
                }
            }
        }
    }
    list
}

#[get("/api/courses")]
async fn courses() -> impl Responder {
    let list = load_courses();
    HttpResponse::Ok().json(list)
}

#[derive(Deserialize)]
struct LaunchReq {
    course: String,
    learner: String,
}

#[post("/api/launch")]
async fn launch(req: web::Json<LaunchReq>) -> actix_web::Result<impl Responder> {
    let orch = std::env::var("ORCH_URL").unwrap_or_else(|_| "http://localhost:8080/launch".into());
    let client = reqwest::Client::new();
    let url = format!("{}?learner={}", orch, req.learner);
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?
        .text()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("application/json").body(resp))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(courses)
            .service(launch)
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
