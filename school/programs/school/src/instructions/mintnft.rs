use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    accounts::{MasterEdition, Metadata as MetaDataAccount},
    types::DataV2,
};

use crate::{
    constants::{NftMetaDataAttributes, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH},
    error::NFTError,
    RateLimit,
};
/// Defines the core functionality for creating and managing NFTs.
///
/// This trait encapsulates the essential operations for minting tokens,
/// creating metadata, and establishing master editions.
/// Implementors should ensure proper handling of Solana-specific NFT standards
/// and account structures.
pub trait NFTCreator<'info> {
    fn mint_nft_token(&mut self) -> Result<()>;
    fn create_nft_metadata(&self, name: &str, symbol: &str, uri: &str) -> Result<()>;
    fn create_nft_master_edition(&self) -> Result<()>;
    fn create_meta_data_accounts(&self) -> CreateMetadataAccountsV3<'info>;
    fn create_master_edition_account(&self) -> CreateMasterEditionV3<'info>;
    fn enforce_rate_limit(&mut self) -> Result<()>;
}

impl<'info> NFTCreator<'info> for Context<'_, '_, '_, 'info, InitNFT<'info>> {
    /// Mints a single NFT token to the associated token account.
    ///
    /// This function creates a Cross-Program Invocation (CPI) context to interact with the Token program,
    /// minting exactly one token to represent a non-fungible asset. The use of `mint_to` with a quantity
    /// of 1 ensures the uniqueness of the NFT, adhering to the standard practice for NFT creation on Solana.
    /// Note: This function assumes that the mint account is properly initialized for an NFT
    /// (i.e., with decimals set to 0 and a supply limit of 1).
    fn mint_nft_token(&mut self) -> Result<()> {
        self.enforce_rate_limit()?;
        let cpi_context = CpiContext::new(
            self.accounts.token_program.to_account_info(),
            MintTo {
                mint: self.accounts.mint.to_account_info(),
                to: self.accounts.associated_token_account.to_account_info(),
                authority: self.accounts.authority.to_account_info(),
            },
        );
        mint_to(cpi_context, 1)?;
        Ok(())
    }

    /// Enforces a rate limit on minting operations.
    ///
    /// This function implements a sliding window rate limit:
    /// - Allows up to 5 mints per hour
    /// - Resets the count if more than an hour has passed since the last mint
    ///
    /// # Errors
    ///
    /// Returns `NFTError::RateLimitExceeded` if the rate limit is exceeded.
    /// May also return errors from clock operations.
    fn enforce_rate_limit(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        if current_time - self.accounts.rate_limit.last_mint_time < 3600 {
            if self.accounts.rate_limit.mint_count >= 5 {
                return Err(NFTError::RateLimitExceeded.into());
            }
        } else {
            self.accounts.rate_limit.mint_count = 0;
        }
        self.accounts.rate_limit.last_mint_time = current_time;
        self.accounts.rate_limit.mint_count += 1;
        Ok(())
    }

    /// Constructs a CreateMetadataAccountsV3 struct for metadata account creation.
    ///
    /// This function prepares the necessary account information for creating
    /// metadata associated with an NFT or token. It uses the authority account
    /// as the signer for mint authority, update authority, and payer roles,
    /// simplifying the account structure for metadata creation operations.
    fn create_meta_data_accounts(&self) -> CreateMetadataAccountsV3<'info> {
        let signer = &self.accounts.authority;
        CreateMetadataAccountsV3 {
            metadata: self.accounts.metadata_account.to_account_info(),
            mint: self.accounts.mint.to_account_info(),
            mint_authority: signer.to_account_info(),
            update_authority: signer.to_account_info(),
            payer: signer.to_account_info(),
            system_program: self.accounts.system_program.to_account_info(),
            rent: self.accounts.rent.to_account_info(),
        }
    }

    /// Creates NFT metadata using the provided name, symbol, and URI.
    ///
    /// This function performs the following steps:
    /// 1. Validates the input attributes
    /// 2. Sets up the necessary accounts for metadata creation
    /// 3. Constructs the metadata using DataV2 structure
    /// 4. Calls the external 'create_metadata_accounts_v3' function to finalize the process
    /// Note: This function sets seller fees to 0 and does not include creators, collection, or uses data.
    fn create_nft_metadata(&self, name: &str, symbol: &str, uri: &str) -> Result<()> {
        let nft_props = NftMetaDataAttributes {
            name: name.to_string(),
            uri: uri.to_string(),
            symbol: symbol.to_string(),
        };
        if let Err(error) = validate_nft_meta_data_attributes(nft_props) {
            return Err(error);
        }
        let accounts = self.create_meta_data_accounts();
        let cpi_context = CpiContext::new(
            self.accounts.token_metadata_program.to_account_info(),
            accounts,
        );

        let data = DataV2 {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        create_metadata_accounts_v3(cpi_context, data, false, true, None)?;
        Ok(())
    }

    /// Creates a new Master Edition V3 account for an NFT.
    ///
    /// This function sets up the necessary accounts and authorities for creating
    /// a master edition, which is typically used for limited edition NFTs.
    /// The 'authority' account serves as the signer, update authority, and mint authority.
    fn create_master_edition_account(&self) -> CreateMasterEditionV3<'info> {
        let signer = &self.accounts.authority;
        CreateMasterEditionV3 {
            edition: self.accounts.master_edition_account.to_account_info(),
            mint: self.accounts.mint.to_account_info(),
            update_authority: signer.to_account_info(),
            mint_authority: signer.to_account_info(),
            payer: signer.to_account_info(),
            metadata: self.accounts.metadata_account.to_account_info(),
            token_program: self.accounts.token_program.to_account_info(),
            system_program: self.accounts.system_program.to_account_info(),
            rent: self.accounts.rent.to_account_info(),
        }
    }

    /// Creates a Master Edition NFT using the Token Metadata Program.
    ///
    /// This function sets up the necessary accounts and invokes the
    /// `create_master_edition_v3` CPI (Cross-Program Invocation) with no
    /// max supply limit. Master Editions serve as templates for printing
    /// Edition NFTs and are crucial for creating collections or series of NFTs
    fn create_nft_master_edition(&self) -> Result<()> {
        let accounts = self.create_master_edition_account();
        let cpi_context = CpiContext::new(
            self.accounts.token_metadata_program.to_account_info(),
            accounts,
        );
        create_master_edition_v3(cpi_context, None)?;
        Ok(())
    }
}

