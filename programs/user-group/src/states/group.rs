use crate::prelude::*;

#[derive(Default, Copy, Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[repr(C)]
pub struct ParticipateRate {
    pub numerator: u8,
    pub denominator: u8,
}
impl ParticipateRate {
    pub fn new(numerator: u8, denominator: u8) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
    pub fn calc_number(&self, num: u64) -> u64 {
        num.checked_mul(self.numerator as u64)
            .unwrap_or(0)
            .checked_div(self.denominator as u64)
            .unwrap_or(u64::MAX)
    }
}
#[account(zero_copy)]
#[derive(Debug, Default, AnchorSerialize, AnchorDeserialize)]
pub struct GroupAccount {
    pub seed: u8,
    pub electing: bool,
    pub freeze: bool,
    pub rate: ParticipateRate,
    pub update: bool,
    padding: [u8; 2],
    pub max_manager: u32,
    pub current_manager: u32,
    pub current_member: u32,
    pub proposals: u32,
    pub index: u32,
    pub sponsor: Pubkey,
    pub admin: Pubkey,
}
impl GroupAccount {
    pub const SEEDS: &'static [u8] = b"group";
    pub const LEN: usize = 1
        + 1
        + 1
        + 2
        + 1
        + 2 // padding
        + 4
        + 4
        + 4
        + 4
        + 4
        + 32
        + 32;

    #[inline(always)]
    pub fn manager_number(&self) -> u64 {
        self.current_manager as u64
    }
    #[inline(always)]
    pub fn member_number(&self) -> u64 {
        self.current_member as u64
    }
    #[inline(always)]
    pub fn total_user(&self) -> u64 {
        self.manager_number() + self.member_number()
    }
    #[inline(always)]
    pub fn hence(&self) -> u64 {
        self.rate.calc_number(self.total_user())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_group_size() {
        assert_eq!(std::mem::size_of::<GroupAccount>(), GroupAccount::LEN);
    }
}
