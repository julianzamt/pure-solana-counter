use borsh::BorshDeserialize;
use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Counter {
    pub number: u32,       // 4 bytes
    pub authority: Pubkey, // 32 bytes
}

impl Sealed for Counter {}

impl Pack for Counter {
    const LEN: usize = 4 + 32;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let num = array_ref![src, 0, 4];
        let auth = array_ref![src, 5, Counter::LEN];

        let number = u32::from_le_bytes(*num);
        let authority = Pubkey::try_from_slice(auth)?;
        Ok(Counter { number, authority })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 4];

        let Counter { number, .. } = self;

        // FIXME -- serialization should work with a oneliner
        dst[0] = number.to_le_bytes()[0];
        dst[1] = number.to_le_bytes()[1];
        dst[2] = number.to_le_bytes()[2];
        dst[3] = number.to_le_bytes()[3];
    }
}
