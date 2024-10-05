use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    system_instruction,
    program::invoke_signed,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto; // Add this line

// Define the program ID (replace with your actual program ID)
solana_program::declare_id!("4oKc82RYVbY4AyVnGmtykmG48XNzJpvdSamfhPmnYpmN");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];
    match instruction {
        0 => initialize_creator_account(accounts, instruction_data[1], program_id),
        1 => process_tip(accounts),
        2 => {
            if instruction_data.len() < 9 {
                return Err(ProgramError::InvalidInstructionData.into());
            }
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            process_withdraw(accounts, amount, program_id)
        },
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initialize_creator_account(accounts: &[AccountInfo], bump: u8, program_id: &Pubkey) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let wallet = next_account_info(accounts_iter)?;
    let creator_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !wallet.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let creator_account_key = creator_account.key;
    let seeds = &[
        b"creator",
        wallet.key.as_ref(),
        &[bump],
    ];

    // Create the account
    let rent = Rent::get()?;
    let space = 1000; // Adjust this based on your creator account data structure
    let lamports = rent.minimum_balance(space);

    invoke_signed(
        &system_instruction::create_account(
            wallet.key,
            creator_account_key,
            lamports,
            space as u64,
            program_id,
        ),
        &[wallet.clone(), creator_account.clone(), system_program.clone()],
        &[seeds],
    )?;

    // Initialize the creator account here
    // For example, you might want to set some data in the account

    msg!("Creator account initialized for wallet: {}", wallet.key);
    Ok(())
}

fn process_tip(accounts: &[AccountInfo]) -> ProgramResult {
    // Existing tip processing logic
    msg!("Processing tip");
    Ok(())
}

fn process_withdraw(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let signer = next_account_info(accounts_iter)?; // Connected wallet (signer)
    let creator_account = next_account_info(accounts_iter)?; // Creator account PDA
    let system_program = next_account_info(accounts_iter)?; // System Program

    msg!("Signer: {}", signer.key);
    msg!("Creator account: {}", creator_account.key);
    msg!("System program: {}", system_program.key);
    msg!("Amount: {}", amount);

    // Verify the signer
    if !signer.is_signer {
        msg!("Signer is not a signer");
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    // Derive the expected PDA and check it matches the provided creator account
    let (expected_pda, bump) = Pubkey::find_program_address(
        &[b"creator", signer.key.as_ref()],
        program_id
    );
    msg!("Expected PDA: {}", expected_pda);
    msg!("Bump: {}", bump);

    if expected_pda != *creator_account.key {
        msg!("Invalid creator account PDA");
        return Err(ProgramError::InvalidAccountData.into());
    }

    // Check if the creator account has enough balance
    if creator_account.lamports() < amount {
        msg!("Insufficient funds in creator account");
        return Err(ProgramError::InsufficientFunds.into());
    }

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(
        creator_account.key,
        signer.key,
        amount
    );

    // Perform the withdrawal using invoke_signed
    invoke_signed(
        &transfer_instruction,
        &[creator_account.clone(), signer.clone(), system_program.clone()],
        &[&[b"creator", signer.key.as_ref(), &[bump]]]
    )?;

    msg!("Withdrawal successful: {} lamports", amount);
    Ok(())
}
