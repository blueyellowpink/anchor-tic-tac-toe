use anchor_lang::prelude::*;

use crate::state::Game;

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one, space = 8 + Game::MAXIMUM_SIZE)]
    pub game: Account<'info, Game>,

    #[account(mut)]
    pub player_one: Signer<'info>,

    pub system_program: Program<'info, System>,
}
