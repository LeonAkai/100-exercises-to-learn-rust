pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // オーバーフローしてラップアラウンドさせるのではなく、
        // u32 の最大値で頭打ちにする “サチュレーティング乗算” を使う想定
        result = result.saturating_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! が u32::MAX（オーバーフロー前の最大値）になることを確認
        assert_eq!(factorial(20), u32::MAX);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
