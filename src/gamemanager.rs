use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use actix_web_lab::sse::{self, ChannelStream};
use serde::Serialize;
use serde_json;

use crate::tictactoe::{self, Board};

pub struct GameManager {
    pub inner: RwLock<GameManagerInner>,
}
impl GameManager {
    pub fn init() -> Arc<Self> {
        Arc::new(GameManager {
            inner: RwLock::new(GameManagerInner {
                games: HashMap::new(),
            }),
        })
    }
}
pub struct GameManagerInner {
    pub games: HashMap<String, Arc<Game>>,
}
impl GameManager {
    pub fn newgame(&self, id: String) -> Result<(), Box<dyn std::error::Error + '_>> {
        match self.inner.write()?.games.insert(id.clone(),Game::new()){
            None => Ok(()),
            Some(g) => {
                log::error!("Game with ID {id} already Existed, was overwritten! Old Game: {:?}", g);
                Ok(())
            }
        }
        
    }
    pub fn getgame(&self, id: String) -> Option<Arc<Game>> {
        match self.inner.read() {
            Err(e) => {
                log::error!("Error getting ReaderLock! {:?}", e);
                None
            }
            Ok(guard) => {
                match guard.games.get(&id) {
                    Some(game) => {
                        let u = game.to_owned();                       
                        Some(u) 
                    },
                    None => None
                }
            }
        }// ?.games.get(&id);
    }
}

#[derive(Debug)]
pub struct Game {
    inner: Mutex<GameInner>
}
#[derive(Debug, Clone)]
struct GameInner {
    pub board: tictactoe::Board,
    pub nextup: tictactoe::Player,
    pub ready: bool,
    spectators: Vec<sse::Sender>,
    credentials: [String;2],
}
impl Game {
    pub fn new() -> Arc<Self> {
        Arc::new(
                Game {
                    inner: Mutex::new( GameInner {
                        board: tictactoe::Board::new(),
                        nextup: tictactoe::Player::X,
                        ready: false,
                        spectators: Vec::new(),
                        credentials: [nanoid::nanoid!(12), nanoid::nanoid!(12) ],
            } )
        })
    }
    pub async fn notify_players(&self) {
        let mut g = self.inner.lock().unwrap();
        if g.nextup == tictactoe::Player::X {
            g.spectators[0].send(sse::Data::new("Your move, Player X!").event("notification")).await.unwrap();
            g.spectators[1].send(sse::Data::new("Wait for your opponent, Player O").event("notification")).await.unwrap();
        } else {
            g.spectators[0].send(sse::Data::new("Wait for your opponent, Player X").event("notification")).await.unwrap();
            g.spectators[1].send(sse::Data::new("Your move, Player O!").event("notification")).await.unwrap();  
        }
    }
    pub async fn join(&self) -> (sse::Sse<ChannelStream>, usize) {
        let (tx, rx) = sse::channel(5);

        let mut g = self.inner.lock().unwrap();
        g.spectators.push(tx);
        if g.spectators.len() == 2 {
            g.ready = true;

            g.spectators[0].send(sse::Data::new(g.credentials[0].clone()).event("credentials")).await.unwrap();
            g.spectators[0].send(sse::Data::new("Game Ready! You are Player X, make your move!").event("notification")).await.unwrap();

            g.spectators[1].send(sse::Data::new(g.credentials[1].clone()).event("credentials")).await.unwrap();
            g.spectators[1].send(sse::Data::new("Game Ready! You are Player O, wait for your opponents move!").event("notification")).await.unwrap();

        } else if g.spectators.len() == 1 {
            g.spectators[0].send(sse::Data::new("Waiting for opponent.").event("notification")).await.unwrap();
        }
        (rx, g.spectators.len())
        
    }
    pub async fn notify_all(&self, msg: String) {
        let s = self.inner.lock().unwrap().spectators.clone();
        for spec in s {
            spec.send(sse::Data::new(msg.clone()).event("notification")).await.unwrap();
        }
    }
    pub fn addmove(&self, newmove: usize, c: String) -> bool{
        log::info!("Move: {newmove}, Credentials: {c}");
        match self.inner.lock() {
            Ok(mut g) => {
                if g.ready && 
                ((c == g.credentials[0] && g.nextup == tictactoe::Player::X) || (c == g.credentials[1] && g.nextup == tictactoe::Player::O)) {
                    g.nextup = !g.nextup;
                    g.board.add_turn(newmove)
                } else {
                    false
                }
                
            },
            Err(e) => false
        }
    }
    pub async fn show(&self) {
        log::info!("Showing Game");
        match self.inner.lock() {
            Ok(g) => {     
                let boardstate = serde_json::to_string(
                    &GameInfo {
                        gamestate: g.board.show(),
                        nextup: g.board.next_turn,
                        outcome: g.board.get_winner()
            }).unwrap();      
                for spec in &g.spectators.clone() {
    
                   
                    spec.send(sse::Data::new(boardstate.clone())).await.unwrap();
                          
                }
                log::info!("All Messages sent"); 
                
            },
            Err(e) => {
                log::error!("Could not show Game due to {:?}", e);
                
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct GameInfo {
    gamestate: [tictactoe::Field;9],
    nextup: tictactoe::Player,
    outcome: Option<(tictactoe::Field, usize)>
}