#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use instructions::*;

mod instructions;
mod constants;
mod states;
mod errors;

declare_id!("BR1gGg7joMV6PxKMgsszrSydXJAPMyk1KQC7wtx8Vz7M");

#[program]
pub mod pump {
    use super::*;

    pub fn create_amm(ctx: Context<CreateAmm>, id: Pubkey) -> Result<()> {
        instructions::create_amm(ctx, id)
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
