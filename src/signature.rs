use serde::{Deserialize, Serialize};

use crate::transaction::Transaction;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Signature;
impl Signature {
    fn signature_tx(&self, tx: Transaction) -> Self {
        todo!()
    }
    fn verify_tx(&self, tx: Transaction) {}
}
