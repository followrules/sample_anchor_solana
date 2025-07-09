use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub use instructions::*;
pub use state::*;

declare_id!("9uPQEcvmrkY4UvpGTsiqijFDoCqHSTrwMemW9FJFY4AD");

#[program]
pub mod sol_overlaunch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.global_state;
        state.owner = *ctx.accounts.initializer.key;
        Ok(())
    }

    pub fn get_owner(_ctx: Context<GetOwner>) -> Result<()> {
        Ok(())
    }

    pub fn change_owner(ctx: Context<ChangeOwner>, new_owner: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.global_state;

        require_keys_eq!(
            state.owner,
            ctx.accounts.current_owner.key(),
            CustomError::Unauthorized
        );

        let old_owner = state.owner;
        state.owner = new_owner;

        emit!(OwnerChanged {
            old_owner,
            new_owner
        });

        Ok(())
    }

}
