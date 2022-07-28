use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, AccountId};
use std::collections::VecDeque;

use crate::board::Board;
use crate::cell::Cell;
use crate::game::Game;
use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameWithData {
    pub game: Game,
    pub data: Board,
}

impl GameWithData {
    pub fn new(first_player: AccountId, second_player: AccountId, field_size: usize) -> Self {
        Self {
            game: Game::new(first_player, second_player, field_size),
            data: Board::new(field_size),
        }
    }
    pub fn make_move(&mut self, move_type: MoveType, cell: Option<Cell>) {
        let first_player = &self.game.first_player;
        let second_player = &self.game.second_player;
        let acc = &env::predecessor_account_id();
        match (acc == first_player, acc == second_player, move_type, cell) {
            (true, false, MoveType::PLACE, Some(cell)) => {
                self.game.first_player_move(&cell);
                self.process_cell(cell);
            }
            (false, true, MoveType::PLACE, Some(cell)) => {
                self.game.second_player_move(&cell);
                self.process_cell(cell);
            }
            (false, true, MoveType::SWAP, _) => {
                let cell = self.game.swap_rule();
                self.data.set_cell(&cell, 0);
                self.process_cell(cell.symm());
            }
            _ => env::panic_str("Incorrect predecessor account, or incorrect move type."),
        }
    }
    pub fn process_cell(&mut self, cell: Cell) {
        let color = self.game.board.get_cell(&cell);
        let (mut border1, mut border2) = if color == 1 {
            (cell.y == 0, cell.y + 1 == self.data.size)
        } else {
            (cell.x == 0, cell.x + 1 == self.data.size)
        };
        let neighbours = cell.get_neighbours(self.data.size);
        let good_neighbours = neighbours
            .iter()
            .filter(|c| self.game.board.get_cell(c) == color);
        border1 = border1 || good_neighbours.clone().any(|c| self.data.get_cell(c) == 1);
        border2 = border2 || good_neighbours.clone().any(|c| self.data.get_cell(c) == 2);
        if border1 && border2 {
            self.game.is_finished = true;
        } else if border1 {
            self.bfs(cell, color, 1);
        } else if border2 {
            self.bfs(cell, color, 2);
        }
    }
    pub fn bfs(&mut self, cell: Cell, color: u8, border: u8) {
        self.data.set_cell(&cell, border);
        let mut q: VecDeque<Cell> = VecDeque::new();
        q.push_back(cell);
        let field_size = self.data.size;
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            let neighbours = v.get_neighbours(field_size);
            let good_neighbours: Vec<Cell> = neighbours
                .into_iter()
                .filter(|c| self.game.board.get_cell(c) == color && self.data.get_cell(c) != border)
                .collect();
            if good_neighbours
                .clone()
                .into_iter()
                .any(|c| self.data.get_cell(&c) > 0)
            {
                self.game.is_finished = true;
                return;
            }
            for c in good_neighbours.into_iter() {
                self.data.set_cell(&c, border);
                q.push_back(Cell { x: c.x, y: c.y });
            }
        }
    }
}
