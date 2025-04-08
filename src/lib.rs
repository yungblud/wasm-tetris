use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

const WIDTH: i32 = 10;
const HEIGHT: i32 = 20;
const MOVE_UNIT: f32 = 0.05;

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub enum PieceKind {
    I,
    L,
    T,
}

#[derive(Clone)]
pub struct Piece {
    pub blocks: Vec<Block>,
    pub x: f32,
    pub y: f32,
    pub kind: PieceKind,
}

impl Piece {
    pub fn new(kind: PieceKind) -> Self {
        let blocks = match kind {
            PieceKind::I => vec![
                Block { x: 0.0, y: 0.0 },
                Block { x: 0.0, y: 1.0 },
                Block { x: 0.0, y: 2.0 },
                Block { x: 0.0, y: 3.0 },
            ],
            PieceKind::L => vec![
                Block { x: 0.0, y: 0.0 },
                Block { x: 0.0, y: 1.0 },
                Block { x: 0.0, y: 2.0 },
                Block { x: 1.0, y: 2.0 },
            ],
            PieceKind::T => vec![
                Block { x: 0.0, y: 0.0 },
                Block { x: 1.0, y: 0.0 },
                Block { x: 2.0, y: 0.0 },
                Block { x: 1.0, y: 1.0 },
            ],
        };

        Self {
            blocks,
            x: 5.0,
            y: 18.0,
            kind,
        }
    }

    pub fn translated_blocks(&self) -> Vec<Block> {
        self.blocks.iter().map(|b| Block {
            x: b.x + self.x,
            y: b.y + self.y,
        }).collect()
    }
}

#[wasm_bindgen]
pub struct GameState {
    blocks: Vec<Block>,
    current_piece: Piece,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            current_piece: Piece::new(PieceKind::T),
        }
    }

    pub fn update(&mut self) {
        self.current_piece.y -= MOVE_UNIT;

        let translated = self.current_piece.translated_blocks();
        for b in &translated {
            if b.y < 0.0 {
                self.blocks.extend(translated.clone());
                self.clear_full_rows();
                self.current_piece = Piece::new(PieceKind::L); // 다음 블록
                break;
            }
        }
    }

    pub fn move_left(&mut self) {
        self.current_piece.x -= MOVE_UNIT;
    }

    pub fn move_right(&mut self) {
        self.current_piece.x += MOVE_UNIT;
    }

    pub fn move_down(&mut self) {
        self.current_piece.y -= MOVE_UNIT;
    }

    pub fn blocks(&self) -> JsValue {
        JsValue::from_serde(&self.blocks).unwrap()
    }

    pub fn current(&self) -> JsValue {
        let translated = self.current_piece.translated_blocks();
        JsValue::from_serde(&translated).unwrap()
    }

    fn clear_full_rows(&mut self) {
        use std::collections::HashMap;

        let mut row_counts = HashMap::new();
        for b in &self.blocks {
            let y = b.y.floor() as i32;
            *row_counts.entry(y).or_insert(0) += 1;
        }

        let full_rows: Vec<i32> = row_counts
            .iter()
            .filter(|(_, count)| **count >= WIDTH)
            .map(|(y, _)| *y)
            .collect();

        if full_rows.is_empty() {
            return;
        }

        self.blocks.retain(|b| {
            let y = b.y.floor() as i32;
            !full_rows.contains(&y)
        });

        for row in full_rows.iter() {
            for b in &mut self.blocks {
                if b.y.floor() as i32 > *row {
                    b.y -= 1.0;
                }
            }
        }
    }
}
