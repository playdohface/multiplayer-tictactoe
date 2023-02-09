#![allow(unused)]

use actix_web::{
    get, http::header, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use gamemanager::GameManager;
use nanoid::nanoid;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub mod gamemanager;
pub mod tictactoe;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let gm = GameManager::init();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&gm)))
            .service(index)
            .service(newgame)
            .service(game)
            .service(addmove)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4)
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello"
}

#[get("/newgame")]
async fn newgame(games: web::Data<GameManager>) -> impl Responder {
    let gameid = nanoid!(8);
    match games.newgame(gameid.clone()) {
        Ok(_) => HttpResponse::Found()
            .append_header((header::LOCATION, gameid))
            .finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{game_id}")]
async fn game(
    id: web::Path<String>,
    gm: web::Data<GameManager>,
) -> impl Responder {
    match gm.getgame(id.into_inner()) {
        Some(g) => {
            g.show()
            
        },
        None => "Game not found!".to_string()
    }
}

#[post("/{game_id}/{move}")]
async fn addmove(
    path: web::Path<(String, usize)>,
    gm: web::Data<GameManager>,
) -> impl Responder {
    let (id, newmove) = path.into_inner();
    match gm.getgame(id) {
        Some(g) => {
            
            g.addmove(newmove);
            g.show()
            
        },
        None => "Game not found!".to_string()
    }
}
