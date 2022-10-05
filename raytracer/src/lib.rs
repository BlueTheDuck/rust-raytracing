extern crate nalgebra as na;

const MIN_T: f64 = 0.001;

macro_rules! float {
    ($t:ident -> 0) => {
        $t.abs() < $crate::MIN_T
    };
    ($t:ident > 0) => {
        $t > -$crate::MIN_T
    };
    ($t:ident < 0) => {
        $t < $crate::MIN_T
    };
}

pub mod color;
pub mod scene;
pub mod shapes;
