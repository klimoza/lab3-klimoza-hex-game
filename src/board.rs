use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::require;
use near_sdk::serde::{Deserialize, Serialize};

use crate::cell::Cell;

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
            field: Base64VecU8::from(vec![0u8; field_len]),
        }
    }

    pub fn get_cell(&self, cell: &Cell) -> u8 {
        require!(
            cell.x < self.size && cell.y < self.size,
            "Cell is out of bounds."
        );
        let index = (self.size * cell.y + cell.x) * 2;
        let byte_index = index / 8;
        let byte: u8 = self.field.0[byte_index];
        let bit_index = index & 7;
        ((byte >> bit_index) & 1) + ((byte >> (bit_index + 1)) & 1) * 2
    }

    pub fn set_cell(&mut self, cell: &Cell, value: u8) {
        require!(
            cell.x < self.size && cell.y < self.size,
            "Cell is out of bounds."
        );
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
        Self {
            size: self.size.clone(),
            field: self.field.clone(),
        }
    }
}
