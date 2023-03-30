use crate::prelude::*;

#[account]
#[derive(Default)]
pub struct AdminAccount {
    pub seed: u8,
    pub current: u8,
    pub initialized: bool,
    padding: u8,
    pub groups: u32,
    pub token_mint: Pubkey,
    pub administrators: [Pubkey; 10],
}
impl AdminAccount {
    pub const SEEDS: &'static [u8] = b"group_admin";
    pub const LEN: usize = 1
        + 1
        + 1
        + 1 // padding
        + 4
        + 32
        + 320;

    #[inline]
    pub fn is_admin(&self, user: &Pubkey) -> bool {
        self.administrators.contains(user)
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.current == 10
    }

    #[inline]
    pub fn will_be_empty(&self) -> bool {
        self.current == 1
    }

    #[inline]
    pub fn add_administrator(&mut self, user: &Pubkey) {
        self.administrators[self.current as usize] = user.clone();
        self.current += 1;
    }

    #[inline(always)]
    pub fn last(&self) -> usize {
        self.current as usize - 1
    }

    #[inline]
    pub fn remove_administrator(&mut self, user: &Pubkey) {
        if let Some(pos) = self.administrators.iter().position(|pubkey| pubkey == user) {
            let last = self.administrators[self.last()];
            self.administrators[self.last()] = Pubkey::default();
            self.administrators[pos] = last.clone();
            self.current -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_size() {
        assert_eq!(std::mem::size_of::<AdminAccount>(), AdminAccount::LEN);
    }
}
