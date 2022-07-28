use cell::Cell;
use game::{Game, GameIndex};
use game_with_data::GameWithData;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Games,
    Field { game_id: GameIndex },
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum MoveType {
    PLACE,
    SWAP,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub games: Vector<GameWithData>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            games: Vector::new(StorageKey::Games),
        }
    }

    pub fn create_game(
        &mut self,
        first_player: AccountId,
        second_player: AccountId,
        field_size: Option<usize>,
    ) -> GameIndex {
        let index = self.games.len();
        let size = field_size.unwrap_or(11);
        self.games
            .push(&GameWithData::new(first_player, second_player, size));
        index
    }

    pub fn get_game(&mut self, index: GameIndex) -> Option<Game> {
        self.games.get(index).map(|x| x.game)
    }

    pub fn make_move(&mut self, index: GameIndex, move_type: MoveType, cell: Option<Cell>) -> Game {
        self.games
            .get(index)
            .expect("Game doesn't exist.")
            .make_move(move_type, cell);
        return self.games.get(index).unwrap().game.clone();
    }
}

pub mod board;
pub mod cell;
pub mod game;
pub mod game_with_data;
