use macros::parse_color;
use vello::peniko::Color;

use crate::grid::{self, Direction, GridPoint};
use crate::render::{Renderer, camera::Camera, draw::{Style, DrawHexagonArgs}};
use crate::util::ComponentWise;
use crate::window::Window;

const COLOR:Color = parse_color!("#131646");

pub fn draw(renderer:&mut Renderer, camera:&Camera, window:&Window) {
    let size = camera.vec2_to_world(window.size);
    let tiles = size.div(grid::RECT_SIZE).ceil();
    let (columns, rows) = (tiles.x as u32 + 2, tiles.y as u32 + 2);
    let top_left = GridPoint::nearest(camera.pos) + Direction::LUP * 1;

    let mut column_top = top_left;
    // slow; figure out something faster
    for column in 0..columns {
        for row in 0..rows {
            renderer.draw_hexagon(DrawHexagonArgs {
                point: column_top + Direction::DOWN * row,
                style: Style { stroke: Some((6., COLOR)), fill: None } }, camera);
        }
        if column % 2 == 0 { column_top += Direction::UP * 1; }
        column_top += Direction::ROWN * 1;
    }
}

