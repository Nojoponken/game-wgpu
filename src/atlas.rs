enum Atlas {
    Grass,
    Stone,
    Dirt,
}

fn get_texture(origin: Atlas) -> [[f32; 2]; 4] {
    let [x, y] = match origin {
        Atlas::Grass => [0.0, 0.0],
        Atlas::Stone => [0.0625, 0.0],
        Atlas::Dirt => [0.125, 0.0],
    };
    [
        [x, y],
        [x + 0.0625, y],
        [x, y + 0.0625],
        [x + 0.0625, y + 0.0625],
    ]
}
