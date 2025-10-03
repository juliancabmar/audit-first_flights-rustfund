use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq3ZYk3u7dKZ7fNjcJjDkP1y");

#[program]
pub mod fundme {
    use super::*;

    // Función para depositar SOL
    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        let fund_account = &mut ctx.accounts.fund_account;

        // Transferir lamports desde el contribuyente a la cuenta del fondo
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.contributor.to_account_info(),
                to: fund_account.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, amount)?;

        // Actualizar el total recaudado
        fund_account.total += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Fund<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,  // quien envía los fondos

    #[account(init_if_needed, payer = contributor, space = 8 + 8, seeds = [b"fund"], bump)]
    pub fund_account: Account<'info, FundAccount>,  // PDA que almacena total

    pub system_program: Program<'info, System>,
}

#[account]
pub struct FundAccount {
    pub total: u64, // total recaudado
}
