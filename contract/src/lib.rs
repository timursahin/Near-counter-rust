use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};
 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {    counter: i128     }

impl Default for Counter {
    fn default() -> Self {
        Self { counter: 3 }
    }
}

#[near_bindgen]
impl Counter {
    pub fn get_counter(&self) -> i128 {
        return self.counter.clone();
    }

    pub fn set_counter(&mut self, new_counter: i128) {
        log!("Saving new counter {} ", new_counter);
        self.counter = new_counter;
    }
}