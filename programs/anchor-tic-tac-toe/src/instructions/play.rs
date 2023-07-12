
use anchor_lang::prelude::*;

use crate::state::Game;

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,

    pub player: Signer<'info>,
}
