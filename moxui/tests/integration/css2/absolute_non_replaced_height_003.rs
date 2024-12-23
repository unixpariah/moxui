use calc_units::Units;
use moxui::tree::{self, Config};

use crate::WgpuCtx;

pub fn absolute_non_replaced_height_003(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    let config = &wgpu_ctx.surface_config;

    // Test passes if a blue rectangle is vertically centered in an hollow black square.
    moxui::tree::Tree::new(
        &wgpu_ctx.device,
        &wgpu_ctx.queue,
        &Config {
            width: config.width as f32,
            height: config.height as f32,
            format: config.format,
            dpi: 94.1,
        },
        |surface| {
            surface.add_child(|item| {
                item.set_position(moxui::rectangle::Position::Relative)
                    .set_size(Units::In(3.0), Units::In(3.0))
                    .set_border_size(
                        Units::Px(2.0),
                        Units::Px(2.0),
                        Units::Px(2.0),
                        Units::Px(2.0),
                    )
                    .set_border_top_color(1.0, 1.0, 1.0, 1.0)
                    .set_border_bottom_color(1.0, 1.0, 1.0, 1.0)
                    .set_border_left_color(1.0, 1.0, 1.0, 1.0)
                    .set_border_right_color(1.0, 1.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_position(moxui::rectangle::Position::Absolute)
                            .set_background_color(0.0, 0.0, 1.0, 1.0)
                            .set_size(Units::Perc(100.0), Units::In(1.0))
                            .set_margin(Units::Auto, Units::Px(0.0), Units::Auto, Units::Px(0.0))
                            .set_coordinates(
                                Units::In(0.5),
                                Units::Auto,
                                Units::In(0.5),
                                Units::Auto,
                            )
                    })
            })
        },
    )
    .finish()
}
