use parley::{Layout, Alignment, AlignmentOptions, FontWeight, GenericFamily, PositionedLayoutItem, StyleProperty};
use vello::{kurbo::{Affine, Line as KurboLine, Point, Rect, Stroke as KurboStroke, Vec2}, peniko::{Color, Fill as KurboFill, color::{AlphaColor, Srgb}}};

use crate::render::{ColorBrush, Renderer};
use crate::grid::GridPoint;
use crate::render::camera::Camera;

type Stroke = (f64, Color);
type Fill = Color;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub stroke: Option<Stroke>,
    pub fill: Option<Fill>,
}

pub struct DrawRectArgs {
    pub rect: Rect,
    pub color: Color,
}

pub struct DrawHexagonArgs {
    pub point: GridPoint,
    pub style: Style,
}

pub struct DrawLineArgs {
    pub start: Point,
    pub end: Point,
    pub stroke: Stroke,
}

pub struct BuildTextArgs<'a> {
    pub text: &'a str,
    pub font_size: f32,
    pub color: Color,
    pub center: bool,
}

pub struct TextLayout {
    layout: Layout<ColorBrush>, center: bool,
}

impl Renderer {
    #[expect(unused)]
    pub fn draw_rect(&mut self, args:DrawRectArgs, camera:&Camera) {
        self.scene.fill(KurboFill::NonZero, camera.transform_inv(), args.color, None, &args.rect);
    }

    pub fn draw_hexagon(&mut self, args:DrawHexagonArgs, camera:&Camera) {
        let shape = args.point.to_hexagon();
        if let Some(color) = args.style.fill {
            self.scene.fill(KurboFill::NonZero, camera.transform_inv(), color, None, &shape);
        }
        if let Some((width, color)) = args.style.stroke {
            self.scene.stroke(&KurboStroke::new(width), camera.transform_inv(), color, None, &shape);
        }
    }

    pub fn draw_line(&mut self, args:DrawLineArgs, camera:&Camera) {
        let (width, color) = args.stroke;
        self.scene.stroke(
            &KurboStroke::new(width), camera.transform_inv(), color, None,
            &KurboLine { p0: args.start, p1: args.end }
        );
    }

    pub fn build_text(&mut self, args:BuildTextArgs) -> TextLayout {
        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, args.text, 1., true);
        builder.push_default(GenericFamily::SystemUi);
        builder.push_default(StyleProperty::FontSize(args.font_size));
        builder.push_default(FontWeight::new(400.));
        builder.push_default(StyleProperty::Brush(ColorBrush(args.color)));
        let mut layout = builder.build(args.text);
        layout.break_all_lines(None);
        layout.align(if args.center { Alignment::Center } else { Alignment::Start }, AlignmentOptions::default());
        TextLayout { layout, center: args.center }
    }

    pub fn draw_text(&mut self, position:Vec2, layout:&TextLayout, camera:&Camera) {
        let mut translate = position;
        if layout.center {
            translate += Vec2::new(-0.5*layout.layout.full_width() as f64, -0.5*layout.layout.height() as f64)
        }

        for line in layout.layout.lines() {
            for item in line.items() {
                match item {
                    PositionedLayoutItem::GlyphRun(glyph_run) => {
                        let run = glyph_run.run();
                        let coords = bytemuck::cast_slice::<_, i16>(run.normalized_coords());
                        self.scene.draw_glyphs(run.font())
                            .font_size(run.font_size())
                            .normalized_coords(coords)
                            .transform(camera.transform_inv().pre_translate(translate))
                            .brush(glyph_run.style().brush)
                            .draw(KurboFill::NonZero, glyph_run.positioned_glyphs().map(|g| {
                                vello::Glyph { id: g.id, x: g.x, y: g.y }
                            }));
                    },
                    PositionedLayoutItem::InlineBox(inline_box) => {
                        let (x0, y0) = (inline_box.x as f64, inline_box.y as f64);
                        let (x1, y1) = (x0 + inline_box.width as f64, y0 + inline_box.height as f64);
                        self.scene.fill(KurboFill::EvenOdd, Affine::default(), AlphaColor::<Srgb>::BLACK, None, &Rect {
                            x0, y0, x1, y1
                        });
                    },
                }
            }
        }
    }

}

