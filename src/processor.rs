use solana_program:: {
    account_info::{ next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, isInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::invoke,
}

use crate::{instruction::NftInstruction, error::NftError, state::Trade, state::Item};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult
    {
        let instruction = NftInstruction::unpack(instruction_data)?;

        match instruction {
            NftInstruction::AddItem {params} => {
                Self::process_add_item(accounts, params, program_id);
            },
            NftInstruction::CreateTrade { price } => {
                Self::process_create_trade(accounts, price, program_id);
            },
            NftInstruction::BuyItem => {
                Self::process_buy_item(accounts);
            }
        }
    }
    
    pub fn process_create_trade(
        accounts: &[AccountInfo],
        price: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        let mut trade_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        
        if !rent.is_exempt(trade_account.lamports(), trade_account.data_len()) {
            return Err(NftError::NotRentExempt.into());
        }

        let mut trade_info = Escrow::unpack_unchecked(&trade_account.data.borrow())?;
        if trade_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        trade_info.is_initialized = true;
        trade_info.start_price = price;

        Trade::pack(trade_info, &mut trade_account.data.borrow_mut())?;
        Ok(())
    }

    pub fn process_add_item(
        accounts: &[AccountInfo],
        params: ItemParams,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        
        let mut trade_account = next_account_info(account_info_iter)?;
        let mut trade_rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;
        let mut item_account = next_account_info(account_info_iter)?;
        let mut item_rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !trade_rent.is_exempt(trade_account.lamports(), trade_account.data_len()) ||
           !item_rent.is_exempt(item_account.lamports(), item_account.data_len()) {
            return Err(NftError::NotRentExempt.into());
        }
        let mut item_info = Item::unpack_unchecked(&item_account.data.borrow())?;
        let mut trade_info = Trade::unpack_unchecked(&trade_account.data.borrow())?;

        if trade_info.is_initialized() || item_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        if trade_info.initializer_pubkey != *initializer.key {
            return Err(ProgramError::InvalidAccountData);
        }
        if item_info.owner != *initializer.key {
            return Err(ProgramError::InvalidAccountData);
        }

        trade_info.amount += 1;

        item_info.is_initialized = true;
        item_info.body_r = params.body_r;
        item_info.body_g = params.body_g;
        item_info.body_b = params.body_b;
        item_info.stripe_r = params.stripe_r;
        item_info.stripe_g = params.stripe_g;
        item_info.stripe_b = params.stripe_b;
        item_info.hat = params.hat;
        item_info.emotion = params.emotion;
        item_info.eyes = params.eyes;
        item_info.owner = *trade_account.key;
        
        Item::pack(item_info, &mut item_account.data.borrow_mut())?;
        Trade::pack(trade_info, &mut trade_account.data.borrow_mut())?;


        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            item_account.key,
            Some(&(*trade_account.key)),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;

        invoke(
            &owner_change_ix,
            &[
                item_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
    
    pub fn process_buy_item( 
        accounts: &[AccountInfo],
    ) -> ProgramResult {

        let account_info_iter = &mut accounts.iter();
        let taker = next_account_info(account_info_iter)?;

        if !taker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let trade_account = next_account_info(account_info_iter)?;
        let mut trade_info = Trade::unpack_unchecked(&trade_account.data.borrow())?;
        let item_account = next_account_info(account_info_iter)?;
        let mut item_info = Item::unpack_unchecked(&item_account.data.borrow())?;
        
        if !trade_info.is_locked || !trade_info.is_initialized {
            return Err(NftError::InvalidTrade.into());
        }
        if trade_info.amount == 0 {
            return Err(NftError::NoTokens.into());
        }
        if !item_info.is_initialized {
            return Err(NftError::InvalidToken.into());
        }
        if item_info.owner != *trade_account.key {
            return Err(NftError::InvalidInstruction.into());
        }

        let funded_account = next_account_info(account_info_iter)?;
        let currentPrice = (1 / trade_info.amount) + trade_info.start_price;
        if *funded_account.lamports != currentPrice {
            return Err(NftError::NotEnoughLamports.into());
        }

        item_info.owner = *taker.key;
        trade_info.amount -= 1;

        Item::pack(item_info, &mut item_account.data.borrow_mut())?;
        Trade::pack(trade_info, &mut trade_account.data.borrow_mut())?;

        **trade_account.lamports.borrow_mut() = trade_account.lamports()
            .checked_add(funded_account.lamports())
            .ok_or(EscrowError::AmountOverflow)?;
        **funded_account.lamports.borrow_mut() = 0;
        
        Ok(())
    }
}