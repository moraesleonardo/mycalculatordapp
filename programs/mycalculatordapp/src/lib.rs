use anchor_lang::prelude::*;

declare_id!("9LU6qKkiwCXMFLJPY3f8Mtmu5f6hVWL325MEiLSzTgHJ");

#[program]
pub mod mycalculatordapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
