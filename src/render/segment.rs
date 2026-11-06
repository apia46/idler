use vello::kurbo::Point;
use vello::peniko::Color;

use crate::game::{GameState, Reference};
use crate::grid::{Direction, GridPoint};
use crate::num::{Number, Quantity};

use crate::render::{Renderer, Renderable, camera::Camera};
use crate::util::Lerp;

use super::draw::DrawLineArgs;

pub struct Segment {
    #[expect(unused)]
    id: usize,
    reference: Reference<Quantity>,
    start: Point,
    end: Point,
    base: Quantity,
    growth: Number,
    fill_color: Color,
    back_color: Color,
    border_color: Color,
    progress: f64,
}

pub struct NewSegmentArgs {
    pub reference: Reference<Quantity>,
    pub start: GridPoint,
    pub direction: Direction,
    pub length: u32,
    pub base: Quantity,
    pub growth: Number,
    pub fill_color: Color,
    pub back_color: Color,
    pub border_color: Color,
}

impl Renderable for Segment {
    type NewArgs = NewSegmentArgs;
    fn new(id:usize, args:NewSegmentArgs, _renderer:&mut Renderer, _state:&GameState) -> Self {
        let NewSegmentArgs { reference, start, direction, length, base, growth, fill_color, back_color, border_color } = args;
        Segment {
            id, reference, start: start.to_point(),
            end: (start + direction * length).to_point(),
            base, growth, fill_color, back_color, border_color,
            progress: 0.,
        }
    }

    fn update(&mut self, _renderer:&mut Renderer, game_state:&GameState, _input_state:&crate::input::InputState) {
        self.progress = ((self.reference.0)(game_state).to_number() / self.base.to_number()).log_f64(self.growth).clamp(0., 1.);
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        let (start, end) = (self.start, self.end);
        renderer.draw_line(DrawLineArgs { start, end, stroke: (16., self.border_color ) }, camera);
        renderer.draw_line(DrawLineArgs { start, end, stroke: (12., self.back_color ) }, camera);
        if self.progress > 0. {
            let progress_end = self.progress.lerp(start, end);
            renderer.draw_line(DrawLineArgs { start, end: progress_end, stroke: (12., self.fill_color ) }, camera);
        }
    }
}

