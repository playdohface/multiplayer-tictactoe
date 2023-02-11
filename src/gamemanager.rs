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
    spectators: Vec<sse::Sender>,
}
impl Game {
    pub fn new() -> Arc<Self> {
        Arc::new(
                Game {
                    inner: Mutex::new( GameInner {
                        board: tictactoe::Board::new(),
                    spectators: Vec::new(),
            } )
        })
    }
    pub async fn join(&self) -> sse::Sse<ChannelStream>{
        let (tx, rx) = sse::channel(5);
        //tx.send(sse::Data::new("Spectator Added")).await.unwrap();
        self.inner.lock().unwrap().spectators.push(tx);
        rx
        
    }
    pub fn addmove(&self, newmove: usize) -> bool{
        match self.inner.lock() {
            Ok(mut g) => {
                g.board.add_turn(newmove)
                
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