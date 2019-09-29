use actix::{Addr, Message};
use crate::actors::main_actor::MainActor;
use crate::actors::child_actor::ChildActor;

#[derive(Debug, Message, Clone)]
pub struct GuessRequestMsg {
  pub max_number: i64,
  pub main_addr: Addr<MainActor>,
  pub game_nr: i64
}

#[derive(Debug, Message, Clone)]
pub struct GuessMsg {
    pub number: i64,
    pub child_id: i64,
    pub child_addr: Addr<ChildActor>,
    pub game_nr: i64
}