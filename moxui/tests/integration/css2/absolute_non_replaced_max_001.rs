use calc_units::Units;
use moxui::tree::{self, Config};

use crate::WgpuCtx;

pub fn absolute_non_replaced_max_001(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    let config = &wgpu_ctx.surface_config;

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
            surface
                .add_child(|item| item.set_content("Test passes if there is a green square containing a smaller black square below. "))
                .add_child(|item| 
                    item.set_position(moxui::rectangle::Position::Absolute)
                        .set_size(Units::Em(4.0), Units::Em(4.0))
                        .set_max_size(Units::Em(2.0), Units::Em(2.0))
                        .set_background_color(0.0, 1.0, 0.0, 1.0)
                        .add_child(|item| 
                            item.set_size(Units::Perc(50.0), Units::Perc(50.0)) 
                                .set_background_color(0.0, 0.0, 0.0, 1.0)))
        },
    )
    .finish()
}
