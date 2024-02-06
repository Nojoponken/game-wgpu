pub enum Atlas {
    Unknown,
    GrassTop,
    GrassSide,
    Dirt,
    Stone,
    Sand,
    StoneBrick,
    MossyBrick,
    Plank,
}

pub fn get_texture_coordinates(origin: Atlas, rotate: u8) -> [[f32; 2]; 4] {
    let [x, y] = match origin {
        Atlas::GrassTop => [0.0, 0.0],
        Atlas::GrassSide => [0.1, 0.0],
        Atlas::Dirt => [0.2, 0.0],
        Atlas::Stone => [0.3, 0.0],
        Atlas::Sand => [0.4, 0.0],
        Atlas::StoneBrick => [0.0, 0.1],
        Atlas::MossyBrick => [0.1, 0.1],
        Atlas::Plank => [0.3, 0.1],
        _ => [0.9, 0.9],
    };
    let mut ret = [[x, y], [x + 0.1, y], [x + 0.1, y + 0.1], [x, y + 0.1]];
    ret.rotate_right((rotate % 4) as usize);
    [ret[0], ret[1], ret[3], ret[2]]
}
