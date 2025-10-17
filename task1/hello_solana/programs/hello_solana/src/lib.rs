use anchor_lang::prelude::*;

declare_id!("2bAioe7nm9AJX6A1KVJA44GcP3LkKPHMaHDcosjGqGnQ");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}




