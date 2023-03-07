use crate::game::Game;
/// Types and methods related to managing many games
use actix_web_lab::sse::{self, ChannelStream};
use log::logger;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

pub struct GameManager {
    inner: RwLock<GameManagerInner>,
}
impl GameManager {
    /// start the gamemanager, launch ping
    pub fn init() -> Arc<Self> {
        let this = Arc::new(GameManager {
            inner: RwLock::new(GameManagerInner {
                games: HashMap::new(),
            }),
        });
        GameManager::start_ping(Arc::clone(&this));
        this
    }
    /// cleans up dead games every 5 minutes
    fn start_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = actix_web::rt::time::interval(Duration::from_secs(300));

            loop {
                interval.tick().await;
                log::info!("Cleanup cycle starts");
                this.remove_dead_games().await;
                log::info!("Finished cleanup cycle.");
            }
        });
    }
}
struct GameManagerInner {
    games: HashMap<String, Arc<Game>>,
}
impl GameManager {
    async fn remove_dead_games(&self) -> Result<(), Box<dyn std::error::Error + '_>> {
        let mut deadgames: Vec<String> = Vec::new();
        for (key, game) in &self.inner.read()?.games {
            if game.is_dead().await {
                deadgames.push(key.to_string());
                log::info!("Found dead game, scheduling for removal: {}", key);
            }
        }
        for key in deadgames {
            self.inner.write()?.games.remove(&key);
            log::info!("Removed game: {}", key);
        }
        Ok(())
    }
    /// Create a new game with a given ID
    /// Will overwrite if a game with the same ID already exists (use uuid)
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
    ///Retrieve a game by id
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_web::test]
    async fn can_create_game() {
        let gm = GameManager::init();
        gm.newgame("foo".into());
        assert!(gm.getgame("foo".into()).is_some());
    }
    #[actix_web::test]
    async fn empty_games_are_deleted() {
        let gm = GameManager::init();
        gm.newgame("bar".into());
        gm.remove_dead_games().await;
        assert!(gm.getgame("bar".into()).is_none());
    }

    #[actix_web::test]
    async fn non_empty_games_are_not_deleted() {
        let gm = GameManager::init();
        gm.newgame("baz".into());
        let p1 = gm.getgame("baz".into()).unwrap().join().await;
        gm.remove_dead_games().await;
        assert!(gm.getgame("baz".into()).is_some());
        drop(p1);
        gm.remove_dead_games().await;
        assert!(gm.getgame("baz".into()).is_none());
    }
}
