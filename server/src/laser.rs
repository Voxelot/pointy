use crate::model::{Messages};
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use nannou::prelude::*;
use nannou_laser as laser;

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

struct Laser {
    point_idx: usize,
    position: Point2,
}

fn laser(laser: &mut Laser, buffer: &mut laser::Buffer) {
    // Write white points to the laser stream at the current position.
    for point in buffer.iter_mut() {
        point.color = [1.0, 1.0, 1.0];
        point.position = laser.position.into();
        // Many lasers have a feature called "scan fail safety" (SFS) where the beam will
        // automatically cut out if the scanner is not moving for safety.
        // To avoid cutting out, we'll offset the point slightly to make a diamond shape.
        let offset = 0.125;
        match laser.point_idx % 4 {
            0 => point.position[0] += offset * 0.5,
            1 => point.position[1] += offset * 0.5,
            2 => point.position[0] -= offset * 0.5,
            _ => point.position[1] -= offset * 0.5,
        }
        laser.point_idx = laser.point_idx.wrapping_add(1);
    }
}

pub struct LaserActor {
    laser_stream: Option<laser::RawStream<Laser>>,
}

impl Default for LaserActor {
    fn default() -> Self {
        LaserActor {
            laser_stream: Option::None
        }
    }
}

impl Actor for LaserActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        let laser_model = Laser {
            point_idx: 0,
            position: pt2(0.0, 0.0),
        };
        let _laser_api = laser::Api::new();
        let laser_stream = _laser_api
            .new_raw_stream(laser_model, laser)
            .build()
            .unwrap();
        self.laser_stream = Option::Some(laser_stream);

        println!("Laser Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Laser Actor is stopped");
    }
}

/// Define handler for `Messages` enum
impl Handler<Messages> for LaserActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Messages::Point(point) => {
                println!("drawing point {}", serde_json::to_string(&point).unwrap());
                let laser_rect = geom::Rect::from_w_h(2.0f32, 2.0f32);
                let point_rect = geom::Rect::from_w_h(1.0f32, 1.0f32);
                let x = point_rect.x.map_value(point.x, &laser_rect.x);
                let y = point_rect.y.map_value(point.y, &laser_rect.y);
                if let Some(laser_stream) = &self.laser_stream {
                    laser_stream.send(move |laser| {
                        laser.position = pt2(x, y);
                    }).unwrap();
                }
                Responses::GotPoint
            },
            _ => {
                Responses::UNKNOWN
            }
        }
    }
}