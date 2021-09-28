use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use crate::error::NftError;

pub struct ItemParams {
        pub body_r: u8,
        pub body_g: u8,
        pub body_b: u8,
        pub stripe_r: u8,
        pub stripe_g: u8,
        pub stripe_b: u8,
        pub hat: u32,
        pub emotion: u8,
        pub eyes: u32,
}

pub enum NftInstruction {
/// Accounts expected:
///
///0. `[signer]` The account of the trade creator
///1. `[writable]` Uninitialized trade account
///2. `[]` Trade account rent
    CreateTrade {
        start_price: u64,
    },

/// 0. `[signer]` The account of the trader
/// 1. `[writable]` The trade account
/// 2. `[]` Trade account rent
/// 3. `[writable]` The item account
/// 4. `[]` Item account rent
/// 5. `[]` Token program
    AddItem {
        params: ItemParams,
    },

///0. [Signer] Buyer
///1. [writable] Trade account
///2. [writable] Item account
///3. [writable] Funded with sol account 
    BuyItem,
}


impl NftInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(NftError::InvalidInstruction)?;

        Ok(match tag {
            0 => Self::AddItem {
                params: ItemParams {
                    body_r: Self::unpack_amount(rest, 0, 1)? as u8,
                    body_g: Self::unpack_amount(rest, 1, 2)? as u8,
                    body_b: Self::unpack_amount(rest, 2, 3)? as u8,
                    stripe_r: Self::unpack_amount(rest, 3, 4)? as u8,
                    stripe_g: Self::unpack_amount(rest, 5, 6)? as u8,
                    stripe_b: Self::unpack_amount(rest, 6, 7)? as u8,
                    hat: Self::unpack_amount(rest, 7, 11)? as u32,
                    emotion: Self::unpack_amount(rest, 11, 12)? as u8,
                    eyes: Self::unpack_amount(rest, 12, 16)? as u32,
                }
            },
            1 => Self::CreateTrade {
                    start_price: Self::unpack_amount(rest, 0, 8)?,
            },
            2 => return Ok(Self::BuyItem),
            _ => return Err(NftError::InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8], start: usize, end: usize) -> Result<u64, ProgramError> {
        let amount = input
            .get(start..end)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(NftError::InvalidInstruction)?;
        Ok(amount)
    }
}