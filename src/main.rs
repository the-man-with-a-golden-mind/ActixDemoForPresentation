#[macro_use()]
extern crate log;
extern crate env_logger;
extern crate lib;
use actix::{Actor, Addr, Context, System, AsyncContext, Handler, Message};

use crate::lib::actors::main_actor::{MainActor};
use crate::lib::actors::child_actor::{ChildActor};

use std::collections::HashMap;

fn main() {
    let nr_of_actors = 10000;
    let mut child_actors: Vec<Addr<ChildActor>> = Vec::new();
    for i in 0..nr_of_actors {
        println!("{:?}", i);
        let new_child_actor = ChildActor{
            id: i,
            addr: None,
            game_nr: 0
        };
        let addr = new_child_actor.start();
        child_actors.push(addr);
    };
    let main_actor = MainActor{
    system_name: String::from("TheGame"),
    game_nr: 0,
    number: 0,
    max_number: 100000,
    child_actors: child_actors.clone(),
    points: HashMap::new(),
    addr: None
    };
    let how_many_actors = main_actor.clone().child_actors.len();
    println!("I have {:?} actors", how_many_actors);
    main_actor.run();
}


