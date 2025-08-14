use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum NameInstruction {
    Initialize(String),
    Update(String),
}
