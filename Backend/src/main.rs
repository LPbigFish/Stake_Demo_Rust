mod dice;
mod game;
mod hasher;
mod keno;

use std::vec;

use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, HttpServer, Responder, web, post, HttpRequest};
use actix_cors::Cors;
use serde::Deserialize;
use crate::dice::Dice;
use crate::game::Game;
use crate::keno::Keno;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data = web::Data::new(AppState::new());

    HttpServer::new( move || {
        actix_web::App::new()
        .wrap(
            Cors::default()
                .allow_any_origin()
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST"])
                .max_age(3600)
        )
        .app_data(data.clone())
            .service(dice_game)
            .service(keno_game)
    }).bind(("127.0.0.1", 8080))?.run().await

}

#[get("/dice")]
async fn dice_game(input_params: web::Json<InputParameters>, data: web::Data<AppState>) -> impl Responder {
    let dice = data.dice;

    let mut input = [0u8; 16];
    input.copy_from_slice(input_params.uuid.as_bytes());

    HttpResponse::Ok().content_type(ContentType::json()).json(serde_json::json!({
        "dice": format!("{:.2}", dice.roll(input))
    }))
}

#[post("/keno")]
async fn keno_game(input_params: web::Json<InputParameters>, data: web::Data<AppState>) -> impl Responder {
    let keno = data.keno;

    let mut input = [0u8; 16];
    input.copy_from_slice(&hasher::new_hash_from_bytes(input_params.uuid.as_bytes()));

    HttpResponse::Ok().content_type(ContentType::json()).json(serde_json::json!({
        "keno": keno.shuff(input).split_at(10).0
    }))
}

#[derive(Deserialize)]
struct InputParameters {
    uuid: String
}

struct AppState {
    keno: Keno,
    dice: Dice
}

impl AppState {
    fn new() -> Self {
        AppState { keno: Keno::new(), dice: Dice::new() }
    }

    fn new_with_params(keno_hash: [u8; 16], dice_hash: [u8; 16]) -> Self {
        AppState {
            keno: Keno::new_with_params(keno_hash),
            dice: Dice::new_with_params(dice_hash)
        }
    }
}