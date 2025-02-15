use anchor_lang::prelude::*;

declare_id!("8h3EdPTn9XEpt41reBbDpoH8iErKwX99cPrk4iCqmM79");

pub mod contexts;
pub mod state;
pub mod errors;

#[program]
pub mod dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
