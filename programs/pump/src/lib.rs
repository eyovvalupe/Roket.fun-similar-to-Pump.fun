#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;

mod instructions;
mod constants;
mod states;
mod errors;

declare_id!("D2hhTiPavX5C256F7aTc48ntHJhSoRQhVj9oUtUwbvsA");

#[program]
pub mod pump {
    use super::*;

    pub fn create_amm(ctx: Context<CreateAmm>, id: Pubkey) -> Result<()> {
        instructions::create_amm(ctx, id)
    }

    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    pub fn create_token_mint(ctx: Context<CreateTokenMint>, token_name: String, token_symbol: String, token_uri: String) -> Result<()> {
        instructions::create_token_mint(ctx, token_name, token_symbol, token_uri)
    }

    pub fn proxy_initialize(
        ctx: Context<ProxyInitialize>,
        nonce: u8,
        open_time: u64,
        init_pc_amount: u64,
        init_coin_amount: u64,
    ) -> Result<()> {
        instructions::initialize(ctx, nonce, open_time, init_pc_amount, init_coin_amount)
    }
}
