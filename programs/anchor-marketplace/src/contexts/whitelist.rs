use crate::{state::Marketplace, state::Whitelist};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct WhitelistCollection<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        has_one = admin,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump
    )]
    marketplace: Account<'info, Marketplace>,
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        space = Whitelist::LEN,
        seeds = [marketplace.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    whitelist: Account<'info, Whitelist>,
    system_program: Program<'info, System>,
}

impl<'info> WhitelistCollection<'info> {
    pub fn whitelist(&mut self, bumps: &WhitelistCollectionBumps) -> Result<()> {
        self.whitelist.bump = bumps.whitelist;
        Ok(())
    }
}
