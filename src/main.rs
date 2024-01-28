use renderer::run;
mod atlas;
mod block;
mod renderer;
mod terrain;
fn main() {
    pollster::block_on(run());
}
