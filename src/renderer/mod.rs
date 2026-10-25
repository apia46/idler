use parley::{Alignment, AlignmentOptions, FontContext, FontWeight, GenericFamily, LayoutContext, PositionedLayoutItem, StyleProperty};
use vello::{Scene, kurbo::{Affine, Rect}, peniko::{Color, Fill, Gradient, ImageBrush, ImageData, color::{AlphaColor, Hsl, Srgb}}};

pub use state::RenderState;

mod state;

pub struct Renderer {
    scene: Scene,
    font_cx: FontContext,
    layout_cx: LayoutContext<ColorBrush>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            scene: Scene::new(),
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
        }
    }

    pub fn draw_scene(&mut self, state:&mut RenderState) -> &Scene {
        let current_frame = state.start_time.elapsed();
        self.scene.reset();
        let text = format!("vello hro... the current time is: {}", current_frame.as_secs_f32());
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, &text, 1., true);
        builder.push_default(GenericFamily::SystemUi);
        builder.push_default(StyleProperty::FontSize(16.));
        builder.push_default(FontWeight::new(400.));
        let color = AlphaColor::<Hsl>::new([(current_frame.as_millis() as f32 / 10.) % 360., 100., 50., 1.]).convert();
        builder.push_default(StyleProperty::Brush(ColorBrush(color)));
        let mut layout = builder.build(&text);
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
                            .transform(Affine::translate((40., 40.)))
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
        state.previous_frame = current_frame;
        &self.scene
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

