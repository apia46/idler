use parley::{Alignment, AlignmentOptions, FontContext, FontWeight, GenericFamily, LayoutContext, PositionedLayoutItem, StyleProperty};
use vello::{Scene, kurbo::{Affine, Rect, Vec2}, peniko::{Color, Fill, Gradient, ImageBrush, ImageData, color::{AlphaColor, Srgb}}};

pub use state::RenderState;

use crate::game::GameState;

mod state;
pub mod button;
pub mod label;

#[derive(Default)]
pub struct Renderer {
    scene: Scene,
    font_cx: FontContext,
    layout_cx: LayoutContext<ColorBrush>,
}

pub struct DrawRectArgs {
    rect: Rect,
}


pub struct DrawTextArgs<'a> {
    position: Vec2,
    text: &'a str,
}

impl Renderer {
    pub fn draw_scene(&mut self, state:&RenderState) -> &Scene {
        self.scene.reset();
        state.draw(self);
        &self.scene
    }

    pub fn draw_rect(&mut self, args:DrawRectArgs) {
        self.scene.fill(Fill::NonZero, Affine::default(), Color::BLACK, None, &args.rect);
    }

    pub fn draw_text(&mut self, args:DrawTextArgs) {
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, args.text, 1., true);
        builder.push_default(GenericFamily::SystemUi);
        builder.push_default(StyleProperty::FontSize(16.));
        builder.push_default(FontWeight::new(400.));
        let color = Color::WHITE;
        builder.push_default(StyleProperty::Brush(ColorBrush(color)));
        let mut layout = builder.build(args.text);
        layout.break_all_lines(None);
        layout.align(Alignment::Start, AlignmentOptions::default());
        for line in layout.lines() {
            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        let run = glyph_run.run();
                        let coords = bytemuck::cast_slice::<_, i16>(run.normalized_coords());
                        self.scene.draw_glyphs(run.font())
                            .font_size(run.font_size())
                            .normalized_coords(coords)
                            .transform(Affine::translate(args.position))
                            .brush(glyph_run.style().brush)
                            .draw(Fill::NonZero, glyph_run.positioned_glyphs().map(|g| {
                                vello::Glyph { id: g.id, x: g.x, y: g.y }
                            }));
                    },
                    PositionedLayoutItem::InlineBox(inline_box) => {
                        let (x0, y0) = (inline_box.x as f64, inline_box.y as f64);
                        let (x1, y1) = (x0 + inline_box.width as f64, y0 + inline_box.height as f64);
                        self.scene.fill(Fill::EvenOdd, Affine::default(), AlphaColor::<Srgb>::BLACK, None, &Rect {
                            x0, y0, x1, y1
                        });
                    },
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ColorBrush(Color);

impl Default for ColorBrush {
    fn default() -> Self {
        ColorBrush(Color::BLACK)
    }
}

impl From<ColorBrush> for vello::peniko::Brush<ImageBrush<&ImageData>, &Gradient> {
    fn from(ColorBrush(value): ColorBrush) -> Self {
        value.into()
    }
}

pub trait Renderable {
    type NewArgs;
    fn new(args:Self::NewArgs, state:&GameState) -> Self;

    fn update(&mut self, state:&GameState) {
        let _ = state;
    }

    fn draw(&self, renderer:&mut Renderer);
}

