use std::collections::VecDeque;

use near_sdk::collections::{Vector};
use near_sdk::{require, near_bindgen, PanicOnDefault, BlockHeight, env, BorshStorageKey, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::Base64VecU8;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Board {
    pub size: usize,
    pub field: Base64VecU8,
}

impl Board {
    pub fn new(size: usize) -> Self {
        require!(size <= 19, "The size of the field must be less or equal 19");
        let field_len = (size * size + 3) / 4;
        Board {
            size: size,
            field: Base64VecU8::from(vec![0u8; field_len])
        }
    }

    pub fn get_cell(&self, cell: &Cell) -> u8 {
        require!(cell.x < self.size && cell.y < self.size, "Cell is out of bounds.");
        let index = (self.size * cell.y + cell.x) * 2;
        let byte_index = index / 8;
        let byte: u8 = self.field.0[byte_index];
        let bit_index = index & 7;
        ((byte >> bit_index) & 1) + ((byte >> (bit_index + 1)) & 1) * 2
    }

    pub fn set_cell(&mut self, cell: &Cell, value: u8) {
        require!(cell.x < self.size && cell.y < self.size, "Cell is out of bounds.");
        require!(value <= 2, "Value is too big.");
        let index = (self.size * cell.y + cell.x) * 2;
        let byte_index = index / 8;
        let byte: u8 = self.field.0[byte_index];
        let bit_index = index & 7;
        let bits = (byte >> bit_index) & 3;
        let new_byte = byte ^ (bits << bit_index) ^ (value << bit_index);
        self.field.0[byte_index] = new_byte;
    }

    pub fn get_coords(&self, bit_number: usize) -> (usize, usize) {
        (bit_number / 2 % self.size, bit_number / 2 / self.size)
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self { size: self.size.clone(), field: self.field.clone() }
    }
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub first_player: AccountId,
    pub second_player: AccountId,
    pub turn: u16,
    pub board: Board,
    pub current_block_height: BlockHeight,
    pub prev_block_height: BlockHeight,
    pub is_finished: bool
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
            is_finished: false
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

    pub fn swap_rule(&mut self) -> (usize, usize) {
        require!(self.turn == 1, "Swap rule can be applied only on the second player first turn");
        let non_zero_byte = self.board.field.0.iter().enumerate().find(|(_, &x)| x != 0).unwrap();
        let mut bit_number = 8 * non_zero_byte.0;
        if (non_zero_byte.1 >> 2) & 1 == 1 {
            bit_number += 2;
        } else if (non_zero_byte.1 >> 4) & 1 == 1 {
            bit_number += 4;
        } else if (non_zero_byte.1 >> 6) & 1 == 1 {
            bit_number += 6;
        }
        let (x, y) = self.board.get_coords(bit_number);
        self.board.set_cell(&Cell {x, y}, 0);
        self.board.set_cell(&Cell {y, x}, 2);
        if env::block_height() != self.current_block_height {
            self.prev_block_height = self.current_block_height;
            self.current_block_height = env::block_height();
        }
        (x, y)
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Self { first_player: self.first_player.clone(), second_player: self.second_player.clone(), turn: self.turn.clone(), board: self.board.clone(), current_block_height: self.current_block_height.clone(), prev_block_height: self.prev_block_height.clone(), is_finished: self.is_finished.clone() }
    }
}

pub type GameIndex = u64;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Games,
    Field { game_id: GameIndex }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameWithData {
    pub game: Game,
    pub data: Board
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cell {
    pub x: usize,
    pub y: usize
}

impl Cell {
    pub fn get_neighbours(&self, field_size: usize) -> Vec<Cell> {
        let mut neighbours: Vec<Cell> = Vec::new();
        let (x, y) = (self.x, self.y);
        if self.x > 0 {
            neighbours.push(Cell { x: x - 1, y});
            if self.y + 1 < field_size {
                neighbours.push(Cell {x : x - 1, y: y + 1});
            }
        }
        if self.y + 1 < field_size {
            neighbours.push(Cell {x, y: y + 1});
        }
        if self.x + 1 < field_size {
            neighbours.push(Cell {x: x + 1, y});
            if self.y > 0 {
                neighbours.push(Cell {x: x + 1, y: y - 1});
            }
        }
        if self.y > 0 {
            neighbours.push(Cell {x, y: y - 1});
        }
        return neighbours;
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}

impl GameWithData {
    pub fn new(first_player: AccountId, second_player: AccountId, field_size: usize) -> Self {
        Self { 
            game: Game::new(first_player, second_player, field_size), 
            data: Board::new(field_size)
        }
    }
    pub fn make_move(&mut self, move_type: MoveType, cell: Option<Cell>){
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
                let (x, y) = self.game.swap_rule();
                self.data.set_cell(&Cell {x, y}, 0);
                self.process_cell(Cell {y, x});
            }
            _ => env::panic_str("Incorrect predecessor account, or incorrect move type.")
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
        let good_neighbours = neighbours.iter().filter(|c| self.game.board.get_cell(c) == color);
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
            let good_neighbours: Vec<Cell> = neighbours.into_iter().filter(|c| self.game.board.get_cell(c) == color && self.data.get_cell(c) != border).collect();
            if good_neighbours.clone().into_iter().any(|c| self.data.get_cell(&c) > 0) {
                self.game.is_finished = true;
                return;
            }
            for c in good_neighbours.into_iter() {
                self.data.set_cell(&c, border);
                q.push_back(Cell {x: c.x, y: c.y});
            }
        }
    }
}


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum MoveType {
    PLACE,
    SWAP
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub games: Vector<GameWithData>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { games: Vector::new(StorageKey::Games) }
    }

    pub fn create_game(&mut self, first_player: AccountId, second_player: AccountId, field_size: Option<usize>) -> GameIndex {
        let index = self.games.len();
        let size = field_size.unwrap_or(11);
        self.games.push(&GameWithData::new(first_player, second_player, size));
        index
    }

    pub fn get_game(&mut self, index: GameIndex) -> Option<Game> {
        self.games.get(index).map(|x| x.game)
    }

    pub fn make_move(&mut self, index: GameIndex, move_type: MoveType, cell: Option<Cell>) -> Game {
        self.games.get(index).expect("Game doesn't exist.").make_move(move_type, cell);
        return self.games.get(index).unwrap().game.clone();
    }
}