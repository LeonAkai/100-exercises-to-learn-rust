use std::result;

// `while` ループを使って階乗関数を書き直してください。
pub fn factorial(n: u32) -> u32 {
    // `todo!()` マクロはプレースホルダーで、
    // コンパイラに「後で実装する」ことを伝えて
    // 一時的に型エラーを抑制します。
    // ただし実行時には panic します。
    let mut result = 1;
    let mut n = n;
    while n > 0 {
        result *= n;
        n -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

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
