use anchor_lang::prelude::*;

declare_id!("3Qvio5Ky2VN2vBdyeo3263GZpQHhGN9bEFkNJxLBXa5S");

#[program]
pub mod anchor_counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        msg!("Counter initialized at 0"); 
        Ok(())
    }

    
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        
      
        match counter_account.count.checked_add(1) {
            Some(new_count) => {
                counter_account.count = new_count;
            }
            None => {
                return err!(CounterError::Overflow);
            }
        }
        
       
        Ok(())
    }

    
    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;

        
        match counter_account.count.checked_sub(1) {
            Some(new_count) => {
                counter_account.count = new_count;
            }
            None => {
                return err!(CounterError::Underflow);
            }
        }
        
       
        Ok(())
    }

    pub fn set_zero(ctx: Context<Update>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        msg!("Counter reset to 0");
        Ok(())
    }

    pub fn get_count(ctx: Context<GetCount>) -> Result<()> {
        let counter_account = &ctx.accounts.counter_account;
        msg!("Current count is: {}", counter_account.count);
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        // 8 bytes for discriminator + size of the CounterAccount struct
        space = 8 + std::mem::size_of::<CounterAccount>(),
        seeds = [b"counterAccount", user.key().as_ref()],
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
        seeds = [b"counterAccount", user.key().as_ref()],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>, 
}


#[derive(Accounts)]
pub struct GetCount<'info> {
    #[account(
        seeds = [b"counterAccount", user.key().as_ref()],
        bump
    )]
    pub counter_account: Account<'info, CounterAccount>,
    pub user: Signer<'info>,
}

// 4. Account Struct:
// Renamed to `CounterAccount` (PascalCase) as is
// standard Rust convention.
#[account]
pub struct CounterAccount {
    pub count: i64, // Using i64 as in your example
}

// 5. Custom Errors:
// This is much safer than just `+= 1` or `-= 1`, which can
// panic the whole program on overflow/underflow.
#[error_code]
pub enum CounterError {
    #[msg("Counter would overflow")]
    Overflow,
    #[msg("Counter would underflow")]
    Underflow,
}
