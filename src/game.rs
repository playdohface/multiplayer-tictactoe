/// Types and methods related to managing a single game

use actix_web_lab::sse::{self, ChannelStream};
use std::sync::{Arc, Mutex};
use crate::tictactoe::{self, Board, Player};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
enum GameError {
    MissingPlayer,
    NoPlayers,
    PoisonedMutex,
}

#[derive(Debug, Clone)]
struct ActivePlayer {
    stream: sse::Sender,
    credentials: String,
    score: usize,
}
impl ActivePlayer {
    pub fn new(connection: sse::Sender) -> Self {
        let cred = nanoid::nanoid!(12);
        ActivePlayer {  
            stream: connection,
            credentials: cred,
            score: 0,
        }
    }
    pub async fn ping(&self) -> bool {
        self.stream
            .send(sse::Event::Comment("ping".into()))
            .await
            .is_ok()
    }
    pub async fn notify(&self, msg: impl Into<&str>) -> bool {
        let msg = self
            .stream
            .send(sse::Data::new(msg.into()).event("notification"))
            .await;
        msg.is_ok()
    }
    pub async fn send_credentials(&self) -> bool {
        let msg = self
            .stream
            .send(sse::Data::new(self.credentials.clone()).event("credentials"))
            .await;
        msg.is_ok()
    }
}

#[derive(Debug)]
pub struct Game {
    inner: Mutex<GameInner>,
}
#[derive(Debug, Clone)]
struct GameInner {
    pub board: tictactoe::Board,
    players: [Option<ActivePlayer>; 2],
    spectators: Vec<sse::Sender>,
    started: bool,

}
impl Game {
    pub fn new() -> Arc<Self> {
        Arc::new(Game {
            inner: Mutex::new(GameInner {
                board: tictactoe::Board::new(),
                players: [None, None],
                spectators: Vec::new(),
                started: false,

            }),
        })
    }
    async fn game_ok(&self) -> Result<[ActivePlayer;2], GameError> {
        if !self.healtchcheck().await? {
            return Err(GameError::MissingPlayer);
        }
        let g = match self.inner.lock() {
            Ok(guard) => guard,
            Err(e) => {
                return Err(GameError::PoisonedMutex);
            }
        };
        let players = g.players.clone();
        drop(g);
        if players[0].is_some() && players[1].is_some() {
            Ok([players[0].clone().unwrap(),players[1].clone().unwrap()])
        } else {
            Err(GameError::MissingPlayer)
        }
    }
    /// Checks whether both players' connections work, sets the player to None if not
    /// Returns Ok(true) if there are two players with working connections
    async fn healtchcheck(&self) -> Result<bool, GameError> {
        let mut ready = true;
        if self.inner.is_poisoned() {
            // Additional steps to insure consistency of state may be required
            //self.inner.clear_poison();
            return Err(GameError::PoisonedMutex);
        }
        let players = &mut self.inner.lock().unwrap().players;       
        for player in players {
            ready = ready && match player {
                Some(p) => { 
                    if !p.ping().await {
                        *player = None;
                        false
                    } else { true } 
                },
                None => false
            }
        }
        Ok(ready)       
    }
    
    async fn try_recover(&self) -> bool {
        false
    }
    async fn check_ready(&self) {
        let players = self.game_ok().await;
        let mut g =  self.inner.lock().unwrap();
        if players.is_ok() && !g.started {
            g.started = true;
            for i in 0..2 {
                    if let Some(p) = &g.players[i] {
                    p.stream.send(sse::Data::new("").event("startgame")).await;
                    match i {
                        0 => {p.notify("Game Ready, make the first move!").await;},
                        1 => {p.notify("Game Ready! You are Player O, wait for your opponents move.").await;},
                        _ => ()
                    }
                }             
            }
        }
    }
    pub async fn join(&self) -> sse::Sse<ChannelStream> {
        let (tx, rx) = sse::channel(30);
        match self.healtchcheck().await {
            Err(_) => {
                self.try_recover();
            }
            Ok(true) => {
                tx.send(sse::Data::new("").event("startgame")).await;

                tx.send(sse::Data::new("You are a spectator in this game").event("notification")).await;
                self.inner.lock().unwrap().spectators.push(tx);
               
            }
            Ok(false) => {
                let mut g = self.inner.lock().unwrap();
                if g.players[0].is_none() {
                    g.players[0] = Some(ActivePlayer::new(tx));
                    if let Some(p) = &g.players[0] {
                        p.send_credentials().await;
                        p.notify("You are Player X in this game").await;
                    }
                } else if g.players[1].is_none() {
                    g.players[1] = Some(ActivePlayer::new(tx));
                    if let Some(p) = &g.players[1] {
                        p.send_credentials().await;
                        p.notify("You are Player O in this game").await;
                    }
                }
            }
        }
        self.check_ready().await;       
        rx
    }

    pub async fn rematch(&self, cred: String) -> bool {
        match self.healtchcheck().await {
            Ok(true) => (),
            _ => { return false; }
        }
        let mut g = self.inner.lock().unwrap();
        if  !(g.players[0].as_ref().unwrap().credentials == cred || g.players[1].as_ref().unwrap().credentials == cred) {
            return false;
        }
        g.players.swap(0, 1);
        g.board = tictactoe::Board::new();
        g.started = false;
        drop(g);
        self.check_ready().await;
        //self.show().await;
        true
    }

