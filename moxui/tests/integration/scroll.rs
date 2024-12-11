use crate::WgpuCtx;
use calc_units::Units;
use moxui::tree;

pub fn scroll(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    moxui::tree::Tree::new(&wgpu_ctx.device, &wgpu_ctx.surface_config, |surface| {
        surface
            .set_background_color(0.0, 0.0, 0.0, 0.0)
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::InlineBlock)
                    .set_background_color(1.0, 0.0, 0.0, 1.0)
                    .set_size(Some(Units::Px(50.0)), Some(Units::Perc(120.0)))
            })
            .add_child(|item| {
                item.set_display(moxui::rectangle::Display::InlineBlock)
                    .set_background_color(0.0, 0.0, 1.0, 1.0)
                    .set_size(Some(Units::Perc(120.0)), Some(Units::Px(50.0)))
            })
    })
    .finish()
}
