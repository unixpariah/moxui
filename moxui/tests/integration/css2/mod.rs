use moxui::tree;

use crate::WgpuCtx;

mod absolute_non_replaced_height_001;
mod absolute_non_replaced_height_003;
mod absolute_non_replaced_height_004;

pub fn css2(wgpu_ctx: &WgpuCtx) -> Vec<tree::Tree> {
    vec![
        absolute_non_replaced_height_001::absolute_non_replaced_height_001(wgpu_ctx),
        absolute_non_replaced_height_003::absolute_non_replaced_height_003(wgpu_ctx),
        absolute_non_replaced_height_004::absolute_non_replaced_height_004(wgpu_ctx),
    ]
}
