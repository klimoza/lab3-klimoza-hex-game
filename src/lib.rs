use near_sdk::collections::{Vector, LookupMap};
use near_sdk::{require, near_bindgen, PanicOnDefault, BlockHeight, env, BorshStorageKey, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{self, Serialize, Deserialize};
use near_sdk::json_types::Base64VecU8;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Board {
    pub size: u8,
    pub field: Base64VecU8,
}

impl Board {
    pub fn new(size: usize) -> Self {
        require!(size <= 19, "The size of the field must be less or equal 19");
        let field_len = (size * size + 3) / 4;
        Board {
            size: size as u8,
            field: Base64VecU8::from(vec![0u8; field_len])
        }
    }

    pub fn get_cell(&self, cell: Cell) -> u8 {
        require!(cell.x < self.size as usize && cell.y < self.size as usize, "Cell is out of bounds.");
        let index = (self.size as usize * cell.y + cell.x) * 2;
        let byte_index = index / 8;
        let byte: u8 = self.field.0[byte_index];
        let bit_index = index & 7;
        ((byte >> bit_index) & 1) + ((byte >> (bit_index + 1)) & 1) * 2
    }

    pub fn set_cell(&mut self, cell: Cell, value: u8) {
        require!(cell.x < self.size as usize && cell.y < self.size as usize, "Cell is out of bounds.");
        require!(value <= 2, "Value is too big.");
        let index = (self.size as usize * cell.y + cell.x) * 2;
        let byte_index = index / 8;
        let byte: u8 = self.field.0[byte_index];
        let bit_index = index & 7;
        let bits = (byte >> bit_index) & 3;
        let new_byte = byte ^ (bits << bit_index) ^ (value << bit_index);
        self.field.0[byte_index] = new_byte;
    }

    pub fn get_coords(&self, bit_number: usize) -> (usize, usize) {
        (bit_number / 2 % self.size as usize, bit_number / 2 / self.size as usize)
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

    pub fn first_player_move(&self, cell: Cell) -> Game {
        require!(self.board.get_cell(cell) == 0, "Cell is already filled.");
        require!(self.turn & 1 == 0, "It's second player turn now.");
        let mut new_board = self.board.clone();
        new_board.set_cell(cell, 1);
        let mut new_game = 
            Game {
                first_player: self.first_player,
                second_player: self.second_player,
                turn: self.turn + 1,
                board: new_board,
                current_block_height: self.current_block_height,
                prev_block_height: self.prev_block_height,
                is_finished: false
            };
        if env::block_height() != self.current_block_height {
            new_game.prev_block_height = self.current_block_height;
            new_game.current_block_height = env::block_height();
        }
        new_game
    }

    pub fn second_player_move(&self, cell: Cell) -> Game {
        require!(self.board.get_cell(cell) == 0, "Cell is already filled.");
        require!(self.turn & 1 == 1, "It's first player turn now.");
        let new_board = self.board.clone();
        new_board.set_cell(cell, 2);
        let mut new_game = 
            Game {
                first_player: self.first_player,
                second_player: self.second_player,
                turn: self.turn + 1,
                board: new_board,
                current_block_height: self.current_block_height,
                prev_block_height: self.prev_block_height,
                is_finished: false
            };
        if env::block_height() != self.current_block_height {
            new_game.prev_block_height = self.current_block_height;
            new_game.current_block_height = env::block_height();
        }
        new_game
    }

    pub fn swap_rule(&self) -> (Game, usize, usize) {
        require!(self.turn == 1, "Swap rule can be applied only on the second player first turn");
        let non_zero_byte = self.board.field.0.iter().enumerate().find(|(i, &x)| x != 0).unwrap();
        let bit_number = 8 * non_zero_byte.0;
        if (non_zero_byte.1 >> 2) & 1 == 1 {
            bit_number += 2;
        } else if (non_zero_byte.1 >> 4) & 1 == 1 {
            bit_number += 4;
        } else if (non_zero_byte.1 >> 6) & 1 == 1 {
            bit_number += 6;
        }
        let (x, y) = self.board.get_coords(bit_number);
        let new_board = self.board.clone();
        new_board.set_cell(Cell {x, y}, 0);
        new_board.set_cell(Cell {y, x}, 2);
        let mut new_game = 
            Game {
                first_player: self.first_player,
                second_player: self.second_player,
                turn: self.turn + 1,
                board: new_board,
                current_block_height: self.current_block_height,
                prev_block_height: self.prev_block_height,
                is_finished: false
            };
        if env::block_height() != self.current_block_height {
            new_game.prev_block_height = self.current_block_height;
            new_game.current_block_height = env::block_height();
        }
        (new_game, x, y)
    }
}

pub type GameIndex = u64;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Data {
    pub field: Vector<Base64VecU8>,
}


#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Games,
    Field { game_id: GameIndex }
}

impl Data {
    pub fn new(field_size: usize, game_id: GameIndex) -> Self {
        require!(field_size <= 19, "The size of the field must be less or equal 19");
        let row_len = (field_size + 3) / 4;
        let field = Vector::new(StorageKey::Field { game_id });
        for _ in 0..field_size {
            field.push(&Base64VecU8::from(vec![0u8; row_len]));
        }
        Self { 
            field,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GameWithData {
    pub game: Game,
    pub data: Data
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cell {
    pub x: usize,
    pub y: usize
}

impl GameWithData {
    pub fn new(first_player: AccountId, second_player: AccountId, field_size: usize, game_id: GameIndex) -> Self {
        Self { 
            game: Game::new(first_player, second_player, field_size), 
            data: Data::new(field_size, game_id) 
        }
    }
    pub fn make_move(&mut self, move_type: MoveType, cell: Option<Cell>){
        let first_player = self.game.first_player;
        let second_player = self.game.second_player;
        match (env::predecessor_account_id(), move_type, cell) {
            (first_player, MoveType::PLACE, Some(cell)) => { 
                let new_game = self.game.first_player_move(cell);
            }
            (second_player, MoveType::PLACE, Some(cell)) => {
                self.game.second_player_move(cell);
            }
            (second_player, MoveType::SWAP, _) => {
                self.game.swap_rule();
            }
            _ => env::panic_str("Incorrect predecessor account, or incorrect move type.")
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
        self.games.push(&GameWithData::new(first_player, second_player, size, index));
        index
    }

    pub fn get_game(&mut self, index: GameIndex) -> Option<Game> {
        self.games.get(index).map(|x| x.game)
    }

    pub fn make_move(&mut self, index: GameIndex, move_type: MoveType, cell: Option<Cell>) -> Game {
        let new_game = self.games.get(index).expect("Game doesn't exist.").make_move(move_type, cell);
    }
}