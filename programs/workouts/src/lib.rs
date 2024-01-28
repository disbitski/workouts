use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("4tRQBjKLrcbkX4gqd3xbVmacn1QgSZX59QczBLv32bPG");

#[program]
pub mod workouts {
    use super::*;

    const WORKOUTS_GOAL: i64 = 100;

    pub fn initialize(ctx: Context<Initialize>, init_message: String) -> ProgramResult {
        let workouts = &mut ctx.accounts.workouts;
        workouts.greeting = init_message;

        Ok(())
    }

    pub fn goal(ctx: Context<Goal>, number: i64) -> ProgramResult {
        let workouts = &mut ctx.accounts.workouts;
        workouts.remaining = WORKOUTS_GOAL-number;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 264)]
    pub workouts: Account<'info, Workouts>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Goal<'info>  {
    #[account(mut)]
    pub workouts: Account<'info, Workouts>,
}

#[account]
pub struct Workouts {
    pub greeting: String,
    pub remaining: i64,
}