use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey, pubkey::Pubkey};

pub mod instructions;
pub mod state;

declare_id!("AsEUn846nbfmDkvzfEDzgZoMiJ4WBDzmRSBYtqQqYuvZ");

use instructions::*;
use state::TradeType;

#[constant]
pub const MPL_TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

#[program]
pub mod escrow {
    use super::*;

    pub fn create_trade(
        ctx: Context<CreateTrade>,
        sol_amount: u64,
        spl_amount: u64,
        trade_type: TradeType
    ) -> Result<()> {
        instructions::create_trade_handler(ctx, sol_amount, spl_amount, trade_type)
    }

    pub fn cancel_trade(ctx: Context<CancelTrade>) -> Result<()> {
        instructions::cancel_trade_handler(ctx)
    }

    pub fn accept_trade(
        ctx: Context<AcceptTrade>,
        sol_amount: u64,
        spl_amount: u64,
        trade_type: TradeType
    ) -> Result<()> {
        instructions::accept_trade_handler(ctx, sol_amount, spl_amount, trade_type)
    }

    pub fn execute_trade(ctx: Context<ExecuteTrade>) -> Result<()> {
        instructions::execute_trade_handler(ctx)
    }
}

#[error_code]
pub enum Errors {
    #[msg("Trade amount can't be zero")]
    TokenAmountZero,

    #[msg("The inactive token must be zero")]
    AmountNotZero,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("SPL Token Account is not provided")]
    AccountNotProvided,

    #[msg("the Escrow Account is not required in this trade")]
    AccountNotRequired,

    #[msg("The signer is not a party to the trade")]
    NotTradeParty, 

    #[msg("The trade is already accepted by the party two")]
    TradeAlreadyAccepted,

    #[msg("The trade is not yet accepted by the party two")]
    TradeNotAccepted,

    #[msg("The mint account doesn't exist in the trade")]
    MintNotExist,

    #[msg("The account details doesn't match with the trade")]
    IncorrectTokenAccount,

    #[msg("The provided account is not initialized")]
    AccountNotInitialized,

    #[msg("The signer is not the owner of the token account")]
    InvalidOwner
}
