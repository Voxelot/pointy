
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Messages {
    INVALID,
    Point(Point)
}

impl Default for Messages {
    fn default() -> Self {
        Self::INVALID
    }
}