    pub async fn addmove(&self, newmove: usize, cred: String) -> bool {
        log::info!("Move: {newmove}, Credentials: {cred}");
        let mut players; 
        if let Ok(p) = self.game_ok().await {
            players = p;
        } else {
            return false;
        }
        
        let mut g = self.inner.lock().unwrap();
        
        let board = &mut g.board;
        let index = match board.next_turn {
            tictactoe::Player::X => 0,
            tictactoe::Player::O => 1,
        };
        if players[index].credentials == cred {
            board.add_turn(newmove)
        } else {
            false
        }
        
    }
    pub async fn show(&self) {
        log::info!("Showing Game");
        match self.inner.lock() {
            Ok(g) => { 
                let gameinfo = GameInfo {
                gamestate: g.board.show(),
                outcome: g.board.get_winner(),
                };
                let boardstate = serde_json::to_string(&gameinfo)
                .unwrap();
                let turn = match g.board.next_turn {
                    tictactoe::Player::X => 0,
                    tictactoe::Player::O => 1
                };
                for i in 0..2 {
                    if let Some(p) = g.players[i].clone() {
                        p.stream.send(sse::Data::new(boardstate.clone())).await;
                        if gameinfo.outcome == None {
                            if turn == i {
                                p.notify(&*format!("Your turn, {}!", g.board.next_turn)).await;
                            } else {
                                p.notify("Wait for your opponent").await;
                            } 
                        }                      
                    }
                }

                for spec in &g.spectators.clone() {
                    spec.send(sse::Data::new(boardstate.clone())).await;
                }
                log::info!("All Messages sent");
            }
            Err(e) => {
                log::error!("Could not show Game due to {:?}", e);
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct GameInfo {
    gamestate: [tictactoe::Field; 9],

    outcome: Option<(tictactoe::Field, usize)>,
}

#[cfg(test)]
mod tests {
    
    use actix_web::{body::{MessageBody, BodySize}, test, HttpRequest, HttpResponse, HttpMessage, Responder};
    use actix_web::http::{header, StatusCode};

    use super::*;

    #[actix_web::test]
    async fn rematch_works() {
        let g = Game::new();
        let s1 = g.join().await;
        assert_eq!(Ok(false), g.healtchcheck().await);
        let s2 = g.join().await;
        assert_eq!(Ok(true), g.healtchcheck().await);
        let cred1 = g.inner.lock().as_ref().unwrap().players[0].as_ref().unwrap().credentials.clone();
        let cred2 = g.inner.lock().as_ref().unwrap().players[1].as_ref().unwrap().credentials.clone();
        assert!(g.rematch(cred1.clone()).await);
        let credafter1 = g.inner.lock().as_ref().unwrap().players[0].as_ref().unwrap().credentials.clone();
        let credafter2 = g.inner.lock().as_ref().unwrap().players[1].as_ref().unwrap().credentials.clone();
        assert_eq!(cred1, credafter2);
        assert_eq!(cred2, credafter1);
    }

    #[actix_web::test]
    async fn can_add_moves() {
        let g = Game::new();
        let s1 = g.join().await;
        assert_eq!(Ok(false), g.healtchcheck().await);
        let s2 = g.join().await;
        assert!(g.game_ok().await.is_ok());
        let players = g.game_ok().await.unwrap();
        assert!(g.addmove(0, players[0].credentials.clone()).await);  
        assert!(!g.addmove(1, players[0].credentials.clone()).await);
        assert!(g.addmove(6, players[1].credentials.clone()).await);
        assert!(g.addmove(1, players[0].credentials.clone()).await);
        assert!(g.addmove(7, players[1].credentials.clone()).await);
        assert!(None == g.inner.lock().unwrap().board.get_winner());
        assert!(g.addmove(2, players[0].credentials.clone()).await);
        assert!(Some((tictactoe::Field::X, 0)) == g.inner.lock().unwrap().board.get_winner());
        
        assert!(!g.addmove(8, players[1].credentials.clone()).await);       
       
     
    }
    #[actix_web::test]
    async fn can_join_empty_game() {
        let g = Game::new();
        let s1 = g.join().await;
        assert_eq!(Ok(false), g.healtchcheck().await);
        let s2 = g.join().await;
        assert_eq!(Ok(true), g.healtchcheck().await);        
    }
    #[actix_web::test]
    async fn rejoin_when_player_drops() {
        let g = Game::new();
        let s1 = g.join().await;
        assert_eq!(Ok(false), g.healtchcheck().await);
        let s2 = g.join().await;
        assert!(g.game_ok().await.is_ok());
        drop(s1);
        assert_eq!(Ok(false), g.healtchcheck().await);
        let s3 = g.join().await;
        assert!(g.game_ok().await.is_ok());
    }
    #[actix_web::test]
    async fn spectators_can_just_drop() {
        let g = Game::new();
        let s1 = g.join().await;
        assert_eq!(Ok(false), g.healtchcheck().await);
        let s2 = g.join().await;
        assert!(g.game_ok().await.is_ok());
        assert_eq!(0, g.inner.lock().unwrap().spectators.len());
        let s3 = g.join().await;
        assert!(g.game_ok().await.is_ok());
        assert_eq!(1, g.inner.lock().unwrap().spectators.len());
        drop(s3);
        assert!(g.game_ok().await.is_ok());
    }

    #[actix_web::test]
    async fn sse_assumptions() {
        let (tx, rx) = sse::channel(5);
        let mut specs: Vec<sse::Sender> = Vec::new();
        specs.push(tx.clone());
        let m = specs[0].send(sse::Event::Comment("ping".into())).await;
        assert!(m.is_ok());
        let testreq = test::TestRequest::default().to_http_request();
        let resp = rx.respond_to( &testreq);
        //let resp = resp.body().as_pin_mut().poll_next();
        assert_eq!(200, resp.status().as_u16());
        //drop(rx);
        assert_eq!(BodySize::Stream, resp.body().size());
        
        let m = specs[0].send(sse::Event::Comment("ping".into())).await;

        //assert!(m.is_err());
    }
}