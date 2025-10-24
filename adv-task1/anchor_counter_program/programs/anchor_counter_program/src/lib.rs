use anchor_lang::prelude::*;

declare_id!("3Qvio5Ky2VN2vBdyeo3263GZpQHhGN9bEFkNJxLBXa5S");

#[program]
pub mod anchor_counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
