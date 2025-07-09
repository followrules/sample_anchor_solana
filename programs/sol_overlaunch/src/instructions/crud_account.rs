use anchor_lang::prelude::*;

use crate::state::GlobalState;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 32, seeds = [b"global-state"], bump)]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub initializer: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ChangeOwner<'info> {
    #[account(mut, seeds = [b"global-state"], bump)]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub current_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetOwner<'info> {
    #[account(seeds = [b"global-state"], bump)]
    pub global_state: Account<'info, GlobalState>,
}
