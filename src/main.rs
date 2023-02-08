use actix_web::{App, HttpServer, HttpResponse, Responder, get, post, middleware::Logger, web, http::header};
use nanoid::nanoid;
use std::{sync::{Arc, Mutex}, collections::HashMap};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let games:HashMap<String, String> = HashMap::new();
    let appdata = Arc::new(Mutex::new(games));

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(Arc::clone(&appdata)))
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
async fn newgame(games: web::Data<Mutex<HashMap<String, String>>>) -> impl Responder {
    let gameid = nanoid!(8);
    games.into_inner().lock().unwrap()
    .insert(gameid.clone(), "Here goes our Game State".to_string()); 

    
    HttpResponse::Found().append_header((header::LOCATION, gameid)).finish()
}

#[get("/{game_id}")]
async fn game(id: web::Path<String>, games: web::Data<Mutex<HashMap<String, String>>>) -> impl Responder {
    let id = id.into_inner();
    match games.into_inner().lock().unwrap().get(&id) {
        Some(game) => format!("Game ID: {id}, Game Data: {game}"),
        None => format!("Game does not exist: Game ID: {id}")
    }
    
}

#[post("/{game_id}/{move}")]
async fn addmove(path: web::Path<(String, usize)>, games: web::Data<Mutex<HashMap<String, String>>>) -> impl Responder {
    let (id, newmove) = path.into_inner();
    match games.into_inner().lock().unwrap().insert(id.clone(), format!("Last move was: {newmove}")) {
        Some(d) => format!("Game ID: {id}, Game Data: {d}"),
        None => format!("Game does not exist: Game ID: {id}")
    }

}

