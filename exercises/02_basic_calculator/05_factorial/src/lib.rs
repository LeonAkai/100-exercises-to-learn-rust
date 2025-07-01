// 非負整数 `n` を受け取り、`n!`（n の階乗）を返す
// 関数 `factorial` を定義してください。
//
// 階乗 `n!` は 1 から `n` までの正の整数をすべて掛け合わせた積です。
// 例: `5!`（ファイブ・ファクトリアル）は `5 * 4 * 3 * 2 * 1` で `120` になります。
// `0!` は特別に `1` と定義されています。
//
// したがって `factorial(0)` は `1`、`factorial(1)` は `1`、
// `factorial(2)` は `2` となります。
//
// まだループは学習していないので、ここでは **再帰** を使って実装しましょう！
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
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
