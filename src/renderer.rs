use parley::{Alignment, AlignmentOptions, FontContext, FontWeight, GenericFamily, LayoutContext, PositionedLayoutItem, StyleProperty};
use vello::{Scene, kurbo::{Affine, Rect}, peniko::{Fill, color::{AlphaColor, Rgba8, Srgb}}};

pub struct Renderer {
    scene: Scene,
    font_cx: FontContext,
    layout_cx: LayoutContext,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            scene: Scene::new(),
            font_cx: FontContext::new(),
            layout_cx: LayoutContext::new(),
        }
    }

    pub fn draw_scene(&mut self) -> &Scene {
        self.scene.reset();
        let text = "vello hro";
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, text, 1., true);
        builder.push_default(GenericFamily::SystemUi);
        builder.push_default(StyleProperty::FontSize(16.));
        builder.push_default(FontWeight::new(400.));
        builder.push_default(StyleProperty::Brush([255,255,255,255]));
        let mut layout = builder.build(text);
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
                            .brush(AlphaColor::from(Rgba8::from_u8_array(glyph_run.style().brush)))
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

        &self.scene
    }
}
//let circle = Circle::new((420.0, 200.0), 120.0);
        //let circle_fill_color = Color::new([0.9529, 0.5451, 0.6588, 1.]);
        //self.scene.fill(
        //    vello::peniko::Fill::NonZero,
        //    Affine::IDENTITY,
        //    circle_fill_color,
        //    None,
        //    &circle,
        //);
