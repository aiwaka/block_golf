use self::{structs::StageInfo, tutorial_stage::tutorial_stage1};

pub mod debug;
mod field_blocks;
pub mod sample;
pub mod structs;
pub mod tutorial_stage;

use debug::debug_stage;
use itertools::Itertools;
use sample::sample_stage;

type GenerateStageInfoFunc = fn() -> StageInfo;

fn stage_vec() -> Vec<GenerateStageInfoFunc> {
    vec![tutorial_stage1, debug_stage, sample_stage]
}

pub fn stage_title_vec() -> Vec<&'static str> {
    stage_vec()
        .into_iter()
        .map(|generator| generator().stage_title)
        .collect_vec()
}

pub fn select_stage(idx: usize) -> StageInfo {
    stage_vec()[idx]()
}
