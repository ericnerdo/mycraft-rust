mod world;
mod graphics;
pub mod game;

fn main() {
    let mut scene = game::scene::Scene::new();
    pollster::block_on(scene.run());
}
