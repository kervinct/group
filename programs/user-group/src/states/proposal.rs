use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
#[repr(C)]
#[non_exhaustive]
pub enum ProposalType {
    Upgrade,
    Downgrade,
    UpdateGroup {
        max_manager: u32,
    },
    ReElection,
    Withdraw {
        mint: Pubkey,
        // always token account for above mint
        receiver: Pubkey,
        amount: u64,
    },
}
impl Default for ProposalType {
    fn default() -> Self {
        Self::Upgrade
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum ProposalStatus {
    Progressing,
    Passed { time: i64 },
    Rejected { time: i64 },
    Updated { time: i64 },
}
impl Default for ProposalStatus {
    fn default() -> Self {
        Self::Progressing
    }
}
#[account]
#[derive(Default, Debug, PartialEq)]
pub struct ProposalAccount {
    pub submitter: Pubkey,
    pub beneficiary: Pubkey,
    pub bene_member: Pubkey,
    pub group: Pubkey,
    pub positive: u32,
    pub negative: u32,
    pub limit: u64,
    pub deadline: i64,
    pub revoke_timeout: i64,
    pub close_timeout: i64,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
}
impl ProposalAccount {
    pub const SEEDS: &'static [u8] = b"proposal";
    pub const LEN: usize = 32 + 32 + 32 + 32 + 4 + 4 + 8 + 8 + 8 + 8 + 80 + 16;

    #[inline(always)]
    pub fn participated(&self) -> u64 {
        (self.positive + self.negative) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_proposal_size() {
        assert_eq!(ProposalAccount::LEN, std::mem::size_of::<ProposalAccount>());
    }

    #[test]
    pub fn test_upgrade_progressing() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Upgrade,
            status: ProposalStatus::Progressing,
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_upgrade_passed() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Upgrade,
            status: ProposalStatus::Passed { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_upgrade_updated() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Upgrade,
            status: ProposalStatus::Updated { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_downgrade_progressing() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Downgrade,
            status: ProposalStatus::Progressing,
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_downgrade_passed() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Downgrade,
            status: ProposalStatus::Passed { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_downgrade_updated() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Upgrade,
            status: ProposalStatus::Updated { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_update_group_progressing() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::UpdateGroup { max_manager: 10 },
            status: ProposalStatus::Progressing,
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_update_group_passed() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::UpdateGroup { max_manager: 10 },
            status: ProposalStatus::Passed { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_update_group_updated() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::UpdateGroup { max_manager: 10 },
            status: ProposalStatus::Updated { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_reelection_progressing() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::ReElection,
            status: ProposalStatus::Progressing,
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_reelection_passed() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::ReElection,
            status: ProposalStatus::Passed { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_reelection_updated() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::ReElection,
            status: ProposalStatus::Updated { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_withdraw_progressing() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Withdraw {
                mint: Pubkey::default(),
                receiver: Pubkey::default(),
                amount: 1000000,
            },
            status: ProposalStatus::Progressing,
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_withdraw_passed() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Withdraw {
                mint: Pubkey::default(),
                receiver: Pubkey::default(),
                amount: 1000000,
            },
            status: ProposalStatus::Passed { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }

    #[test]
    pub fn test_withdraw_updated() {
        let proposal = ProposalAccount {
            proposal_type: ProposalType::Withdraw {
                mint: Pubkey::default(),
                receiver: Pubkey::default(),
                amount: 1000000,
            },
            status: ProposalStatus::Updated { time: 1642650232 },
            ..ProposalAccount::default()
        };
        let mut data = vec![0u8; ProposalAccount::LEN];
        assert_eq!(data.len(), ProposalAccount::LEN);
        assert!(AnchorSerialize::serialize(&proposal, &mut data.as_mut_slice()).is_ok());
        let res: ProposalAccount = AnchorDeserialize::deserialize(&mut data.as_slice()).unwrap();
        assert_eq!(proposal, res);
    }
}
