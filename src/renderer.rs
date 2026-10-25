use vello::{Scene, kurbo::{Affine, Circle}, peniko::Color};

pub struct Renderer {
    scene: Scene,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            scene: Scene::new()
        }
    }

    pub fn draw_scene(&mut self) -> &Scene {
        self.scene.reset();
        let circle = Circle::new((420.0, 200.0), 120.0);
        let circle_fill_color = Color::new([0.9529, 0.5451, 0.6588, 1.]);
        self.scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            circle_fill_color,
            None,
            &circle,
        );
        &self.scene
    }
}

