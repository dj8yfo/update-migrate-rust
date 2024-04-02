use near_sdk::near;

use near_sdk::collections::Vector;
use near_sdk::json_types::{U64, U128};

use near_sdk::{env, AccountId, NearToken, PanicOnDefault};

mod update;

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[near(serializers=[borsh, json])]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct GuestBook {
    messages: Vector<PostedMessage>,
    payments: Vector<NearToken>,
    manager: AccountId,
}

#[near]
impl GuestBook {
    #[init]
    pub fn init(manager: AccountId) -> Self {
        Self {
            messages: Vector::new(b"m"),
            payments: Vector::new(b"p"),
            manager,
        }
    }

    #[payable]
    pub fn add_message(&mut self, text: String) {
        let payment = env::attached_deposit();
        let premium = payment >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            premium,
            sender,
            text,
        };
        self.messages.push(&message);
        self.payments.push(&payment);
    }

    pub fn get_messages(&self, from_index: Option<U128>, limit: Option<U64>) -> Vec<PostedMessage> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(u64::from(limit.unwrap_or(U64::from(10))) as usize)
            .collect()
    }

    pub fn get_payments(&self, from_index: Option<U128>, limit: Option<U64>) -> Vec<U128> {
        let from = u128::from(from_index.unwrap_or(U128(0)));

        self.payments
            .iter()
            .skip(from as usize)
            .take(u64::from(limit.unwrap_or(U64::from(10))) as usize)
            .map(|x| U128(x.as_yoctonear()))
            .collect()
    }
}
