use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, env, near_bindgen};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u32,
    
}   
    
#[near_bindgen]
impl Counter {
    
    // Public read-only method: Returns the counter value.
    pub fn get_num(&self) -> u32 {
        return self.val;
    }
    pub fn resetnumber(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }
  
pub fn NewNumber(&mut self, number: u32)   {
        self.val = number;
        env::log_str(&format!("this is new number {}", (self.val)));
    }
}
