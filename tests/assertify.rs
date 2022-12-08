use assertify::assertify;

#[test]
fn assertify_simple_expr() {
    assertify!(1 - 2 == -1);
}

#[test]
#[should_panic(
    expected = "failed: 1 + 2 == 0\n  actual:      3\n  expected: == 0\n"
)]
fn fail_simple_eq() {
    assertify!(1 + 2 == 0);
}

#[test]
#[should_panic(expected = "failed: false")]
fn fail_simple_literal() {
    assertify!(false);
}

#[test]
#[should_panic(
    expected = "failed: 1 + 2 != 3\n  actual:      3\n  expected: != 3\n"
)]
fn fail_simple_ne() {
    assertify!(1 + 2 != 3);
}

#[test]
#[should_panic(
    expected = "failed: 1 + 2 > 4\n  actual:     3\n  expected: > 4\n"
)]
fn fail_simple_gt() {
    assertify!(1 + 2 > 4);
}

fn result(good: bool) -> Result<(), &'static str> {
    if good {
        Ok(())
    } else {
        Err("bad")
    }
}

#[test]
#[should_panic(
    expected = "failed: result(false) == Ok(())\n  actual:      Err(\"bad\")\n  expected: == Ok(())\n"
)]
fn fail_result_ok() {
    assertify!(result(false) == Ok(()));
}
