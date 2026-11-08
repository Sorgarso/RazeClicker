use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Action {
    Key { vk: u32, down: bool },
    Button { btn: u8, down: bool }, // 0=L 1=R 2=M 3=X1 4=X2
    Move { x: i32, y: i32 },
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RecEvent {
    pub t_us: u64,
    pub action: Action,
}

pub const REC_IDLE: u8 = 0;
pub const REC_RECORDING: u8 = 1;
pub const REC_PLAYING: u8 = 2;
