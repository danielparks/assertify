#[macro_export]
macro_rules! assertify {
    ($func:ident($($arg:expr),*) == $expected:expr) => {
        let actual = $func($($arg),*);
        if actual != $expected {
            panic!(
                "failed: {}\n  \
                  expected: {:?}\n  \
                  actual:   {:?}\n",
                stringify!($func($($arg),*)), $expected, actual);
        }
    }
}

#[macro_export]
macro_rules! testify {
    ($name:ident, $func:ident($($arg:expr),*) == $expected:expr) => {
        #[test]
        fn $name() {
            assertify!($func($($arg),*) == $expected);
        }
    }
}

#[cfg(test)]
mod tests {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    testify!(add_pos, add(1, 2) == 3);
    testify!(add_neg, add(-1, 2) == 1);
    testify!(add_all_expressions, add(add(1, 1), 5 - 3) == 2 + 5 - 3);

    fn concat(a: &str, b: &str) -> String {
        let mut s = String::with_capacity(a.len() + b.len());
        s.push_str(a);
        s.push_str(b);
        s
    }

    testify!(concat_literal, concat("a", "b") == "ab");

    fn concat_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(a.len() + b.len());
        v.extend_from_slice(a);
        v.extend_from_slice(b);
        v
    }

    testify!(concat_bytes_literals, concat_bytes(b"a", b"b") == b"ab");
}
