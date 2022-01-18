use std::collections::HashMap;

// UI FILE
// use enum_dispatch::enum_dispatch;
use playground_2::*;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct Painter;

#[derive(Default)]
pub struct LabStageState {
    is_animated: bool,
    painter: Painter,
    //etc
}

pub struct LabPipeline {
    pub pipeline_states: HashMap<Pipeline, LabStageState>,
}

impl LabPipeline {
    pub fn new() -> LabPipeline {
        Self {
            pipeline_states: playground_2::Pipeline::iter()
                .map(|stage| (stage, Self::lab_stage_state(&stage)))
                .collect(),
        }
    }

    pub fn lab_stage_state(stage: &Pipeline) -> LabStageState {
        match stage {
            //Write LabStageState only for stages that are not default
            Pipeline::Etc => LabStageState {
                is_animated: false,
                painter: Painter,
            },
            _ => Default::default(),
        }
    }
}
