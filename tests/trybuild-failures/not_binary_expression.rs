#![allow(deprecated)]

use assertify::assertify;

fn main() {
    assertify!("foo");
    unreachable!();
}
