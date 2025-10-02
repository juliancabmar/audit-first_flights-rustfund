use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq3ZYk3u7dKZ7fNjcJjDkP1y");

#[program]
pub mod minimal_counter {
    use super::*;

    // Inicializa la cuenta con valor 0
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter.value = 0;
        Ok(())
    }

    // Actualiza el valor de la cuenta
    pub fn set_value(ctx: Context<SetValue>, new_value: u64) -> Result<()> {
        ctx.accounts.counter.value = new_value;
        Ok(())
    }
}

// Contexto para inicializar
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Contexto para actualizar valor
#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// Struct persistente
#[account]
pub struct Counter {
    pub value: u64,
}
