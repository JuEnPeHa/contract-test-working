// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen,log};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Counter {
    value: u64,
}

#[near_bindgen]
impl Counter {
    #[init]
    pub fn new(value: u64) -> Self {
        log!("Custom counter initialization!");
        Self { value }
    }
    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn get(&self) -> u64 {
        self.value
    }
    pub fn set(&mut self, value: u64) {
        self.value = value;
    }
}