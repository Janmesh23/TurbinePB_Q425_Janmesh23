use anchor_lang::prelude::*;

declare_id!("bALNiReK9MR7YnAsdorbLLqGZADMqQvfQ24rSbcK9uj");

#[program]
pub mod counter {
    use super::*;
 
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account;
        counter.count = 0;
        msg!("Counter account initialized! Current count: {}", counter.count);
        Ok(())
    }
 
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account;
            
        match counter.count.checked_add(1) {
            Some(new_count) => {
                counter.count = new_count;
            }
            None => {
                return err!(CounterError::CannotIncrementOverflow);
            }
        }       
        msg!("Counter incremented! New count: {}", counter.count);
        Ok(())
    }  
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account;
       
        match counter.count.checked_sub(1) {
            Some(new_count) => {
                counter.count = new_count;
            }
            None => {
                return err!(CounterError::CannotDecrementFromZero);
            }
        }      
        msg!("Counter decremented! New count: {}", counter.count);
        Ok(())
    }
   
    pub fn set_zero(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter_account;
        counter.count = 0;
        msg!("Counter reset to zero!");
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8,
        seeds = [b"counter".as_ref(), user.key().as_ref()],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut, 
        seeds = [b"counter".as_ref(), user.key().as_ref()],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>, 
}


#[account]
pub struct CounterAccount {
    pub count: u64,
}


#[error_code]
pub enum CounterError {
    #[msg("Cannot increment the counter, it would overflow.")]
    CannotIncrementOverflow,
    #[msg("Cannot decrement the counter, it's already at zero.")]
    CannotDecrementFromZero,
}