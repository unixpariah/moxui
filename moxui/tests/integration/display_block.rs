use crate::WgpuCtx;
use calc_macro::calc;
use calc_units::Units;
use moxui::tree;

pub fn display_block(wgpu_ctx: &WgpuCtx) -> tree::Tree {
    moxui::tree::Tree::new(&wgpu_ctx.device, &wgpu_ctx.surface_config, |surface| {
        surface
            .set_background_color(0.0, 0.0, 0.0, 0.0)
            .add_child(|item| {
                item.set_background_color(0.0, 0.0, 1.0, 1.0)
                    .add_child(|item| {
                        item.set_background_color(0.0, 1.0, 0.0, 1.0)
                            .set_margin(
                                Units::Px(50.0),
                                Units::Px(0.0),
                                Units::Px(0.0),
                                calc!("50%" - 25px),
                            )
                            .set_size(Some(Units::Px(50.0)), Some(Units::Px(50.0)))
                    })
            })
            .add_child(|item| {
                item.set_background_color(1.0, 0.0, 0.0, 1.0)
                    .set_size(None, Some(Units::Px(50.0)))
            })
            .add_child(|item| item.set_background_color(0.0, 0.0, 1.0, 1.0))
            .add_child(|item| {
                item.set_background_color(0.0, 1.0, 0.0, 1.0)
                    .set_size(Some(Units::Perc(50.0)), Some(Units::Px(50.0)))
            })
            .add_child(|item| {
                item.set_background_color(1.0, 0.0, 0.0, 1.0)
                    .set_size(Some(Units::Perc(50.0)), Some(Units::Px(50.0)))
                    .set_margin(
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(50.0),
                        Units::Px(0.0),
                    )
            })
            .add_child(|item| {
                item.set_background_color(0.5, 0.5, 0.0, 1.0)
                    .set_margin(
                        Units::Px(25.0),
                        Units::Px(0.0),
                        Units::Px(0.0),
                        Units::Px(0.0),
                    )
                    .add_child(|item| {
                        item.set_background_color(0.0, 1.0, 0.0, 1.0)
                            .set_size(Some(Units::Px(50.0)), Some(Units::Px(50.0)))
                    })
                    .add_child(|item| {
                        item.set_background_color(0.0, 0.0, 1.0, 1.0)
                            .set_size(Some(Units::Px(50.0)), Some(Units::Px(50.0)))
                    })
            })
    })
    .finish()
}
