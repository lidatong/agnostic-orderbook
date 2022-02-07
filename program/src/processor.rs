use borsh::BorshDeserialize;
use num_traits::FromPrimitive;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::AgnosticOrderbookInstruction;

pub mod cancel_order;
pub mod close_market;
pub mod consume_events;
pub mod create_market;
pub mod new_order;

#[allow(missing_docs)]
pub mod msrm_token {
    use solana_program::declare_id;

    declare_id!("MSRMcoVyrFxnSgo5uXwone5SKcGhT1KEJMFEkMEWf9L");
}

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = FromPrimitive::from_u8(instruction_data[0])
            .ok_or(ProgramError::InvalidInstructionData)?;
        let instruction_data = &instruction_data[1..];
        msg!("Instruction unpacked");

        match instruction {
            AgnosticOrderbookInstruction::CreateMarket => {
                msg!("Instruction: Create Market");
                let accounts = create_market::Accounts::parse(accounts)?;
                let params = create_market::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                create_market::process(program_id, accounts, params)?;
            }
            AgnosticOrderbookInstruction::NewOrder => {
                msg!("Instruction: New Order");
                let accounts = new_order::Accounts::parse(accounts)?;
                let params = new_order::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                new_order::process(program_id, accounts, params)?;
            }
            AgnosticOrderbookInstruction::ConsumeEvents => {
                msg!("Instruction: Consume Events");
                let accounts = consume_events::Accounts::parse(accounts)?;
                let params = consume_events::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                consume_events::process(program_id, accounts, params)?;
            }
            AgnosticOrderbookInstruction::CancelOrder => {
                msg!("Instruction: Cancel Order");
                let accounts = cancel_order::Accounts::parse(accounts)?;
                let params = cancel_order::Params::try_from_slice(instruction_data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                cancel_order::process(program_id, accounts, params)?;
            }
            AgnosticOrderbookInstruction::CloseMarket => {
                msg!("Instruction: Close Market");
                let accounts = close_market::Accounts::parse(accounts)?;
                close_market::process(program_id, accounts, close_market::Params {})?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use {
        super::*,
        assert_matches::*,
        solana_program::instruction::{AccountMeta, Instruction},
        solana_program_test::*,
        solana_sdk::{signature::Signer, transaction::Transaction},
    };

    #[tokio::test]
    async fn test_transaction() -> anyhow::Result<()> {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "agnostic_orderbook",
            agnostic_orderbook::id(),
            processor!(process_instruction),
        )
            .start()
            .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        banks_client.process_transaction(transaction).await?;

        Ok(())
    }
}