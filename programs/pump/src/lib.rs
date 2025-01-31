use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod constants;
pub mod states;
pub mod errors;

declare_id!("37tkvZheP1dFm3Pfj1uRUHPqFVq9HBpVnNPYFqTQU493");

#[program]
pub mod pump {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

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

#[derive(Accounts)]
pub struct Initialize {}
