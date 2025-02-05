use amm_anchor::Initialize2;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[derive(Accounts, Clone)]
pub struct ProxyInitialize<'info> {
    ///CHECK: Safe
    pub amm_program: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account()]
    pub amm_authority: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub amm_lp_mint: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(
        owner = token_program.key()
    )]
    pub amm_coin_mint: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(
        owner = token_program.key()
    )]
    pub amm_pc_mint: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Account. Must be non zero, owned by $authority
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account()]
    pub amm_config: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub create_fee_destination: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(
        address = amm_anchor::openbook_program_id::id(),
    )]
    pub market_program: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(
        owner = market_program.key()
    )]
    pub market: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    ///CHECK: Safe
    #[account(
        mut,
        owner = token_program.key()
    )]
    pub user_token_coin: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(
        mut,
        owner = token_program.key()
    )]
    pub user_token_pc: UncheckedAccount<'info>,
    ///CHECK: Safe
    #[account(mut)]
    pub user_token_lp: UncheckedAccount<'info>,
    ///CHECK: Safe
    pub token_program: Program<'info, Token>,
    ///CHECK: Safe
    pub associated_token_program: Program<'info, AssociatedToken>,
    ///CHECK: Safe
    pub system_program: Program<'info, System>,
    ///CHECK: Safe
    pub sysvar_rent: Sysvar<'info, Rent>,
    
}

impl<'a, 'b, 'c, 'info> From<&mut ProxyInitialize<'info>> for CpiContext<'a, 'b, 'c, 'info, Initialize2<'info>> {
    fn from(accounts: &mut ProxyInitialize<'info>) -> CpiContext<'a, 'b, 'c, 'info, Initialize2<'info>> {
        let cpi_accounts = Initialize2 {
            amm: accounts.amm.clone(),
            amm_authority: accounts.amm_authority.clone(),
            amm_open_orders: accounts.amm_open_orders.clone(),
            amm_lp_mint: accounts.amm_lp_mint.clone(),
            amm_coin_mint: accounts.amm_coin_mint.clone(),
            amm_pc_mint: accounts.amm_pc_mint.clone(),
            amm_coin_vault: accounts.amm_coin_vault.clone(),
            amm_pc_vault: accounts.amm_pc_vault.clone(),
            amm_target_orders: accounts.amm_target_orders.clone(),
            amm_config: accounts.amm_config.clone(),
            create_fee_destination: accounts.create_fee_destination.clone(),
            market_program: accounts.market_program.clone(),
            market: accounts.market.clone(),
            user_wallet: accounts.user_wallet.clone(),
            user_token_coin: accounts.user_token_coin.clone(),
            user_token_pc: accounts.user_token_pc.clone(),
            user_token_lp: accounts.user_token_lp.clone(),
            token_program: accounts.token_program.clone(),
            associated_token_program: accounts.associated_token_program.clone(),
            system_program: accounts.system_program.clone(),
            sysvar_rent: accounts.sysvar_rent.clone(),
        };
        let cpi_program = accounts.amm_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn initialize(
    ctx: Context<ProxyInitialize>,
    nonce: u8,
    open_time: u64,
    init_pc_amount: u64,
    init_coin_amount: u64,
) -> Result<()> {
    amm_anchor::initialize(
        ctx.accounts.into(),
        nonce,
        open_time,
        init_pc_amount,
        init_coin_amount,
    )
}