#![allow(unused)]

use actix_web::{
    get, http::{header, StatusCode}, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder,
};
use actix_files::{self as fs, NamedFile};
use gamemanager::GameManager;
use nanoid::nanoid;
use std::{
    path::PathBuf,
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
            .service(game_events)
            //.service(game)
            .service(addmove)
            .service(fs::Files::new("/{gameid}", "client"))
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
    let gameurl = format!("{gameid}/client.html");
    match games.newgame(gameid) {
        Ok(_) => HttpResponse::Found()
            .append_header((header::LOCATION, gameurl))
            .finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{game_id}/{file}")]
async fn game(filename: web::Path<(String, String)>) -> impl Responder {
    let (_, file) = filename.into_inner();
    let file = format!("./client/{file}");
    log::info!("{:?}", file);
    let path:PathBuf = file.parse().unwrap();
    NamedFile::open(path).unwrap()
    
}

#[get("/{game_id}/events")]
async fn game_events(
    id: web::Path<String>,
    gm: web::Data<GameManager>,
) -> impl Responder {
    log::info!("Looking for Game {}", &id);
    match gm.getgame(id.into_inner()) {
        Some(g) => {
            log::info!("Game Found, Joining...");
            Some(g.join().await)           
        },
        None => {
            log::error!("Could not find game!");
            None
        }
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
            
            if g.addmove(newmove) {
                g.show().await;
                "Move Accepted"
            } else {
                "Invalid Move!"
            }                        
        },
        None => "Game not found!"
    }
}
