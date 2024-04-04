use anchor_lang::prelude::*;

declare_id!("F7uwK9BXr7orrQWJujdoTVbd7P7yMDnPA84udYYyPPLM");

#[program]
pub mod rust_program_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
