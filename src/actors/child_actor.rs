extern crate actix;

use crate::actors::common_msg::{GuessMsg, GuessRequestMsg};
use actix::{Actor, Addr, AsyncContext, Context, Handler};
use chrono::prelude::*;
use log::info;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct ChildActor {
    pub id: i64,                        // Actor inner id
    pub addr: Option<Addr<ChildActor>>, // Actor addres necessary for messages
    pub game_nr: i64,                   // Actual game nr
}

impl ChildActor {
    pub fn create_new(game_nr: i64) -> Addr<ChildActor> {
        let generated_id: i64 = Utc::now().timestamp_nanos();
        let child_actor = ChildActor {
            id: generated_id,
            addr: None,
            game_nr: game_nr,
        };
        let addr = child_actor.start();
        return addr;
    }

    pub fn get_addr(&self) -> Option<Addr<ChildActor>> {
        self.addr.clone()
    }
}

impl Actor for ChildActor {
    type Context = Context<ChildActor>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
        info!(
            "I am ChildActor {} and I am alive! Context: {:?}",
            self.id.to_string(),
            ctx.address()
        );
    }
}

impl Handler<GuessRequestMsg> for ChildActor {
    type Result = ();

    fn handle(&mut self, msg: GuessRequestMsg, ctx: &mut Context<ChildActor>) {
        if msg.game_nr > self.game_nr {
            self.game_nr = msg.game_nr;
        };
        let mut rng = rand::thread_rng();
        let max_number = msg.max_number;
        let roll = rng.gen_range(0, max_number.clone());
        let guess = GuessMsg {
            number: roll,
            child_id: self.id,
            game_nr: self.game_nr,
            child_addr: ctx.address(),
        };

        // let ten_millis = time::Duration::from_millis((roll * 100) as u64);
        // thread::sleep(ten_millis);
        match msg.main_addr.try_send(guess) {
            Ok(()) => {
                ();
            }
            _ => {
                println!("Cannot send message to MainActor.");
                ();
            }
        };
    }
}
