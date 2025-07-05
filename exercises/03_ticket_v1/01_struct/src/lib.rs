// `Order` という構造体を定義してください。フィールドは次の 2 つです。
// - `price`: 符号なし整数
// - `quantity`: 符号なし整数
//
// さらに `is_available` というメソッドを実装し、`quantity` が 0 より大きいとき
// `true` を返し、そうでなければ `false` を返すようにします。
struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(&self) -> bool {
        self.quantity > 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
