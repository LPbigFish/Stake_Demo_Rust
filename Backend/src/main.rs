mod dice;
mod game;
mod hasher;
mod keno;

use std::collections::HashMap;
use std::sync::Mutex;
use std::vec;

use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse, HttpServer, Responder, web, post, HttpRequest};
use actix_cors::Cors;
use hasher::*;
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
                .allowed_methods(vec!["GET", "POST"])
                .max_age(3600)
        )
        .app_data(data.clone())
            .service(dice_game)
            .service(keno_game)
            .service(index)
    }).bind(("127.0.0.1", 8080))?.run().await

}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
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

#[get("/keno")]
async fn keno_game(input_params: web::Json<InputParameters>, data: web::Data<AppState>) -> impl Responder {
    let time = std::time::Instant::now();
    let keno = data.keno;

    let input = data.handle_user_hash(input_params.uuid.clone());

    
    println!("Time: {:?}", time.elapsed());
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
    dice: Dice,
    user_hash: Mutex<HashMap<String, [u8;16]>>
}

impl AppState {
    fn new() -> Self {
        AppState { keno: Keno::new(), dice: Dice::new(), user_hash: Mutex::new(HashMap::new()) }
    }

    fn new_with_params(keno_hash: [u8; 16], dice_hash: [u8; 16], user_hash: HashMap<String, [u8; 16]>) -> Self {
        AppState {
            keno: Keno::new_with_params(keno_hash),
            dice: Dice::new_with_params(dice_hash),
            user_hash: Mutex::new(user_hash)
        }
    }

    fn handle_user_hash(&self, uuid: String) -> [u8;16] {
        let mut the_map = self.user_hash.lock().unwrap();

        if the_map.contains_key(&uuid) {
            let hash = new_hash_from_bytes(&(the_map[&uuid].as_ref())).clone();
            (*the_map).insert(uuid, hash).unwrap()
        } else {
            let hash = new_hash_from_bytes(uuid.as_bytes());
            (*the_map).insert(uuid, new_hash_from_bytes(hash.as_ref()).clone());
            hash
        }
    }
}