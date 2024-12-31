use moxui::tree;

use crate::WgpuCtx;

mod absolute_non_replaced_height_001;
mod absolute_non_replaced_height_003;
mod absolute_non_replaced_height_004;
mod absolute_non_replaced_height_005;
mod absolute_non_replaced_height_006;
mod absolute_non_replaced_height_007;
mod absolute_non_replaced_height_008;
mod absolute_non_replaced_height_009;
mod absolute_non_replaced_height_010;
mod absolute_non_replaced_height_011;
mod absolute_non_replaced_height_012;
mod absolute_non_replaced_height_013;

mod absolute_non_replaced_width_001;

pub fn css2(wgpu_ctx: &WgpuCtx) -> Vec<tree::Tree> {
    vec![
        absolute_non_replaced_height_001::absolute_non_replaced_height_001(wgpu_ctx),
        absolute_non_replaced_height_003::absolute_non_replaced_height_003(wgpu_ctx),
        absolute_non_replaced_height_004::absolute_non_replaced_height_004(wgpu_ctx),
        absolute_non_replaced_height_005::absolute_non_replaced_height_005(wgpu_ctx),
        absolute_non_replaced_height_006::absolute_non_replaced_height_006(wgpu_ctx),
        absolute_non_replaced_height_007::absolute_non_replaced_height_007(wgpu_ctx),
        absolute_non_replaced_height_008::absolute_non_replaced_height_008(wgpu_ctx),
        absolute_non_replaced_height_009::absolute_non_replaced_height_009(wgpu_ctx),
        absolute_non_replaced_height_010::absolute_non_replaced_height_010(wgpu_ctx),
        absolute_non_replaced_height_011::absolute_non_replaced_height_011(wgpu_ctx),
        absolute_non_replaced_height_012::absolute_non_replaced_height_012(wgpu_ctx),
        absolute_non_replaced_height_013::absolute_non_replaced_height_013(wgpu_ctx),
        //absolute_non_replaced_width_001::absolute_non_replaced_width_001(wgpu_ctx),
    ]
}
