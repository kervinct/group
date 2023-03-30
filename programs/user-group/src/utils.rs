use crate::prelude::*;

const TIME_ELAPSE: i64 = 43200; // 12 hours

pub(crate) fn is_admin(admin: &AdminAccount, user: &Pubkey) -> Result<()> {
    if !admin.is_admin(user) {
        return if cfg!(feature = "dev") {
            err!(GroupError::OperationUnauthorized)
        } else {
            Err(GroupError::OperationUnauthorized.into())
        };
    }
    Ok(())
}

pub(crate) fn is_not_admin(admin: &AdminAccount, user: &Pubkey) -> Result<()> {
    if admin.is_admin(user) {
        return if cfg!(feature = "dev") {
            err!(GroupError::AlreadyAdmin)
        } else {
            Err(GroupError::AlreadyAdmin.into())
        };
    }
    Ok(())
}

pub(crate) fn is_valid_deadline(ctx: &Context<SubmitProposal>, deadline: i64) -> Result<()> {
    if deadline < &ctx.accounts.clock.unix_timestamp + TIME_ELAPSE {
        return if cfg!(feature = "dev") {
            err!(GroupError::NotEnoughTimeElapse)
        } else {
            Err(GroupError::NotEnoughTimeElapse.into())
        };
    }
    Ok(())
}

pub(crate) fn is_valid_limit(limit: u64) -> Result<()> {
    if limit == 0 {
        return if cfg!(feature = "dev") {
            err!(GroupError::InvalidLimit)
        } else {
            Err(GroupError::InvalidLimit.into())
        };
    }
    Ok(())
}

pub(crate) fn is_not_outdated(proposal: &ProposalAccount, now: i64) -> Result<()> {
    if now > proposal.revoke_timeout {
        return if cfg!(feature = "dev") {
            err!(GroupError::RevokeOutdated)
        } else {
            Err(GroupError::RevokeOutdated.into())
        };
    }
    Ok(())
}

pub(crate) fn is_valid_elapse(proposal: &ProposalAccount, now: i64) -> Result<()> {
    if now > proposal.deadline {
        return if cfg!(feature = "dev") {
            err!(GroupError::ProposalOutdated)
        } else {
            Err(GroupError::ProposalOutdated.into())
        };
    }
    Ok(())
}

pub(crate) fn is_proposal_allow_close(proposal: &ProposalAccount, now: i64) -> Result<()> {
    match proposal.status {
        ProposalStatus::Progressing | ProposalStatus::Updated { .. } => {
            if proposal.deadline > now {
                return if cfg!(feature = "dev") {
                    err!(GroupError::NotAllowedToCloseProgressingProposal)
                } else {
                    Err(GroupError::NotAllowedToCloseProgressingProposal.into())
                };
            }
        }
        _ => {}
    }
    Ok(())
}
