use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;
declare_id!("5zws23e8bohUwsmmGRR6AeXoGjXSyx2RPxNGCHVU1QpB");

#[program]
pub mod sol_overlaunch {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.global_state;
        if state.owner != Pubkey::default() {
            return Err(ProgramError::AccountAlreadyInitialized.into());
        }
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
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        signers: Vec<Pubkey>,
        threshold: u8,
        office_id: String,
    ) -> Result<()> {
        instructions::create_proposal::CreateProposal::handler(ctx, signers, threshold, office_id)
    }
    pub fn deposit_spl(ctx: Context<DepositSpl>, amount: u64) -> Result<()> {
        instructions::deposit_spl::deposit_spl_handler(ctx, amount)
    }

    pub fn propose_payment(
        ctx: Context<ProposePayment>,
        recipients: Vec<Recipient>,
        mint: Pubkey,
        nonce: u64,
    ) -> Result<()> {
        instructions::propose_payment::ProposePayment::handler(ctx, recipients, mint, nonce)
    }

    pub fn approve_payment(ctx: Context<ApprovePayment>) -> Result<()> {
        instructions::approve_payment::ApprovePayment::handler(ctx)
    }

    pub fn execute_sol_payment(ctx: Context<ExecuteSplPayment>) -> Result<()> {
        instructions::execute_payment::ExecuteSplPayment::handler(ctx)
    }


    pub fn init_spl_vault_handler(
        ctx: Context<InitSplVault>,
        vault_bump: u8,
    ) -> Result<()> {
        instructions::initial_spl_vault::init_spl_vault_handler(ctx, vault_bump)
    }

    pub fn refund_spl(ctx: Context<RefundSpl>, amount: u64) -> Result<()> {
        instructions::refund_spl_handler(ctx, amount)
    }

    // pub fn execute_spl_payment(ctx: Context<ExecuteSplPayment>) -> Result<()> {
    //     instructions::execute_spl_payment::ExecuteSplPayment::handler(ctx)
    // }
}
