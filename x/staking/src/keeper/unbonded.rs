use crate::UnbondingDelegationEntry;

use super::*;

impl<
        SK: StoreKey,
        PSK: ParamsSubspaceKey,
        AK: AuthKeeper<SK, M>,
        BK: StakingBankKeeper<SK, M>,
        KH: KeeperHooks<SK, AK, M>,
        M: Module,
    > Keeper<SK, PSK, AK, BK, KH, M>
{
    pub fn unbond<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        del_addr: &AccAddress,
        val_addr: &ValAddress,
        shares: Decimal256,
    ) -> anyhow::Result<Uint256> {
        // check if a delegation object exists in the store
        let mut delegation = if let Some(delegation) = self.delegation(ctx, del_addr, val_addr)? {
            delegation
        } else {
            return Err(anyhow::anyhow!("no delegator for address"));
        };

        // call the before-delegation-modified hook
        self.before_delegation_shares_modified(ctx, del_addr, val_addr);

        // ensure that we have enough shares to remove
        if delegation.shares < shares {
            return Err(anyhow::anyhow!("not enough delegation shares"));
        }

        // get validator
        let mut validator = if let Some(validator) = self.validator(ctx, val_addr)? {
            validator
        } else {
            return Err(anyhow::anyhow!("no validator found"));
        };

        // subtract shares from delegation
        delegation.shares -= shares;

        let is_validator_operator =
            Vec::from(del_addr.clone()) == Vec::from(validator.operator_address.clone());

        // If the delegation is the operator of the validator and undelegating will decrease the validator's
        // self-delegation below their minimum, we jail the validator.
        if is_validator_operator
            && !validator.jailed
            && validator
                .tokens_from_shares(delegation.shares)?
                .to_uint_floor()
                < validator.min_self_delegation
        {
            self.jail_validator(ctx, &mut validator)?;
            validator = self.validator(ctx, &validator.operator_address)?.expect(
                "validator record must exists.\nPrevious step setup validator with the address.",
            )
        }

        // remove the delegation
        if delegation.shares.is_zero() {
            self.remove_delegation(ctx, &delegation)?;
        } else {
            self.set_delegation(ctx, &delegation)?;
            // call the after delegation modification hook
            self.after_delegation_modified(ctx, del_addr, &delegation.validator_address);
        }

        // remove the shares and coins from the validator
        // NOTE that the amount is later (in keeper.Delegation) moved between staking module pools
        let tokens_amount = self.remove_validator_tokens_and_shares(ctx, &mut validator, shares)?;
        if validator.delegator_shares.is_zero() && validator.status == BondStatus::Unbonded {
            // if not unbonded, we must instead remove validator in EndBlocker once it finishes its unbonding period
            self.remove_validator(ctx, &validator)?;
        }
        Ok(tokens_amount)
    }

    /// undelegate unbonds an amount of delegator shares from a given validator. It
    /// will verify that the unbonding entries between the delegator and validator
    /// are not exceeded and unbond the staked tokens (based on shares) by creating
    /// an unbonding object and inserting it into the unbonding queue which will be
    /// processed during the staking end_blocker.
    pub fn undelegate<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        del_addr: &AccAddress,
        val_addr: &ValAddress,
        shares: Decimal256,
    ) -> anyhow::Result<Timestamp> {
        // get validator
        let validator = if let Some(validator) = self.validator(ctx, val_addr)? {
            validator
        } else {
            return Err(anyhow::anyhow!("no validator found"));
        };

        if self.has_max_unbonding_delegation_entries(ctx, del_addr, val_addr)? {
            return Err(anyhow::anyhow!(
                "unbonding delegation max entries limit exceeded"
            ));
        }

        let return_amount = self.unbond(ctx, del_addr, val_addr, shares)?;

        // transfer the validator tokens to the not bonded pool
        if validator.status == BondStatus::Bonded {
            self.bonded_tokens_to_not_bonded(ctx, return_amount)?;
        }

        let params = self.staking_params_keeper.try_get(ctx)?;
        let completion_time = ctx.get_time().checked_add(params.unbonding_time()).unwrap();

        let entry = UnbondingDelegationEntry {
            creation_height: ctx.height(),
            completion_time,
            initial_balance: return_amount,
            balance: return_amount,
        };
        let unbonding_delegation =
            self.set_unbonding_delegation_entry(ctx, del_addr, val_addr, entry)?;

        self.insert_ubd_queue(ctx, &unbonding_delegation, completion_time)?;
        Ok(completion_time)
    }

    pub fn unbonded_to_bonded<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        validator: &mut Validator,
    ) -> anyhow::Result<()> {
        if validator.status != BondStatus::Unbonded {
            return Err(anyhow::anyhow!(
                "bad state transition unbonded to bonded, validator: {}",
                validator.operator_address
            ));
        }
        self.bond_validator(ctx, validator)?;
        Ok(())
    }

    /// ValidateUnbondAmount validates that a given unbond or redelegation amount is
    /// valied based on upon the converted shares. If the amount is valid, the total
    /// amount of respective shares is returned, otherwise an error is returned.
    pub fn validate_unbond_amount<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &CTX,
        del_addr: &AccAddress,
        val_addr: &ValAddress,
        amount: Uint256,
    ) -> anyhow::Result<Decimal256> {
        let validator = self
            .validator(ctx, val_addr)?
            .ok_or(anyhow::anyhow!("account not found"))?;
        let delegation = self
            .delegation(ctx, del_addr, val_addr)?
            .ok_or(anyhow::anyhow!("Delegation is not found."))?;
        let mut shares = validator.shares_from_tokens(amount)?;
        let truncated_shares = validator.shares_from_tokens_truncated(amount)?;
        let delegation_shares = delegation.shares;

        if truncated_shares > delegation_shares {
            return Err(anyhow::anyhow!("invalid shares amount"));
        }

        // Cap the shares at the delegation's shares. Shares being greater could occur
        // due to rounding, however we don't want to truncate the shares or take the
        // minimum because we want to allow for the full withdraw of shares from a
        // delegation.
        if shares > delegation_shares {
            shares = delegation_shares;
        }

        Ok(shares)
    }
}
