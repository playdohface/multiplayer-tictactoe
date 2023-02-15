#![allow(unused)]
#![allow(unstable_features)]

use actix_files::{self as fs, NamedFile};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key,
    get,
    http::{header, StatusCode},
    middleware::Logger,
    post, web, App, HttpResponse, HttpServer, Responder,
};
use gamemanager::GameManager;
use nanoid::nanoid;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub mod gamemanager;
pub mod tictactoe;
pub mod game;

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
            .service(getgame)
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
    let gameurl = format!("{gameid}/game");
    match games.newgame(gameid) {
        Ok(_) => HttpResponse::Found()
            .append_header((header::LOCATION, gameurl))
            .finish(),
        Err(e) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{game_id}/game")]
async fn getgame(id: web::Path<String>) -> impl Responder {
    let id = id.into_inner();
    let file = format!("client/client.html");
    let path: PathBuf = file.parse().unwrap();
    NamedFile::open(path).unwrap()
}

#[get("/{game_id}/events")]
async fn game_events(id: web::Path<String>, gm: web::Data<GameManager>) -> impl Responder {
    let id = id.into_inner();
    match gm.getgame(id.clone()) {
        Some(g) => {
            let stream = g.join().await;

            Some(stream)
        }
        None => {
            log::error!("Could not find game!");
            None
        }
    }
}

#[post("/{game_id}/{move}/{credentials}")]
async fn addmove(
    path: web::Path<(String, usize, String)>,
    gm: web::Data<GameManager>,
) -> impl Responder {
    let (id, newmove, credentials) = path.into_inner();

    match gm.getgame(id) {
        Some(g) => {
            if g.addmove(newmove, credentials).await {
                g.show().await;
                //g.notify_players().await;
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
        None => HttpResponse::NotFound().finish(),
    }
}
