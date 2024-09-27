use cgmath::Vector2;

use crate::renderer::Rendering2D;

use super::{colorlerp, Partical, ParticalShape, Player};

pub fn update_particals(particals: &mut Vec<Partical>, dt: f32) {
    for partical in &mut *particals {
        partical.pos += partical.vel * dt;
        partical.time += dt;
    }
    particals.retain(|partical| partical.time < partical.duration);
}

pub fn draw_particals(
    drawing: &mut Rendering2D<'_, '_>,
    particals: &mut Vec<Partical>,
) {
    for partical in particals {
        let lerped_color = colorlerp(
            partical.starting_color,
            partical.ending_color,
            partical.time / partical.duration,
        );
        match partical.shape {
            ParticalShape::Square => {
                drawing.draw_quad(
                    partical.pos,
                    Vector2::new(partical.size, partical.size),
                    lerped_color,
                    0.0,
                    None,
                );
            }
            //    drawing.draw_rectangle_v(
            //    Vector2::new(
            //        partical.pos.x - player.pos.x + screenwidth as f32 / 2.0 - partical.size / 2.0,
            //        partical.pos.y - player.pos.y + screenheight as f32 / 2.0 - partical.size / 2.0,
            //    ),
            //    Vector2::new(partical.size, partical.size),
            //    lerped_color,
            //),
            ParticalShape::Circle => {}
            //    drawing.draw_circle_v(
            //    Vector2::new(
            //        partical.pos.x - player.pos.x + screenwidth as f32 / 2.0,
            //        partical.pos.y - player.pos.y + screenheight as f32 / 2.0,
            //    ),
            //    partical.size / 2.0,
            //    lerped_color,
            //),
            ParticalShape::RotSquare => {}
        }
    }
}
