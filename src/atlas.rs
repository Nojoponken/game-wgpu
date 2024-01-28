pub enum Atlas {
    GrassTop,
    Stone,
    Dirt,
    GrassSide,
    Plank,
    StoneSlabSide,
    StoneSlabTop,
    Brick,
    TntSide,
    TntTop,
    TntBottom,
    Cobble,
    Bedrock,
    Sand,
    MossyCobble,
}

pub fn get_texture(origin: Atlas) -> [[f32; 2]; 4] {
    let [x, y] = match origin {
        Atlas::GrassTop => [0.0, 0.0],
        Atlas::Stone => [0.0625, 0.0],
        Atlas::Dirt => [0.125, 0.0],
        Atlas::MossyCobble => [0.25, 0.125],
        _ => [0.875, 0.0625],
    };
    [
        [x, y],
        [x + 0.0625, y],
        [x, y + 0.0625],
        [x + 0.0625, y + 0.0625],
    ]
}
