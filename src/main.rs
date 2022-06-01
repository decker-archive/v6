use actix_web::{get, web, App, HttpServer, Responder, Result};
use nanoid::nanoid;
mod utils;
mod responses;


#[get("/auth/fingerprint")]
async fn fingerprint() -> Result<impl Responder> {
    let snowflake = utils::gen_sf();
    let d = responses::Fingerprint {
        fingerprint: format!("{}.{}", snowflake.to_string(), nanoid!())
    };

    Ok(web::Json(d))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fingerprint)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
