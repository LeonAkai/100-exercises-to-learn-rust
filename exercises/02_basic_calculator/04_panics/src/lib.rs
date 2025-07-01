/// 旅の出発点と到着点、それに要した時間を与えると、
/// 旅の平均速度を計算して返します。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: `time_elapsed` が 0 の場合はカスタムメッセージで panic させる
    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }
    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 `#[should_panic]` アノテーションを付けると、テスト対象コードが
    //    panic することを期待していると示せます。`expected` を使えば
    //    panic メッセージの内容も検証できます。
    //    これらは Rust 組み込みのテストフレームワークの機能です！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
