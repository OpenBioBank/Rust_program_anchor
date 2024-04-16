use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("F7uwK9BXr7orrQWJujdoTVbd7P7yMDnPA84udYYyPPLM");

#[program]
pub mod rust_program_anchor {

    use super::*;

    pub fn initialize_mint_account(
        ctx: Context<InitializeMintAccount>,
        id: u64, 
        description: String, 
        owner: String,
        creator: String,
        authorize: bool,
        url: String,
        is_initialized: bool,
        cid: String,
        is_mutable: bool,
    ) -> Result<()> {
        let metadata_account = &mut ctx.accounts.metadata_account;
        
        metadata_account.id = id;
        metadata_account.description = description;
        metadata_account.owner = owner;
        metadata_account.creator = creator;
        metadata_account.authorize = authorize;
        metadata_account.url = url;
        metadata_account.is_initialized = is_initialized;
        metadata_account.cid = cid.clone();
        metadata_account.is_mutable = is_mutable;

        Ok(())
    }

    pub fn create_nft(
        ctx: Context<CreateNft>,
        cid: String,
    ) -> Result<()> {
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&[ctx.accounts.initializer.key().as_ref(),cid.as_bytes(),&[ctx.bumps.mint]]]),
                1,
            )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    id:u64,
    description:String,
    owner: String,
    creator:String,
    authorize: bool,
    url: String,
    is_initialized: bool,
    cid: String,
    is_mutable: bool,
)]
pub struct InitializeMintAccount<'info> {
    #[account(
        init,
        seeds = [initializer.key().as_ref(),cid.as_bytes()],
        bump,
        payer = initializer,
        mint::decimals = 0,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [mint.key().as_ref(),cid.as_bytes()],
        bump,
        payer = initializer,
        space = 8 + 4 + description.len() + owner.len() + creator.len() + 4 + url.len() + 4 + cid.len() + 4,
    )]
    pub metadata_account: Account<'info,MetadataAccount>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
#[instruction(cid:String)]
pub struct CreateNft<'info> {
    #[account(
        seeds = [initializer.key().as_ref(),cid.as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>
}

#[account]
pub struct MetadataAccount {
    pub id: u64,
    pub description: String,
    pub owner: String,
    pub creator: String,
    pub authorize: bool,
    pub url: String,
    pub is_initialized: bool,
    pub cid: String,
    pub is_mutable: bool,
}