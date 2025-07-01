// 👇 以下の行（`///` で始まる行）は **ドキュメンテーションコメント** と呼ばれます。
//    直後のアイテム（ここでは `speed` 関数）にドキュメントを紐づけます。
//    この演習ディレクトリで `cargo doc --open` を実行すると、Rust がこれらのコメントから
//    HTML ドキュメントを生成し、ブラウザで表示してくれます。

/// 旅の出発点と到着点、それに要した時間を与えると、
/// 平均速度を計算して返します。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: テストを通すために正しい値を持つ変数 `distance` を定義してください
    //  `distance` の型注釈は必要でしょうか？その理由も考えてみましょう。
    let distance = end - start;
    // 以下の行は変更しないでください
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
