// オーバーフローしたときにラップアラウンドするよう、`dev` プロファイルをカスタマイズしてください。
// 正しい記法は Cargo のドキュメントで確認できます:
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// このカスタマイズは演習の `Cargo.toml` ではなく、リポジトリのルートにある
// `Cargo.toml` へ記述する必要があります（理由は後ほど説明します）。

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! は 2432902008176640000 で、u32 には大きすぎます。
        // デフォルトの dev プロファイルでは `cargo test` 実行時に panic しますが、
        // ここではラップアラウンドを期待します。
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // 読みやすさのためにアンダースコアで区切った大きな数値リテラル！
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
