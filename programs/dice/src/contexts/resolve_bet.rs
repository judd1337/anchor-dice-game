use anchor_lang::{prelude::*, solana_program::{self, ed25519_program, hash, sysvar::instructions::load_instruction_at_checked}};

use crate::state::Bet;

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    
    #[account(mut)]
    pub player: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    pub instructions_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(
            0, 
            &self.instructions_sysvar.to_account_info()
        )?;

        // TODO: listen to video again why we have this?
        require_keys_eq!(ix.program_id, ed25519_program::ID, CustomError::CustomError);
        
        // TODO: listen to video again why we have this?
        require_eq!(ix.accounts.len(), 0, CustomError::CustomError);

        // Get the first index
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, CustomError::CustomError);
        let signature_in_program = &signatures[0];

        require!(signature_in_program.is_verifiable, CustomError::CustomError);

        require_keys_eq!(
            signature_in_program.public_key.unwarp(),
            self.house.key(),
            CustomError::CustomError
        );

        require!(
            signature_in_program.signature.unwrap().eq(sig),
            CustomError::CustomError
        );

        require!(
            signature_in_program.message.as_ref().unwrap().eq(&self.bet.to_slice()),
            CustomError::CustomError
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash = hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // We will get a number between 1 - 100
        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payout = (self.bet.amount as u128)
                .checked_mul(10000 - 150 as u128)
                .unwrap()
                .checked_div(self.bet.roll as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap() as u64;
        }

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [b"vault", &self.house.key().to_bytes()[..], &[bumps.vault]];

        let signer_seeds = &[&seeds[..]][..];

        // TODO: finish this code
        //let cpi_context = CpiContext::Transfer{
        //}

        Ok(())
    }
}

