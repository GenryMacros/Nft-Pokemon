use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Trade {
    pub is_initialized: bool,
    pub is_locked: bool,
    pub initializer_pubkey: Pubkey,
    pub amount: u64,
    pub price: u64
}

impl Sealed for Trade {}

impl IsInitialized for Trade{
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Trade {
    pub fn is_locked(&self) -> bool {
        self.is_locked
    }
}

impl Pack for Trade {
    const LEN: usize = 50;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Trade::LEN];
        let (
            is_initialized,
            is_locked,
            initializer_pubkey,
            amount,
            price,
        ) = array_refs![src, 1, 1, 32, 8, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_locked = match is_locked {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Trade {
            is_initialized,
            is_locked,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            amount: u64::from_le_bytes(*amount),
            price: u64::from_le_bytes(*price),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Trade::LEN];

        let (
            is_initialized_dst,
            is_locked_dst,
            initializer_pubkey_dst,
            amount_dst,
            price_dst,
        ) = mut_array_refs![dst, 1, 1, 32, 8, 8];

        let Trade {
            is_initialized,
            is_locked,
            initializer_pubkey,
            amount,
            price,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        is_locked_dst[0] = *is_locked as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        *amount_dst = amount.to_le_bytes();
        *price_dst = price.to_le_bytes();
    }
}

pub struct Item {
    pub is_initialized: bool,
    pub body_r: u8,
    pub body_g: u8,
    pub body_b: u8,
    pub stripe_r: u8,
    pub stripe_g: u8,
    pub stripe_b: u8,
    pub hat: u32,
    pub emotion: u8,
    pub eyes: u32,
    pub owner: Pubkey
}

impl Sealed for Item {}

impl IsInitialized for Item {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Item {
    const LEN: usize = 48;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Item::LEN];
        let (
            is_initialized,
            body_r,
            body_g,
            body_b,
            stripe_r,
            stripe_g,
            stripe_b,
            hat,
            emotion,
            eyes,
            owner,
        ) = array_refs![src, 1, 1, 1, 1, 1, 1, 1, 4, 1, 4, 32];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Item { 
            is_initialized,
            body_r: u8::from_le_bytes(*body_r),
            body_g: u8::from_le_bytes(*body_g),
            body_b: u8::from_le_bytes(*body_b),
            stripe_r: u8::from_le_bytes(*stripe_r),
            stripe_g: u8::from_le_bytes(*stripe_g),
            stripe_b: u8::from_le_bytes(*stripe_b),
            hat: u32::from_le_bytes(*hat),
            emotion: u8::from_le_bytes(*emotion),
            eyes: u32::from_le_bytes(*eyes),
            owner: Pubkey::new_from_array(*owner),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Item::LEN];

        let (
            is_initialized_dst,
            body_r_dst,
            body_g_dst,
            body_b_dst,
            stripe_r_dst,
            stripe_g_dst,
            stripe_b_dst,
            hat_dst,
            emotion_dst,
            eyes_dst,
            owner_dst
        ) = mut_array_refs![dst, 1, 1, 1, 1, 1, 1, 1, 4, 1, 4, 32];

        let Item {
            is_initialized,
            body_r,
            body_g,
            body_b,
            stripe_r,
            stripe_g,
            stripe_b,
            hat,
            emotion,
            eyes,
            owner,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        *body_r_dst = body_r.to_le_bytes();
        *body_g_dst = body_g.to_le_bytes();
        *body_b_dst = body_b.to_le_bytes();
        *stripe_r_dst = stripe_r.to_le_bytes();
        *stripe_g_dst = stripe_g.to_le_bytes();
        *stripe_b_dst = stripe_b.to_le_bytes();
        *hat_dst = hat.to_le_bytes();
        *emotion_dst = emotion.to_le_bytes();
        *eyes_dst = eyes.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
    }
}