use anchor_lang::prelude::*;

declare_id!("HLsVdYjbk8hPVxg5tZWAJ8uJzy5HBAxKLeSG5gnRiZAp");

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
