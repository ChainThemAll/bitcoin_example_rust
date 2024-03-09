use serde::{Deserialize, Serialize};

/// 交易
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,         // 交易ID
    vin: Vec<TXInput>,   // 输入
    vout: Vec<TXOutput>, // 输出
}

impl Transaction {
    pub fn new(
        id: Vec<u8>,
        vin: Vec<TXInput>, // 输入
        vout: Vec<TXOutput>,
    ) -> Self {
        Self {
            id,   // 交易ID
            vin,  // 输入
            vout, // 输出
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TXInput(pub String);
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TXOutput(pub String);

#[test]
fn test_Default() {
    let txs = Transaction::default();
    println!("{:?}", txs);
}
