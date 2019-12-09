use assertify::assertify;

fn main() {
    assertify!(1 + 2);
    unreachable!();
}
