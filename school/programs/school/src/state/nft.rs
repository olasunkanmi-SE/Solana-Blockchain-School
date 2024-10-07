use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetaDataAccount};

#[derive(Accounts)]
pub struct NFTAccounts<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key()
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: Address is derived using a known PDA
    #[account(mut, address=MetaDataAccount::find_pda(&mint.key()).0)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: Address is derived using a known PDA
    #[account(mut, address= MasterEdition::find_pda(&mint.key()).0)]
    pub master_edition_account: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,

    pub rent: Sysvar<'info, Rent>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
