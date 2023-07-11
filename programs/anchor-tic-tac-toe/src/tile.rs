use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Tile {
    pub row: u8,
    pub column: u8,
}
