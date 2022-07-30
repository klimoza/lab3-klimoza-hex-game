The Game of Hex
===============
Hex is a two player abstract strategy board game in which players attempt to connect opposite sides of a rhombus-shaped board made of hexagonal cells. Hex was invented by mathematician and poet Piet Hein in 1942 and later rediscovered and popularized by John Nash. 

## Description
This contract implements hex-game backed by storage on NEAR blockchain.
Contract in `src/lib.rs` provides methods to create new game, make an allowed move in one of already existing games or view information about game by index.

The project is divided into separate files, each file contains one of the structures, that is used to keep information about the game, and tests for this structure.

## Interacting with contract

#### `create_game(first_player: AccountId, second_player: AccountId, field_size: usize) -> GameIndex`

Creates new game with given parameters and returns index of created game. For example:
```console
➜ near call crossword.klimoza.testnet create_game '{"first_player": "crossword.klimoza.testnet", "second_player": "klimoza.testnet", "field_size": 2}' --accountId crossword.klimoza.testnet
Scheduling a call: crossword.klimoza.testnet.create_game({"first_player": "crossword.klimoza.testnet", "second_player": "klimoza.testnet", "field_size": 2})
Doing account.functionCall()
Receipt: 95QtZWq7chRA4MyftnkPE8Smoi7iQo9kWpeVfxFZGDEV
	Log [crossword.klimoza.testnet]: Created board:
	Log [crossword.klimoza.testnet]: . .
	Log [crossword.klimoza.testnet]:  . .
Transaction Id BHvhLbLWHhKbmnewLHzMv7XY2y5izssY9spWEdVm6wUE
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/BHvhLbLWHhKbmnewLHzMv7XY2y5izssY9spWEdVm6wUE
4
```

#### `make_move(index: GameIndex, move_type: MoveType, cell: Option<Cell>) -> Game`
Tries to make a move in the game at the given index and returns Game if move is correct(panics otherwise). Used structures:
```rust
pub type GameIndex = u64;

pub enum MoveType {
    PLACE,
    SWAP,
}

pub struct Cell {
    pub x: usize,
    pub y: usize,
}
```
You can omit the `cell` parameter if `move_type` is `SWAP`(i.e. applying swap rule on the current move). For example:
```console
➜ near call crossword.klimoza.testnet make_move '{"index": 4, "move_type": "SWAP"}' --accountId klimoza.testnet
Scheduling a call: crossword.klimoza.testnet.make_move({"index": 4, "move_type": "SWAP"})
Doing account.functionCall()
Receipt: 9SntyX6t9j8jZCZMNb5wMn82k7YQKXoWH8tuYbaNZQs6
	Log [crossword.klimoza.testnet]: Old board:
	Log [crossword.klimoza.testnet]: . R
	Log [crossword.klimoza.testnet]:  . .
	Log [crossword.klimoza.testnet]: New board:
	Log [crossword.klimoza.testnet]: . .
	Log [crossword.klimoza.testnet]:  B .
Transaction Id FqsxYTzGaHYXvmxk3xYhBR3Mq4rveK5UvzkMNNerL6Th
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/FqsxYTzGaHYXvmxk3xYhBR3Mq4rveK5UvzkMNNerL6Th
{
  first_player: 'crossword.klimoza.testnet',
  second_player: 'klimoza.testnet',
  turn: 2,
  board: { size: 2, field: 'IA==' },
  current_block_height: 96244955,
  prev_block_height: 96244934,
  is_finished: false
}
```

#### `get_game(index: GameIndex) -> Option<Game>`
Returns the game at the given index(if there is one). For example:
```console
➜ near call crossword.klimoza.testnet get_game '{"index": 4}' --accountId crossword.klimoza.testnet
Scheduling a call: crossword.klimoza.testnet.get_game({"index": 4})
Doing account.functionCall()
Receipt: Fzjemr6ukV7mWNLNaJLrjxwU7uwnz2wgLfcNRNvLMyEH
	Log [crossword.klimoza.testnet]: Game board:
	Log [crossword.klimoza.testnet]: R B
	Log [crossword.klimoza.testnet]:  B .
Transaction Id DwpznqSWT8WqsirqFm64e8xHPEu1S3J52FpKsAR4okAQ
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/DwpznqSWT8WqsirqFm64e8xHPEu1S3J52FpKsAR4okAQ
{
  first_player: 'crossword.klimoza.testnet',
  second_player: 'klimoza.testnet',
  turn: 4,
  board: { size: 2, field: 'KQ==' },
  current_block_height: 96244985,
  prev_block_height: 96244971,
  is_finished: true
}
```

## Testing
At the moment, the projects contains 33 tests, each of which lies in the file of the structure it is testing. You can run them all using following command:
```console
cargo test
```
Alternatively, you can specify test group you want to run, for example:
```console
cargo test cell_tests
```

## Demonstration

[![Video](https://img.youtube.com/vi/mwgUEafpeow/0.jpg)](https://youtu.be/mwgUEafpeow)