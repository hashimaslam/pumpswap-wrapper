use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, instruction::{AccountMeta, Instruction}, system_instruction};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("9uhAD6LjbwbtajGPLqzGwpqZgGbmCYenkwmkLSWp2oRX");

#[program]
pub mod multi_wallet_swap {
    use super::*;

    pub fn multi_wallet_swap(
        ctx: Context<MultiWalletSwap>,
        swaps: Vec<SwapInput>,
        fee_bps: u16,
    ) -> Result<()> {
        for swap in swaps.iter() {
            let fee = swap.max_quote_amount_in * fee_bps as u64 / 10_000;
            let amount_to_swap = swap.max_quote_amount_in - fee;

            // Transfer fee (assumes SOL for now)
            invoke(
                &system_instruction::transfer(
                    &ctx.accounts.user.key(),
                    &ctx.accounts.fee_receiver.key(),
                    fee,
                ),
                &[ctx.accounts.user.to_account_info(), ctx.accounts.fee_receiver.to_account_info()],
            )?;

            // CPI to PumpSwap `buy`
            let mut accounts = vec![
                AccountMeta::new(ctx.accounts.pool.key(), false),
                AccountMeta::new(ctx.accounts.user.key(), true),
                AccountMeta::new_readonly(ctx.accounts.global_config.key(), false),
                AccountMeta::new_readonly(ctx.accounts.base_mint.key(), false),
                AccountMeta::new_readonly(ctx.accounts.quote_mint.key(), false),
                AccountMeta::new(ctx.accounts.user_base_account.key(), false),
                AccountMeta::new(ctx.accounts.user_quote_account.key(), false),
                AccountMeta::new(ctx.accounts.pool_base_account.key(), false),
                AccountMeta::new(ctx.accounts.pool_quote_account.key(), false),
                AccountMeta::new(ctx.accounts.protocol_fee_recipient.key(), false),
                AccountMeta::new(ctx.accounts.protocol_fee_recipient_token_account.key(), false),
                AccountMeta::new(ctx.accounts.quote_token_program.key(), false),
                AccountMeta::new(ctx.accounts.base_token_program.key(), false),
                AccountMeta::new(ctx.accounts.event_authority.key(), false),
                AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
                AccountMeta::new(ctx.accounts.pump_swap_program.key(), false),
                AccountMeta::new(anchor_lang::solana_program::system_program::id(), false),
            ];

            let ix_data = {
                let mut data = vec![];
                data.extend_from_slice(&anchor_lang::solana_program::hash::hash("global:buy".as_bytes()).to_bytes()[..8]);
                data.extend_from_slice(&swap.base_amount_out.to_le_bytes());
                data.extend_from_slice(&amount_to_swap.to_le_bytes());
                data
            };

            let ix = Instruction {
                program_id: ctx.accounts.pump_swap_program.key(),
                accounts,
                data: ix_data,
            };

            invoke(
                &ix,
                &[
                    ctx.accounts.pool.to_account_info(),
                    ctx.accounts.user.to_account_info(),
                    ctx.accounts.global_config.to_account_info(),
                    ctx.accounts.base_mint.to_account_info(),
                    ctx.accounts.quote_mint.to_account_info(),
                    ctx.accounts.user_base_account.to_account_info(),
                    ctx.accounts.user_quote_account.to_account_info(),
                    ctx.accounts.pool_base_account.to_account_info(),
                    ctx.accounts.pool_quote_account.to_account_info(),
                    ctx.accounts.protocol_fee_recipient.to_account_info(),
                    ctx.accounts.protocol_fee_recipient_token_account.to_account_info(),
                    ctx.accounts.quote_token_program.to_account_info(),
                    ctx.accounts.base_token_program.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                    ctx.accounts.associated_token_program.to_account_info(),
                    ctx.accounts.event_authority.to_account_info(),
                    ctx.accounts.pump_swap_program.to_account_info()
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapInput {
    pub base_amount_out: u64,
    pub max_quote_amount_in: u64,
}

#[derive(Accounts)]
pub struct MultiWalletSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    /// CHECK:
    pub fee_receiver: AccountInfo<'info>,
    // PumpSwap accounts
    /// CHECK:
    pub pool: AccountInfo<'info>,
    /// CHECK:
    pub user_base_account: AccountInfo<'info>,
    /// CHECK:
    pub user_quote_account: AccountInfo<'info>,
    /// CHECK:
    pub pool_base_account: AccountInfo<'info>,
    /// CHECK:
    pub pool_quote_account: AccountInfo<'info>,
    /// CHECK:
    pub protocol_fee_recipient: AccountInfo<'info>,
    /// CHECK:
    pub quote_token_program: AccountInfo<'info>,
    /// CHECK:
    pub event_authority: AccountInfo<'info>,
    /// CHECK:
    pub global_config: AccountInfo<'info>,
    /// CHECK:
    pub base_mint: AccountInfo<'info>,
    /// CHECK:
    pub quote_mint: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub protocol_fee_recipient_token_account: AccountInfo<'info>,
    /// CHECK:
    pub base_token_program: AccountInfo<'info>,
    /// CHECK:
    pub associated_token_program: AccountInfo<'info>,
    #[account(executable)]
    /// CHECK:
    pub pump_swap_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
