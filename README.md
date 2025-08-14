# Name Storage Program (Native Solana)

A basic, low-level Solana program written using Rust and the native Solana SDK (no Anchor) to demonstrate how to:

- Create a system account manually on-chain
- Store custom structured data (`String`) using Borsh serialization
- Update that data later via new instructions

---

## Overview

This program allows users to:

- Initialize a new Solana account with a user-defined name (`String`)
- Update the name stored in that account

The program is built using **native Solana development practices**, without Anchor or program-derived addresses (PDAs). All serialization is handled using **Borsh**, a performant binary serialization library.

---

## Features

-  Native Solana program (no framework dependency)
-  Creates and writes to on-chain accounts directly
-  Supports updating stored state
-  Uses Borsh for structured serialization

---

## Program Structure

### Instruction Enum

```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub enum NameInstruction {
    Initialize(String), // Creates a new account and stores a name
    Update(String),     // Updates an existing account's name
}
```

### Data structure
```rust
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Data {
    pub name: String,
}
```

### Instruction Processor
```rust
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [payer, name_account, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    match NameInstruction::try_from_slice(instruction_data)? {
        NameInstruction::Initialize(name) => {
            // 1. Create a new account
            // 2. Allocate space (100 bytes here, adjust as needed)
            // 3. Fund with enough lamports (1 SOL here)
            // 4. Serialize and store the name
        }
        NameInstruction::Update(name) => {
            // 1. Deserialize existing data
            // 2. Overwrite name field
            // 3. Reserialize updated struct
        }
    }

    Ok(())
}
```

