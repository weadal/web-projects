use std::f64;
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    pub fn add(&mut self, v2: Vector2) {
        self.x += v2.x;
        self.y += v2.y;
    }

    pub fn sqr_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn distance(&self, to_position: &Vector2) -> f64 {
        let a = *self - *to_position;
        a.magnitude()
    }

    pub fn normalize(&self) -> Vector2 {
        let length = f64::sqrt(self.sqr_magnitude());

        //ゼロ除算回避
        if length <= 0.0 {
            return Vector2::zero();
        }

        Vector2 {
            x: self.x / length,
            y: self.y / length,
        }
    }

    pub fn right(&self) -> Vector2 {
        //[ 0  1]
        //[-1  0]

        //(1,0)を(0,1)にし、
        //(0,1)を(-1,0)にする行列をベクトルに掛ける　※ターミナル描画系だと(0, 1)は下方向になる

        // x =  Vx * 0 + Vy * -1
        // y =  Vx * 1 + Vy * 0
        let vec = self.clone();

        let x = vec.x * 0.0 + vec.y * -1.0;
        let y = vec.x * 1.0 + vec.y * 0.0;

        Vector2 { x, y }
    }
    pub fn left(&self) -> Vector2 {
        let vec = self.clone();

        let x = vec.x * 0.0 + vec.y * 1.0;
        let y = vec.x * -1.0 + vec.y * 0.0;

        Vector2 { x, y }
    }
    pub fn rotate(&self, angle: f64) -> Vector2 {
        let rad = f64::to_radians(angle);

        let x = self.x * f64::cos(rad) + self.y * -f64::sin(rad);
        let y = self.x * f64::sin(rad) + self.y * f64::cos(rad);

        Vector2 { x, y }
    }
}
impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Transform {
    pub position: Vector2,
    pub scale: f64,
    pub velocity: Vector2,
}

pub struct Clock {
    pub time: Vec<Option<f64>>,
    pub alarm: Vec<Option<f64>>,
}
impl Clock {
    pub fn new() -> Self {
        Clock {
            time: vec![Some(0.0)],
            alarm: vec![None],
        }
    }
    pub fn time_reset(&mut self, index: usize) {
        if index >= self.time.len() {
            panic!("index out of range");
        }
        if let Some(_) = self.time[index] {
            self.time[index] = Some(0.0);
        }
    }
    pub fn time_set(&mut self, time: f64, index: usize) {
        if index >= self.time.len() {
            panic!("index out of range");
        }

        self.time[index] = Some(time);
    }
}

pub enum GameState {
    Title,
    Main,
    GameOver,
}
