use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, require, AccountId, BlockHeight};

use crate::board::Board;
use crate::cell::Cell;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub first_player: AccountId,
    pub second_player: AccountId,
    pub turn: u16,
    pub board: Board,
    pub current_block_height: BlockHeight,
    pub prev_block_height: BlockHeight,
    pub is_finished: bool,
}

impl Game {
    pub fn new(first_player: AccountId, second_player: AccountId, field_size: usize) -> Self {
        Game {
            first_player,
            second_player,
            turn: 0,
            board: Board::new(field_size),
            current_block_height: env::block_height(),
            prev_block_height: 0,
            is_finished: false,
        }
    }

    pub fn first_player_move(&mut self, cell: &Cell) {
        require!(self.board.get_cell(cell) == 0, "Cell is already filled.");
        require!(self.turn & 1 == 0, "It's second player turn now.");
        self.board.set_cell(cell, 1);
        self.turn += 1;
        if env::block_height() != self.current_block_height {
            self.prev_block_height = self.current_block_height;
            self.current_block_height = env::block_height();
        }
    }

    pub fn second_player_move(&mut self, cell: &Cell) {
        require!(self.board.get_cell(cell) == 0, "Cell is already filled.");
        require!(self.turn & 1 == 1, "It's first player turn now.");
        self.board.set_cell(cell, 2);
        self.turn += 1;
        if env::block_height() != self.current_block_height {
            self.prev_block_height = self.current_block_height;
            self.current_block_height = env::block_height();
        }
    }

    pub fn swap_rule(&mut self) -> Cell {
        require!(
            self.turn == 1,
            "Swap rule can be applied only on the second player first turn"
        );
        let non_zero_byte = self
            .board
            .field
            .0
            .iter()
            .enumerate()
            .find(|(_, &x)| x != 0)
            .unwrap();
        let mut bit_number = 8 * non_zero_byte.0;
        if (non_zero_byte.1 >> 2) & 1 == 1 {
            bit_number += 2;
        } else if (non_zero_byte.1 >> 4) & 1 == 1 {
            bit_number += 4;
        } else if (non_zero_byte.1 >> 6) & 1 == 1 {
            bit_number += 6;
        }
        let cell = self.board.get_coords(bit_number);
        self.board.set_cell(&cell, 0);
        self.board.set_cell(&cell.symm(), 2);
        if env::block_height() != self.current_block_height {
            self.prev_block_height = self.current_block_height;
            self.current_block_height = env::block_height();
        }
        cell
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self {
            first_player: self.first_player.clone(),
            second_player: self.second_player.clone(),
            turn: self.turn.clone(),
            board: self.board.clone(),
            current_block_height: self.current_block_height.clone(),
            prev_block_height: self.prev_block_height.clone(),
            is_finished: self.is_finished.clone(),
        }
    }
}

pub type GameIndex = u64;
