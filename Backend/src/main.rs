mod dice;
mod game;
mod hasher;
mod keno;

use actix_web::{get, HttpResponse, HttpServer, Responder};
use rand::RngCore;
use crate::dice::Dice;
use crate::game::Game;
use crate::keno::Keno;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        actix_web::App::new()
            .service(dice_game)
            .service(keno_game)
    }).bind(("127.0.0.1", 8080))?.run().await

}

#[get("/dice")]
async fn dice_game() -> impl Responder {
    let dice = Dice::new();
    let mut rng = rand::thread_rng();
    let mut input = [0u8; 16];
    rng.fill_bytes(&mut input[..]);
    let roll = dice.roll(input);

    HttpResponse::Ok().body(format!("{:.2}", roll))
}

#[get("/keno")]
async fn keno_game() -> impl Responder {
    let keno = Keno::new();

    let mut rng = rand::thread_rng();
    let mut input = [0u8; 16];
    rng.fill_bytes(&mut input[..]);

    HttpResponse::Ok().body(format!("{:?}", keno.shuff(input).split_at(10).0))
}