use vello::kurbo::Point;
use vello::peniko::Color;

use crate::game::Reference;
use crate::grid::{Direction, GridPoint};
use crate::num::{Number, Quantity};

use crate::render::{UpdateContext, Renderer, Renderable, camera::Camera, draw::DrawLineArgs};
use crate::state::resource::ResourceKey;
use crate::util::Lerp;

pub struct ResourceTree {
    #[expect(unused)]
    key: ResourceKey,
    reference: Reference<Quantity>,
    segments: Vec<Segment>,
    fill_color: Color,
    back_color: Color,
    border_color: Color,
}

#[derive(Clone, Copy)]
struct Segment {
    start: Point,
    end: Point,
    base: Quantity,
    growth: Number,
    progress: f64,
}

pub struct NewResourceTreeArgs {
    pub key: ResourceKey,
    pub reference: Reference<Quantity>,
    pub segments: Vec<NewSegmentArgs>,
    pub fill_color: Color,
    pub back_color: Color,
    pub border_color: Color,
}

pub struct NewSegmentArgs {
    pub start: GridPoint,
    pub direction: Direction,
    pub length: u32,
    pub base: Quantity,
    pub growth: Number,
}

impl<'a> Renderable<'a> for ResourceTree {
    type NewArgs = NewResourceTreeArgs;
    fn new(args:NewResourceTreeArgs) -> Self {
        let NewResourceTreeArgs {
            key, reference, segments:segment_args,
            fill_color, back_color, border_color
        } = args;
        let mut segments = Vec::with_capacity(segment_args.len());
        for NewSegmentArgs { start, direction, length, base, growth } in segment_args {
            let end = (start + direction * length).to_point();
            let start = start.to_point();
            segments.push(Segment {
                start, end, base, growth, progress: 0.,
            });
        }
        ResourceTree { key, reference, segments, fill_color, back_color, border_color }
    }

    fn update(&mut self, ctx:&mut UpdateContext) {
        let resource = ctx.game_state.fetch(self.reference).to_number();
        for segment in &mut self.segments {
            segment.progress = (resource / segment.base.to_number()).logf(segment.growth).clamp(0., 1.);
        }
    }

    fn draw(&self, renderer:&mut Renderer, camera:&Camera) {
        for Segment { start, end, .. } in self.segments.iter().copied() {
            renderer.draw_line(DrawLineArgs { start: start, end: end, stroke: (16., self.border_color) }, camera);
        }
        for Segment { start, end, .. } in self.segments.iter().copied() {   
            renderer.draw_line(DrawLineArgs { start: start, end: end, stroke: (12., self.back_color) }, camera);
        }
        for Segment { start, end, progress, .. } in self.segments.iter().copied() {   
            if progress > 0. {
                let progress_end = progress.lerp(start, end);
                renderer.draw_line(DrawLineArgs { start, end: progress_end, stroke: (12., self.fill_color ) }, camera);
            }
        }
    }
}

