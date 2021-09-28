use std::convert::TryInfo;
use solana_program::program_error::ProgramError;

pub struct ItemParams {
        body_r: u8,
        body_g: u8,
        body_b: u8,
        stripe_r: u8,
        stripe_g: u8,
        stripe_b: u8,
        hat: u32,
        emotion: u8,
        eyes: u32,
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
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::AddItem {
                params: ItemParams {
                    body_r: Self::unpack_amount(rest, 0, 1),
                    body_g: Self::unpack_amount(rest, 1, 2),
                    body_b: Self::unpack_amount(rest, 2, 3),
                    stripe_r: Self::unpack_amount(rest, 3, 4),
                    stripe_g: Self::unpack_amount(rest, 5, 6),
                    stripe_b: Self::unpack_amount(rest, 6, 7),
                    hat: Self::unpack_amount(rest, 7, 11),
                    emotion: Self::unpack_amount(rest, 11, 12),
                    eyes: Self::unpack_amount(rest, 12, 16),
                }
            },
            1 => Self::CreateTrade {
                    start_price: Self::unpack_amount(rest, 0, 8),
            },
            2 => Self::BuyItem 
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8], start: u8, end: u8) -> Result<u64, ProgramError> {
        let amount = input
            .get(start..end)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount);
    }
}