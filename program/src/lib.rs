use crate::{error::Crows, instruction::RoosterCommand};
use borsh::{BorshDeserialize, BorshSerialize};
use mpl_token_metadata::instruction::{
    builders::TransferBuilder, InstructionBuilder, TransferArgs,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    program_memory::sol_memcpy,
    pubkey,
    pubkey::Pubkey,
};

pub mod assertions;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;

pub use mpl_token_metadata::{processor::AuthorizationData, state::TokenDelegateRole};

solana_program::declare_id!("roos9SDjRQhy5iq8gVihwBoVvgYcNDxqhj1HdhGpiu5");

pub const SPL_TOKEN_PROGRAM_ID: Pubkey = pubkey!("Token4Q2B47VCdUy8u3rSTMMk2bGA1k7eN8qfKSzdiM");
pub const SPL_ATA_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATAccPjxdgWfJKKN4PmfJ55FbEDEwD8zJUwVjuL9MuHy");
pub const MPL_TOKEN_AUTH_RULES_PROGRAM_ID: Pubkey =
    pubkey!("AuthxYNhPnnrGBo1wdzeUdukrsFpHvR42wghx8ZPNEo4");
