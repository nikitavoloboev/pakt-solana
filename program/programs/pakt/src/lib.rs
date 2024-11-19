use anchor_lang::prelude::*;

declare_id!("AdhyDTom3FbdoFA2RkfCtKdy95wUtd9rAHQU7oAe9HHS");

#[program]
pub mod pakt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
