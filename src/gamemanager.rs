use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

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
        let game = (Game::new());
        match self.inner.write()?.games.insert(id.clone(), game){
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
    //players: [Option<actix_web_lab::sse::Sender>; 2]
}
impl Game {
    pub fn new() -> Arc<Self> {
        Arc::new(
                Game {
                    inner: Mutex::new( GameInner {
                        board: tictactoe::Board::new(),
                //players: [None;2]
            } )
        })
    }
    pub fn addmove(&self, newmove: usize) -> bool{
        match self.inner.lock() {
            Ok(mut g) => g.board.add_turn(newmove),
            Err(e) => false
        }
    }
    pub fn show(&self) -> String {
        match self.inner.lock() {
            Ok(g) => g.board.show(),
            Err(e) => "Error".into()
        }
    }
}
