pub use super::*;

impl<
        SK: StoreKey,
        PSK: ParamsSubspaceKey,
        AK: AccountKeeper<SK>,
        BK: BankKeeper<SK>,
        KH: KeeperHooks<SK>,
    > Keeper<SK, PSK, AK, BK, KH>
{
    pub fn after_validator_created<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        validator: &Validator,
    ) {
        if let Some(ref hooks) = self.hooks_keeper {
            hooks.after_validator_created(ctx, validator.operator_address.clone());
        }
    }

    pub fn before_delegation_created<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        delegation: &Delegation,
    ) {
        if let Some(ref hooks) = self.hooks_keeper {
            hooks.before_delegation_created(
                ctx,
                delegation.delegator_address.clone(),
                delegation.validator_address.clone(),
            );
        }
    }

    pub fn after_delegation_modified<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        delegation: &Delegation,
    ) {
        if let Some(ref hooks) = self.hooks_keeper {
            hooks.after_delegation_modified(
                ctx,
                delegation.delegator_address.clone(),
                delegation.validator_address.clone(),
            );
        }
    }

    pub fn after_validator_bonded<DB: Database, CTX: TransactionalContext<DB, SK>>(
        &self,
        ctx: &mut CTX,
        validator: &Validator,
    ) {
        if let Some(ref hooks) = self.hooks_keeper {
            hooks.after_validator_bonded(
                ctx,
                validator.cons_addr(),
                validator.operator_address.clone(),
            );
        }
    }
}
