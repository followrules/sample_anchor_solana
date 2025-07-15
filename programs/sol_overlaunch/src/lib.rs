use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;
declare_id!("8BxFXbfwnVWafHev8SBgzdipFXTxieFuaWmBqYgjVDee");

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
    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        signers: Vec<Pubkey>,
        threshold: u8,
        office_id: String,
    ) -> Result<()> {
        instructions::create_multisig::CreateMultisig::handler(ctx, signers, threshold, office_id)
    }
    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        instructions::deposit_spl::deposit_spl_handler(ctx, amount)
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        instructions::deposit_sol::DepositSol::handler(ctx, amount)
    }

    pub fn propose_payment(
        ctx: Context<ProposePayment>,
        to: Pubkey,
        amount: u64,
        is_spl: bool,
        mint: Option<Pubkey>,
        nonce: u64,
    ) -> Result<()> {
        instructions::propose_payment::ProposePayment::handler(ctx, to, amount, is_spl, mint, nonce)
    }

    pub fn approve_payment(ctx: Context<ApprovePayment>, signer_index: u8) -> Result<()> {
        instructions::approve_payment::ApprovePayment::handler(ctx, signer_index)
    }

    pub fn execute_sol_payment(ctx: Context<ExecuteSolPayment>) -> Result<()> {
        instructions::execute_sol_payment::ExecuteSolPayment::handler(ctx)
    }

    pub fn execute_spl_payment(ctx: Context<ExecuteSplPayment>) -> Result<()> {
        instructions::execute_spl_payment::ExecuteSplPayment::handler(ctx)
    }
}
