extern crate actix;

use actix::{Actor, Addr, Context, System, AsyncContext, Handler, Message};
use crate::actors::child_actor::ChildActor;
use crate::actors::common_msg::{GuessRequestMsg, GuessMsg};
use std::collections::HashMap;
use rand::Rng;
use log::{error};

#[derive(Message, Clone, Debug)]
pub struct RunActors{}

#[derive(Clone, Debug)]
pub struct MainActor {
    pub system_name: String,
    pub game_nr: i64,
    pub number: i64,
    pub max_number: i64,
    pub child_actors: Vec<Addr<ChildActor>>,
    pub points: HashMap<String, i64>,
    pub addr: Option<Addr<MainActor>>
}

#[derive(Debug)]
pub struct MsgAddChildActor{}
impl Message for MsgAddChildActor {
  type Result = Option<Addr<ChildActor>>;
}

impl MainActor {
    pub fn run(self) {
        println!("RUN");
        let system_name = self.clone().system_name;
        let system = System::new(system_name.to_string());
        let _ = self.start();
        system.run();
    }

    pub fn add_child_actor(mut self, new_actor: Addr<ChildActor>) {
        &self.child_actors.push(new_actor);
    }

    pub fn new_game(&mut self) {
        let mut rng = rand::thread_rng();
        let max_number = self.max_number.clone();
        let lucky_number = rng.gen_range(0, max_number);
        self.number = lucky_number;
        self.game_nr += 1;
        let guess_request = GuessRequestMsg{max_number: max_number, main_addr: self.addr.clone().unwrap(), game_nr: self.game_nr};
        for actor in self.child_actors.clone() {
            match actor.try_send(guess_request.clone()) {
                Ok(()) => (),
                _ => {
                    error!("Cannot send and GuessRequestMsg to Actor {:?} ", actor);
                    ();
                }
            };
        };
    }

}

impl Handler<GuessMsg> for MainActor {
  type Result = ();

  fn handle(&mut self, msg: GuessMsg, ctx: &mut Context<MainActor>) {
      println!("Guess: {:?} and the number is: {:?}", msg.clone().number, self.number.clone());
      println!("GAME NR: {:?} - {:?}", self.game_nr.clone(), msg.game_nr.clone());
    if self.number.clone() == msg.number && self.game_nr.clone() == msg.game_nr {
        let child_points = self.points.entry(msg.child_id.to_string()).or_insert(0);
        *child_points += 1;
        self.new_game();
    } else {
        let guess_request = GuessRequestMsg{max_number: self.max_number, main_addr: self.addr.clone().unwrap(), game_nr: self.game_nr};
        match msg.child_addr.try_send(guess_request) {
            Ok(()) => (),
            _ => {
                error!("Can not send guess request to actor: {}", msg.child_id);
            }
        };
    }
  }
}

impl Actor for MainActor {
  type Context = Context<MainActor>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.addr = Some(ctx.address());
    self.new_game();
    println!("I am MainActor and I am alive!");
  }
}
