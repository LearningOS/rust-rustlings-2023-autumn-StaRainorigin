// errors2.rs
//
//假设我们正在编写一个游戏，您可以在其中使用代币购买物品。所有物品花费 5 个代币，
//每当您购买物品时，都会产生 1 个代币的处理费。
//游戏玩家将输入他们想要购买的物品数量，
//`total_cost` 函数将计算代币的总成本。
//不过，由于玩家输入了数量，我们将其作为字符串获取--
//他们可能会输入任何内容，而不仅仅是数字！
//
//现在，这个函数根本不处理错误情况
//（并且也没有正确处理成功案例）。我们想做的是：
//如果我们对非数字的字符串调用“parse”函数，
//该函数将返回一个 `ParseIntError`，在这种情况下，
//我们希望立即从函数中返回该错误，而不是尝试进行乘法和加法。
//
//至少有两种方法可以实现这一点，而且都是正确的——但其中一种要短得多！
//
//
//执行 `rusdlingshinterrors2` 或使用 `hint`watch 子命令来获取提示。
// 


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();
    
    match qty {
        Ok(qty) => Ok(qty * cost_per_item + processing_fee),
        Err(e) => Err(e),
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
