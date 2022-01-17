pub use strum::EnumIter;
use strum::IntoEnumIterator;

// BASE FILE

pub struct Layout;

#[derive(EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pipeline {
    Processing,
    Footprint,
    Etc,
}

pub enum StageResult {
    Running,
    Converged,
}

impl Pipeline {
    pub fn step_functions(&self, layout: &mut Layout) -> StageResult {
        match self {
            Pipeline::Processing => processing_fun(layout),
            Pipeline::Footprint => footprint_fun(layout),
            Pipeline::Etc => etc_fun(layout),
        }
    }

    pub fn run(layout: &mut Layout) {
        for stage in Self::iter() {
            while let StageResult::Running = stage.step_functions(layout) {}
        }
    }
}

pub fn processing_fun(_input: &mut Layout) -> StageResult {
    todo!()
}

pub fn footprint_fun(_input: &mut Layout) -> StageResult {
    todo!()
}

pub fn etc_fun(_input: &mut Layout) -> StageResult {
    todo!()
}
