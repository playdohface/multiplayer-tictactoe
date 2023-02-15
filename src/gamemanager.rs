/// Types and methods related to managing many games

use actix_web_lab::sse::{self, ChannelStream};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use crate::game::Game;


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
        match self.inner.write()?.games.insert(id.clone(), Game::new()) {
            None => Ok(()),
            Some(g) => {
                log::error!(
                    "Game with ID {id} already Existed, was overwritten! Old Game: {:?}",
                    g
                );
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
            Ok(guard) => match guard.games.get(&id) {
                Some(game) => {
                    let u = game.to_owned();
                    Some(u)
                }
                None => None,
            },
        } // ?.games.get(&id);
    }
}
