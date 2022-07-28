use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn get_neighbours(&self, field_size: usize) -> Vec<Cell> {
        let mut neighbours: Vec<Cell> = Vec::new();
        let (x, y) = (self.x, self.y);
        if self.x > 0 {
            neighbours.push(Cell { x: x - 1, y });
            if self.y + 1 < field_size {
                neighbours.push(Cell { x: x - 1, y: y + 1 });
            }
        }
        if self.y + 1 < field_size {
            neighbours.push(Cell { x, y: y + 1 });
        }
        if self.x + 1 < field_size {
            neighbours.push(Cell { x: x + 1, y });
            if self.y > 0 {
                neighbours.push(Cell { x: x + 1, y: y - 1 });
            }
        }
        if self.y > 0 {
            neighbours.push(Cell { x, y: y - 1 });
        }
        return neighbours;
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
