use self::{
    aisle::{aisle0, aisle1, aisle2},
    jamming::jamming1,
    planet::{square_planet, strange_gravity},
    sample::sample_stage,
    structs::StageInfo,
    tutorial_stage::{
        fan_tutorial, gravity_tutorial, magnet_tutorial, switch_tutorial, tutorial1, tutorial2,
    },
};
use itertools::Itertools;

pub mod structs;

mod aisle;
mod debug;
mod field_blocks;
mod jamming;
mod planet;
mod sample;
mod tutorial_stage;

type GenerateStageInfoFunc = fn() -> StageInfo;

fn stage_vec() -> Vec<GenerateStageInfoFunc> {
    vec![
        sample_stage,
        tutorial1,
        tutorial2,
        fan_tutorial,
        magnet_tutorial,
        switch_tutorial,
        gravity_tutorial,
        aisle0,
        aisle1,
        aisle2,
        jamming1,
        // debug_stage,
        strange_gravity,
        square_planet,
    ]
}

// NOTE: ステージ名をキーにした辞書形式で持つほうが楽だし自然かもしれない
pub fn stage_title_vec() -> Vec<&'static str> {
    stage_vec()
        .into_iter()
        .map(|generator| generator().stage_title)
        .collect_vec()
}

pub fn select_stage(idx: usize) -> StageInfo {
    stage_vec()[idx]()
}
