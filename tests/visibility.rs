
#[macro_use]
extern crate dynamiclayout;

use dynamiclayout::{Vec3, Matrix2};

dynamiclayout!(Layout + Accessor {
    a: Vec3,
    b: f32,
    c: Matrix2
});

#[test]
fn test_visibility() {
    // No need to do anything, just seeing if things are properly exported
}
