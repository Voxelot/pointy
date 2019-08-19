#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod laser;
mod model;
mod ws;

use actix::{System, Actor, Addr};
use std::io::Error;
use laser::LaserActor;
use crate::model::Point;

#[derive(Clone)]
pub struct ActorSet {
    laser_actor: Addr<LaserActor>
}


fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    let data = model::Messages::Point(Point {
        x: 5f64,
        y: 1f64
    });
    let data = serde_json::to_string(&data).unwrap();
    println!("{}", data);

    println!("Hello Pointy!");

    // start actors
    let sys = System::new("pointy");
    // start workers
    let actor_set: ActorSet = ActorSet {
        laser_actor: LaserActor.start()
    };
    // init webservice
    ws::start(actor_set);
    sys.run()
}
