use renderer::run;
mod atlas;
mod block;
mod renderer;
pub mod terrain;
mod player;
fn main() {
    pollster::block_on(run());
}
