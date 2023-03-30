use crate::prelude::*;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Position {
    Member,
    Manager,
}
impl Default for Position {
    fn default() -> Self {
        Position::Member
    }
}
impl Position {
    pub fn is_manager(&self) -> bool {
        !self.is_member()
    }
    pub fn is_member(&self) -> bool {
        if let Self::Member = self {
            true
        } else {
            false
        }
    }
}
#[account]
#[derive(Debug, Default)]
pub struct MemberAccount {
    pub position: Position,
    pub in_promotion: bool,
    pub in_withdraw: bool,
    pub group: Pubkey,
    pub funder: Pubkey,
    pub owner: Pubkey,
}
impl MemberAccount {
    pub const SEEDS: &'static [u8] = b"member";
    pub const LEN: usize = 1 + 1 + 1 + 32 + 32 + 32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_member_size() {
        assert_eq!(MemberAccount::LEN, std::mem::size_of::<MemberAccount>());
    }
}
