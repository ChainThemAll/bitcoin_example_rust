use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct HashValue {
    pub hash: [u8; 32],
}

#[test]
fn test_Default() {
    let hash = HashValue::default();
    assert_eq!(
        HashValue {
            hash: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ]
        },
        hash
    )
}
