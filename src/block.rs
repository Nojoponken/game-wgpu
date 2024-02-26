use crate::atlas::Atlas;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub block_id: u8,
    pub block_state: u8,
}

pub fn get_texture(block_id: u8, normal: [i8; 3]) -> Atlas {
    enum Face {
        Top,
        Bottom,
        North,
        South,
        West,
        East,
    }
    let face = match normal {
        [-1, 0, 0] => Face::West,
        [1, 0, 0] => Face::East,
        [0, 0, -1] => Face::North,
        [0, 0, 1] => Face::South,
        [0, -1, 0] => Face::Bottom,
        _ => Face::Top,
    };
    match block_id {
        1 => match face {
            Face::Top => Atlas::GrassTop,
            Face::Bottom => Atlas::Dirt,
            _ => Atlas::GrassSide,
        },
        2 => Atlas::Dirt,
        3 => Atlas::Stone,
        4 => Atlas::Sand,
        5 => Atlas::StoneBrick,
        6 => Atlas::MossyBrick,
        7 => Atlas::IronBars,
        8 => Atlas::Plank,
        9 => Atlas::Glass,
        10 => match face {
            Face::Top => Atlas::LogTop,
            Face::Bottom => Atlas::LogTop,
            _ => Atlas::LogSide,
        },
        0 => Atlas::Water,
        _ => Atlas::Unknown,
    }
}
