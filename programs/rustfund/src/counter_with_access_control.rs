use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq3ZYk3u7dKZ7fNjcJjDkP1y");

#[program]
pub mod owner_counter {
    use super::*;

    // Inicializa la cuenta con valor 0 y guarda el owner
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.value = 0;
        counter.owner = ctx.accounts.user.key();  // asigna owner
        Ok(())
    }

    // Actualiza el valor de la cuenta solo si es el owner
    pub fn set_value(ctx: Context<SetValue>, new_value: u64) -> Result<()> {
        let counter = &ctx.accounts.counter;

        // Control de acceso
        if ctx.accounts.user.key() != counter.owner {
            return Err(ErrorCode::UnauthorizedAccess.into());
        }

        ctx.accounts.counter.value = new_value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)] // 8 bytes de discriminator + 8 de value + 32 de owner
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,  // usuario que intenta cambiar el contador
}

#[account]
pub struct Counter {
    pub value: u64,
    pub owner: Pubkey, // control de acceso
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    UnauthorizedAccess,
}
