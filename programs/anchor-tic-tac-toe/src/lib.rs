mod errors;
mod instructions;
mod state;

use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;
use crate::instructions::*;

declare_id!("4LW9DrrBZDdVvo3Jm674zN2hbs1vaKnjQ96TD2dC36KC");

#[program]
pub mod anchor_tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        ctx.accounts
            .game
            .start([ctx.accounts.player_one.key(), player_two])
    }

    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        let game = &mut ctx.accounts.game;

        require_keys_eq!(
            game.current_player(),
            ctx.accounts.player.key(),
            TicTacToeError::NotPlayersTurn
        );

        game.play(&tile)
    }
}
