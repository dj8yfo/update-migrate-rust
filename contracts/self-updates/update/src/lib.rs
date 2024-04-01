use near_sdk::near;

use near_sdk::collections::Vector;
use near_sdk::json_types::U128;

use near_sdk::{env, AccountId, NearToken, PanicOnDefault};

mod migrate;
mod update;

const POINT_ONE: NearToken = NearToken::from_yoctonear(100_000_000_000_000_000_000_000);

#[near(serializers = [borsh, json])]
pub struct PostedMessage {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct GuestBook {
    messages: Vector<PostedMessage>,
    manager: AccountId,
}

#[near]
impl GuestBook {
    #[init]
    pub fn init(manager: AccountId) -> Self {
        Self {
            messages: Vector::new(b"m"),
            manager,
        }
    }

    #[payable]
    pub fn add_message(&mut self, text: String) {
        let payment = env::attached_deposit();
        let sender = env::predecessor_account_id();
        let premium = payment >= POINT_ONE;
        let message = PostedMessage {
            payment,
            sender,
            premium,
            text,
        };
        self.messages.push(&message);
    }

    pub fn get_messages(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<PostedMessage> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit.unwrap_or(10) as usize)
            .collect()
    }
}
