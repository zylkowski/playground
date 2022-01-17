// BASE FILE
use enum_dispatch::enum_dispatch;

pub struct StageA;
pub struct StageB;

#[enum_dispatch]
pub enum Stage {
    StageA,
    StageB,
}

#[enum_dispatch(Stage)]
pub trait Run {
    fn run(&self);
}

impl Run for StageA {
    fn run(&self) {
        print!("Stage A run");
    }
}

impl Run for StageB {
    fn run(&self) {
        print!("Stage B run");
    }
}

#[cfg(test)]
mod tests {
    // use crate::*;

    #[test]
    fn it_works() {
        // let a: Stage = StageA {}.into();

        // a.run();
    }
}
