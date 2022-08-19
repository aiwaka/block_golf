use self::{
    aisle::{aisle0, aisle1, aisle2},
    jamming::jamming1,
    planet::square_planet,
    structs::StageInfo,
    tutorial_stage::{fan_tutorial, switch_tutorial, tutorial1, tutorial2},
};

pub mod structs;

mod aisle;
mod debug;
mod field_blocks;
mod jamming;
mod planet;
mod sample;
mod tutorial_stage;

use debug::debug_stage;
use itertools::Itertools;
use sample::sample_stage;

type GenerateStageInfoFunc = fn() -> StageInfo;

fn stage_vec() -> Vec<GenerateStageInfoFunc> {
    vec![
        tutorial1,
        tutorial2,
        fan_tutorial,
        switch_tutorial,
        sample_stage,
        aisle0,
        aisle1,
        aisle2,
        jamming1,
        debug_stage,
        square_planet,
    ]
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
