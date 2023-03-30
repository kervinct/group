use crate::prelude::*;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum SignType {
    Agreed,
    Denied,
}
#[account]
pub struct SignatureAccount {
    pub signer: Pubkey,
    pub created_at: i64,
    pub amount: u64,
    pub proposal: Pubkey,
    pub sign_type: SignType,
}
impl SignatureAccount {
    pub const SEEDS: &'static [u8] = b"signature";
    pub const LEN: usize = (1 + 32 + 8 + 8 + 32 + 1) as usize;

    #[inline(always)]
    pub fn is_agreed(&self) -> bool {
        if let SignType::Agreed = self.sign_type {
            true
        } else {
            false
        }
    }
    #[inline(always)]
    pub fn is_denied(&self) -> bool {
        !self.is_agreed()
    }
}
