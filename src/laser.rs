use super::model::Point;
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;

enum Messages {
    SendPoint { point: Point },
}

enum Responses {
    GotPoint,
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
            Messages::SendPoint {point} => Responses::GotPoint,
        }
    }
}