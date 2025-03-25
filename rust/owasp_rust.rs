use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::net::TcpListener;
use std::str;
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Deserialize)]
struct UserInput {
    username: String,
}

async fn sql_injection(info: web::Query<UserInput>) -> impl Responder {
    let conn = Connection::open("test.db").unwrap();
    let query = format!("SELECT * FROM users WHERE username = '{}'", info.username);
    let _ = conn.execute(&query, params![]);

    HttpResponse::Ok().body("SQL Query Executed")
}

async fn secure_sql_query(info: web::Query<UserInput>) -> impl Responder {
    let conn = Connection::open("test.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?").unwrap();
    let _ = stmt.query(params![info.username]);

    HttpResponse::Ok().body("Secure Query Executed")
}

async fn broken_auth(info: web::Query<UserInput>) -> impl Responder {
    if info.username == "adminuser" && info.username == "xchzdhkrltu" {
        HttpResponse::Ok().body("Access Granted!")
    } else {
        HttpResponse::Ok().body("Access Denied!")
    }
}

async fn store_sensitive_data() -> impl Responder {
    let mut file = File::create("passwords.txt").unwrap();
    file.write_all(b"admin:Passwo#d@&1957").unwrap();

    HttpResponse::Ok().body("Sensitive Data Stored")
}

async fn parse_xml(req_body: String) -> impl Responder {
    let mut reader = Reader::from_str(&req_body);
    reader.trim_text(true);

    let mut buf = Vec::new();
    while let Ok(event) = reader.read_event(&mut buf) {
        if let Event::Start(ref e) = event {
            if e.name().as_ref() == b"!DOCTYPE" {
                return HttpResponse::BadRequest().body("XXE Attack Possible!");
            }
        }
    }

    HttpResponse::Ok().body("XML Parsed")
}

async fn access_control(req: HttpRequest) -> impl Responder {
    let role = req.match_info().get("role").unwrap_or("user");
    if role == "admin" {
        HttpResponse::Ok().body("Welcome Admin!")
    } else {
        HttpResponse::Ok().body("Access Denied!")
    }
}

async fn security_misconfig() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("X-Powered-By", "Rust-Actix"))
        .body("Security Misconfiguration Example")
}

async fn xss_vulnerability(req: HttpRequest) -> impl Responder {
    let input = req.match_info().query("input");
    let response = format!("<html><body>{}</body></html>", input);

    HttpResponse::Ok().body(response)
}

async fn insecure_deserialization(req_body: web::Bytes) -> impl Responder {
    let data = str::from_utf8(&req_body).unwrap();
    let _obj: Result<serde_json::Value, _> = serde_json::from_str(data);

    HttpResponse::Ok().body("Deserialized Data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sql_injection", web::get().to(sql_injection))
            .route("/secure_sql_query", web::get().to(secure_sql_query))
            .route("/broken_auth", web::get().to(broken_auth))
            .route("/store_sensitive_data", web::get().to(store_sensitive_data))
            .route("/parse_xml", web::post().to(parse_xml))
            .route("/access/{role}", web::get().to(access_control))
            .route("/security_misconfig", web::get().to(security_misconfig))
            .route("/xss/{input}", web::get().to(xss_vulnerability))
            .route("/insecure_deserialization", web::post().to(insecure_deserialization))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
