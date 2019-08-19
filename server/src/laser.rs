use crate::model::{Point, Messages};
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;

pub enum Responses {
    GotPoint,
    UNKNOWN
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Message for Messages {
    type Result = Responses;
}

pub struct LaserActor;

impl Actor for LaserActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Laser Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Laser Actor is stopped");
    }
}

/// Define handler for `Messages` enum
impl Handler<Messages> for LaserActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Messages::Point(point) => {
                println!("drawing point {}", serde_json::to_string(&point).unwrap());
                Responses::GotPoint
            },
            _ => {
                Responses::UNKNOWN
            }
        }
    }
}