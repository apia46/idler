use parley::{Layout, Alignment, AlignmentOptions, FontContext, FontWeight, GenericFamily, LayoutContext, PositionedLayoutItem, StyleProperty};
use vello::{Scene, kurbo::{Affine, Rect, Vec2}, peniko::{Color, Fill, Gradient, ImageBrush, ImageData, color::{AlphaColor, Srgb}}};

pub use state::RenderState;

use crate::{game::GameState, input::InputState};

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
    color: Color,
}

pub struct BuildTextArgs<'a> {
    text: &'a str,
    color: Color,
}

pub type TextLayout = Option<Layout<ColorBrush>>;

impl Renderer {
    pub fn draw_scene(&mut self, state:&RenderState) -> &Scene {
        self.scene.reset();
        state.draw(self);
        &self.scene
    }

    pub fn draw_rect(&mut self, args:DrawRectArgs) {
        self.scene.fill(Fill::NonZero, Affine::default(), args.color, None, &args.rect);
    }

    pub fn build_text(&mut self, args:BuildTextArgs) -> TextLayout {
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, args.text, 1., true);
        builder.push_default(GenericFamily::SystemUi);
        builder.push_default(StyleProperty::FontSize(16.));
        builder.push_default(FontWeight::new(400.));
        builder.push_default(StyleProperty::Brush(ColorBrush(args.color)));
        let mut layout = builder.build(args.text);
        layout.break_all_lines(None);
        layout.align(Alignment::Start, AlignmentOptions::default());
        Some(layout)
    }

    pub fn draw_text(&mut self, position:Vec2, layout:&TextLayout) {
        let Some(layout) = layout else { return; };
        for line in layout.lines() {
            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        let run = glyph_run.run();
                        let coords = bytemuck::cast_slice::<_, i16>(run.normalized_coords());
                        self.scene.draw_glyphs(run.font())
                            .font_size(run.font_size())
                            .normalized_coords(coords)
                            .transform(Affine::translate(position))
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
pub struct ColorBrush(Color);

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
    fn new(id:usize, args:Self::NewArgs, renderer:&mut Renderer, state:&GameState) -> Self;

    fn update(&mut self, renderer:&mut Renderer, game_state:&GameState, input_state:&InputState) {
        let _ =renderer;
        let _ = game_state;
        let _ = input_state;
    }

    fn draw(&self, renderer:&mut Renderer);
}

