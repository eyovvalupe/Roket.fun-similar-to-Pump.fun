use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Amm {
    ///CHECK: safe
    pub id: Pubkey,
    ///CHECK: safe
    pub admin: Pubkey,
    ///CHECK: safe
    pub fee: u16,
    ///CHECK: safe
    pub lock: bool,
}

impl Amm {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 2;
}

#[account]
#[derive(Default)]
pub struct Pool {
    ///CHECK: safe
    pub amm: Pubkey,
    ///CHECK: safe
    pub mint_a: Pubkey,
}

impl Pool {
    pub const LEN: usize = 8 + 32 + 32;
}