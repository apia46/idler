use vello::peniko::Color;

use crate::game::Reference;
use crate::grid::Direction;
use crate::num::{Number, Quantity};
use crate::render::Renderable;
use crate::render::{Renderer, segment::{Segment as RenderSegment, NewSegmentArgs as NewRenderSegmentArgs}};
use crate::state::{State, button::NewButtonArgs};

pub struct NewResourceArgs {
    pub reference: Reference<Quantity>,
    pub start_button: NewButtonArgs,
    pub fill_color: Color,
    pub back_color: Color,
    pub border_color: Color,
    pub segments: Vec<NewSegmentArgs>,
}

pub struct NewSegmentArgs {
    pub direction: Direction,
    pub length: u32,
    pub growth: Number,
}

impl State {
    pub fn new_resource(&mut self, args:NewResourceArgs, renderer: &mut Renderer) {
        let NewResourceArgs { reference, start_button, segments, fill_color, back_color, border_color } = args;

        self.new_button(args.start_button, renderer);

        let segment_count = &mut self.entities.segments;
        let mut position = start_button.position;
        let mut number = Quantity::ONE;
        for segment in segments {
            let NewSegmentArgs { direction, length, growth } = segment;
            self.render_state.segments.push(RenderSegment::new(*segment_count, NewRenderSegmentArgs {
                reference, start: position, direction, length, base: number, growth, fill_color, back_color, border_color
            }, renderer, &self.game_state));
            position += direction * length;
            *number.as_number() *= growth;
            *segment_count += 1;
        }
    }
}

