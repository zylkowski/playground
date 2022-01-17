// UI FILE
// use enum_dispatch::enum_dispatch;
use playground_2::*;

// #[enum_dispatch(Stage)]
pub trait Draw {
    fn draw(&self);
}

impl Draw for Stage {
    fn draw(&self) {
        match self {
            Stage::StageA(_) => todo!(),
            Stage::StageB(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    // use playground_2::*;

    // use crate::*;

    #[test]
    fn it_works() {
        // let a: Stage = StageA {}.into();

        // a.draw();
    }
}
