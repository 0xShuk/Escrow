use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    TokenAccount,
    Transfer,
    CloseAccount,
    Token,
    Mint
};
use crate::state::Trade;
use crate::{Errors, ID,};

#[derive(Accounts)]
pub struct CancelTrade<'info> {
    #[account(
        mut,
        has_one = party_one,
        has_one = party_two,
        close = party_one
    )]
    pub trade_details: Box<Account<'info, Trade>>,

    #[account(
        mut,
        seeds = [
            b"escrow-one",
            trade_details.party_one.key().as_ref(),
            trade_details.party_two.key().as_ref(),
        ],
        bump
    )]
    pub escrow_party_one: Option<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            b"escrow-two",
            trade_details.party_one.key().as_ref(),
            trade_details.party_two.key().as_ref(),
        ],
        bump
    )]
    pub escrow_party_two: Option<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = party_one,
        token::mint = one_mint,
        token::authority = party_one,
        address = trade_details.one_send_address.unwrap() @ Errors::IncorrectTokenAccount
    )]
    pub one_send_address: Option<Box<Account<'info, TokenAccount>>>,

    #[account(
        init_if_needed,
        payer = party_one,
        token::mint = two_mint,
        token::authority = party_two,
        address = trade_details.two_send_address.unwrap() @ Errors::IncorrectTokenAccount
    )]
    pub two_send_address: Option<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub party_one: Signer<'info>,

    /// CHECK: This account is customly validated 
    #[account(mut)] 
    pub party_two: AccountInfo<'info>,

    #[account(
        address = trade_details.one_mint.unwrap() @ Errors::MintNotExist
    )]
    pub one_mint: Option<Account<'info,Mint>>,

    #[account(
        address = trade_details.two_mint.unwrap() @ Errors::MintNotExist
    )]
    pub two_mint: Option<Account<'info, Mint>>,

    
    #[account(
        seeds = [b"escrow"],
        bump
    )]
    pub escrow_authority: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CancelTrade<'info> {
    pub fn transfer_spl(&self, party: u8, amount: u64) -> Result<()> {
        let (_escrow_authority, bump) = Pubkey::find_program_address(&[b"escrow"], &ID);
        let escrow_seed = &[&b"escrow"[..], &[bump]];

        let transfer_cpi_program = self.token_program.to_account_info();
        let close_cpi_program = self.token_program.to_account_info();

        let transfer_cpi_accounts = if party == 0 {
            Transfer {
                from: self.escrow_party_one.as_ref().unwrap().to_account_info(),
                to: self.one_send_address.as_ref().unwrap().to_account_info(),
                authority: self.escrow_authority.to_account_info()
            }
        } else {
            Transfer {
                from: self.escrow_party_two.as_ref().unwrap().to_account_info(),
                to: self.two_send_address.as_ref().unwrap().to_account_info(),
                authority: self.escrow_authority.to_account_info()
            }
        };
        
        let close_cpi_accounts = if party == 0 {
            CloseAccount {
                account: self.escrow_party_one.as_ref().unwrap().to_account_info(),
                destination: self.party_one.to_account_info(),
                authority: self.escrow_authority.to_account_info()
            }
        } else {
            CloseAccount {
                account: self.escrow_party_two.as_ref().unwrap().to_account_info(),
                destination: self.party_two.to_account_info(),
                authority: self.escrow_authority.to_account_info()
            }
        };

        let transfer_context = CpiContext::new(transfer_cpi_program, transfer_cpi_accounts);
        let close_context = CpiContext::new(close_cpi_program, close_cpi_accounts);

        token::transfer(
            transfer_context.with_signer(&[&escrow_seed[..]]),
            amount
        )?;

        token::close_account(
            close_context.with_signer(&[&escrow_seed[..]])
        )?;
            
        Ok(())
    }

    pub fn transfer_sol(&self,party: u8,amount: u64) -> Result<()> {
        if party == 0 {
            **self.trade_details.to_account_info().try_borrow_mut_lamports()? -= amount;
            **self.party_one.to_account_info().try_borrow_mut_lamports()? += amount;
        } else {
            **self.trade_details.to_account_info().try_borrow_mut_lamports()? -= amount;
            **self.party_two.to_account_info().try_borrow_mut_lamports()? += amount;  
        };
        
        Ok(())
    }

}

pub fn cancel_trade_handler(ctx: Context<CancelTrade>) -> Result<()> {

    let trade_details = &ctx.accounts.trade_details;

    if trade_details.is_confirmed {
        if trade_details.spl_amount[0] > 0 {
            ctx.accounts.transfer_spl(0, trade_details.spl_amount[0])?;
        }

        if trade_details.spl_amount[1] > 0 {
            ctx.accounts.transfer_spl(1, trade_details.spl_amount[1])?;

        }
        
        if trade_details.sol_amount[0] > 0 {
            ctx.accounts.transfer_sol(0, trade_details.sol_amount[0])?;
        }

        if trade_details.sol_amount[1] > 0 {
            ctx.accounts.transfer_sol(1, trade_details.sol_amount[1])?;
        }
    } else {
        if trade_details.spl_amount[0] > 0 {
            ctx.accounts.transfer_spl(0, trade_details.spl_amount[0])?;
        }

        if trade_details.sol_amount[0] > 0 {
            ctx.accounts.transfer_sol(0, trade_details.sol_amount[0])?;
        }
    }

    Ok(())
}
