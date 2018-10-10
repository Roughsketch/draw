use ggez::{Context, GameResult};
use ggez::graphics::{DrawMode, Image, Mesh, MeshBuilder, Point2};
use rayon::prelude::*;

use std::path::Path;

pub struct MainState {
    pub image: Image,
    pub points: Vec<Point2>,
}

impl MainState {
    pub fn new<P: AsRef<Path>>(ctx: &mut Context, path: P) -> GameResult<Self> {

        Ok(Self {
            image: Image::new(ctx, path),
            points: Vec::new(),
        })
    }

    pub fn reset(&mut self) {
        points.clear();
    }

    pub fn valid_space(&self, x: usize, y: usize) -> bool {
        //  If spot isn't empty, then you can't place it
        if !self.board[crate::RANK * y + x].is_none() {
            return false;
        }

        //t spot left is always valid
        if self.turns == crate::RANK * crate::RANK - 1 {
            return true;
        }

        static DIRECTIONS: [Direction;  8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];

        DIRECTIONS.into_par_iter().any(|dir| {
            check_valid(&self.board, self.turn, *dir, x, y)
        })
    }

    pub fn captures(&self, x: usize, y: usize) -> Vec<usize> {
        static DIRECTIONS: [Direction;  8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];

        DIRECTIONS.into_par_iter().flat_map(|dir| {
            check_captures(&self.board, self.turn, *dir, x, y)
        }).collect::<Vec<usize>>()
    }

    pub fn place(&mut self, x: usize, y: usize) {
        self.board[crate::RANK * y + x] = Some(self.turn);
        
        self.captures(x, y).iter().for_each(|&index| {
            self.board[index] = Some(self.turn);
        });

        self.next_turn();
    }

    pub fn next_turn(&mut self) {
        if self.turn == Piece::White {
            self.turn = Piece::Black;
        } else {
            self.turn = Piece::White;
        }

        self.turns += 1;
    }

    pub fn has_move(&self) -> bool {
        (0..crate::RANK * crate::RANK).into_par_iter().any(|index| {
            self.valid_space(index % crate::RANK, index % crate::RANK)
        })
    }

    pub fn check_winner(&mut self) -> Winner {
        let white = self.board.par_iter().filter(|&x| *x == Some(Piece::White)).count();
        let black = crate::RANK * crate::RANK - white;

        if black > white {
            self.winner = Some(Winner::Black);
        } else if white > black {
            self.winner = Some(Winner::White);
        } else {
            self.winner = Some(Winner::Tie);
        }

        self.winner.unwrap()
    }

    pub fn auto_mode(&mut self, value: bool) {
        self.auto_mode = value;
    }

    pub fn is_auto(&self) -> bool {
        self.auto_mode
    }
}