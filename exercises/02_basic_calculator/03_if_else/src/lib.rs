/// `n` が偶数の場合は `12` を返し、
/// `n` が `3` で割り切れる場合は `13` を返し、
/// それ以外の場合は `17` を返します。
fn magic_number(n: u32) -> u32 {
    if n % 2 == 0 {
        12
    } else if n % 3 == 0 {
        13
    } else {
        17
    }
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
