use solana_sdk::pubkey::Pubkey;
use plerkle_serialization::account_info_generated::account_info::AccountInfo;
use crate::{
    instruction::InstructionBundle,
    error::BlockbusterError,
};
use crate::programs::ProgramParseResult;


pub trait ParseResult: Sync + Send {
    fn result_type(&self) -> ProgramParseResult;

    fn result(&self) -> &Self where Self: Sized {
        self
    }
}

pub struct NotUsed(());

impl NotUsed {
    pub fn new() -> Self {
        NotUsed(())
    }
}

impl ParseResult for NotUsed {
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::Unknown
    }
}

pub trait ProgramParser: Sync + Send {
    fn key(&self) -> Pubkey;
    fn key_match(&self, key: &Pubkey) -> bool;
    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<dyn ParseResult>, BlockbusterError>;
    fn handle_instruction(&self, bundle: &InstructionBundle) -> Result<Box<dyn ParseResult>, BlockbusterError>;
}
