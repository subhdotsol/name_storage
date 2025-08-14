use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
    program_error::ProgramError, pubkey::Pubkey, system_instruction,
};

// Instructions enum
#[derive(BorshSerialize, BorshDeserialize)]
pub enum NameInstruction {
    Initialize(String),
    Update(String),
}

// Data stored in the account
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Data {
    pub name: String,
}

// Program logic
pub fn process_instruction(
    program_id: &Pubkey,      // Program ID
    accounts: &[AccountInfo], // Accounts passed to the program
    instruction_data: &[u8],  // Serialized instruction
) -> ProgramResult {
    let [payer, name_account, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    match NameInstruction::try_from_slice(instruction_data)? {
        NameInstruction::Initialize(name) => {
            msg!("Initializing name account with name: {}", name);

            // You should ideally calculate rent-exempt lamports and correct data size here
            let create_ix = system_instruction::create_account(
                payer.key,
                name_account.key,
                1_000_000_000, // Example lamports
                100,           // Example size in bytes (adjust as needed)
                program_id,
            );

            invoke(
                &create_ix,
                &[
                    payer.clone(),
                    name_account.clone(),
                    system_program_info.clone(),
                ],
            )?;

            let name_account_data = Data { name };
            name_account_data.serialize(&mut *name_account.data.borrow_mut())?;
        }

        NameInstruction::Update(name) => {
            msg!("Updating name account to: {}", name);

            let mut name_account_data = Data::try_from_slice(&name_account.data.borrow())?;
            name_account_data.name = name;
            name_account_data.serialize(&mut *name_account.data.borrow_mut())?;
        }
    }

    Ok(())
}
