use borsh::{BorshDeserialize, BorshSerialize};
use coal_utils::AccountDeserialize;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct RewardState {
    pub id: u64,
    pub rewards: u64,
    pub claimed: u64,
    pub top_miners: [Pubkey; 8],
    pub top_rewards: [u64; 8],
}

impl AccountDeserialize for RewardState {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        borsh::from_slice::<Self>(data)
            .map_err(|_| ProgramError::InvalidAccountData)
            .and_then(|_| {
                // SAFETY: We can transmute slice to reference safely since it’s valid after Borsh check
                Ok(unsafe { &*(data.as_ptr() as *const Self) })
            })
    }

    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        borsh::from_slice::<Self>(data)
            .map_err(|_| ProgramError::InvalidAccountData)
            .and_then(|_| {
                Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) })
            })
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub last_reset_at: i64,
    pub min_difficulty: u64,
    pub base_reward_rate: u64,
    pub top_balance: u64,
}

impl AccountDeserialize for Config {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        borsh::from_slice::<Self>(data)
            .map_err(|_| ProgramError::InvalidAccountData)
            .and_then(|_| Ok(unsafe { &*(data.as_ptr() as *const Self) }))
    }

    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        borsh::from_slice::<Self>(data)
            .map_err(|_| ProgramError::InvalidAccountData)
            .and_then(|_| Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) }))
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub struct MinerState {
    pub authority: Pubkey,
    pub miner: Pubkey,
    pub balance: u64,
    pub last_hash: [u8; 32],
    pub last_hash_at: i64,
    pub last_stake_at: i64,
    pub total_hashes: u64,
    pub total_rewards: u64,
    pub challenge: [u8; 32],
}

impl AccountDeserialize for MinerState {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        borsh::from_slice::<Self>(data)
            .map_err(|_| ProgramError::InvalidAccountData)
            .and_then(|_| Ok(unsafe { &*(data.as_ptr() as *const Self) }))
    }

    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        borsh::from_slice::<Self>(data)
            .map_err(|_| ProgramError::InvalidAccountData)
            .and_then(|_| Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) }))
    }
}