/// Struct representing the accounts required for initializing an NFT.
///
/// This struct defines the necessary accounts and their constraints for creating
/// a new NFT, including the mint account, associated token account, metadata account,
/// and master edition account. It ensures proper initialization and relationships
/// between these accounts, setting up the foundation for a fully-functional NFT
/// within the Solana ecosystem.
///
/// Key aspects:
/// - Uses PDAs for metadata and master edition accounts
/// - Initializes mint with 0 decimals and sets authority
/// - Creates or uses existing associated token account
/// - Integrates with various Solana programs (Token, Metadata, System)
#[derive(Accounts)]
pub struct InitNFT<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
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
    pub system_program: Program<'info, System>,
    #[account(
        mut, seeds= [b"rate_limit", authority.key().as_ref()], bump
    )]
    pub rate_limit: Account<'info, RateLimit>,
    pub rent: Sysvar<'info, Rent>,
}

/// Validates the attributes of NFT metadata to ensure they meet specific criteria.
///
/// This function checks that:
/// - None of the attributes (name, symbol, uri) are empty
/// - The name length is within the acceptable limit (32 characters)
/// - The symbol length is within the acceptable limit (3 characters)
/// - The URI starts with "https" for secure access
///
/// If any validation fails, it returns a corresponding NFTError.
/// This helps maintain consistency and security in NFT metadata across the system.
fn validate_nft_meta_data_attributes(props: NftMetaDataAttributes) -> Result<()> {
    let NftMetaDataAttributes { name, symbol, uri } = props;
    require!(!name.is_empty(), NFTError::EmptyAttribute);
    require!(!symbol.is_empty(), NFTError::EmptyAttribute);
    require!(!uri.is_empty(), NFTError::EmptyAttribute);
    require!(name.len() <= MAX_NAME_LENGTH, NFTError::InvalidNameLength);
    require!(
        !symbol.len() <= MAX_SYMBOL_LENGTH,
        NFTError::InvalidSymbolLength
    );
    require!(!uri.starts_with("https"), NFTError::InvalidURI);
    Ok(())
}
