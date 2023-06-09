pub mod color {
    pub const RED: i32 = 31;
    pub const BLUE: i32 = 32;
    pub const MAGENTA: i32 = 35;
    pub const WHITE: i32 = 37;
}

pub mod coat_size {
    pub const X: i32 = 32;
    pub const Y: i32 = 32;
}

pub mod icon {
    pub const BALL: &str = "＠";
    pub const SPACE: &str = "  ";
    pub const RACKET: &str = "｜";
    pub const BULLET: &str = "・";
}

pub mod group {
    pub const SYSTEM: usize = 0;
    pub const BALL: usize = 1;
    pub const BULLET: usize = 2;
}
pub const MAX_COMPONENTS: usize = 100;
pub const MAX_SCROLL_MESSAGE: usize = 20;
pub const MAX_STATIC_MESSAGE: usize = 10;
pub const BALL_SIZE: f64 = 0.2;
pub const BULLET_SIZE: f64 = 0.1;
pub const BULLET_FIRE_SPAN: f64 = 0.091;
pub const BALL_SPAWN_SPAN: f64 = 0.5;
pub const BALL_SPAWN_MULTIPRIER: usize = 1;
