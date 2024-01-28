use renderer::run;
mod atlas;
mod block;
mod renderer;
fn main() {
    pollster::block_on(run());
}
