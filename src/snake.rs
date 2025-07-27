// kanjisnake
// (C) 2025, Andreas Gaiser
// Snake abstraction.

use crate::arena::*;
use std::collections::VecDeque;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell {
    pub pos: ArenaPosition,
    pub character: char,
}

pub struct Snake {
    pub body: VecDeque<SnakeCell>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(arena: &mut Arena) -> Self {
        let head = SnakeCell {
            pos: ArenaPosition {
                x: arena.width / 2,
                y: arena.height / 2,
            },
            character: '$',
        };
        let second_to_head = SnakeCell {
            pos: ArenaPosition {
                x: arena.width / 2,
                y: arena.height / 2 + 1,
            },
            character: '龍',
        };
        return Self {
            body: VecDeque::from([head, second_to_head]),
            direction: Direction::Up,
        };
        // The snake is always at least two elements long.
    }

    fn is_covered_by_snake(&self, pos: &ArenaPosition) -> bool {
        for snake_cell in &self.body {
            if snake_cell.pos.x == pos.x && snake_cell.pos.y == pos.y {
                return true;
            }
        }
        return false;
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    fn shift_characters_forward(&mut self, left_input: char) {
        let mut current_char = left_input;
        for cell in self.body.iter_mut().rev() {
            let buffer = cell.character;
            cell.character = current_char;
            current_char = buffer;
        }
    }

    // returns true iff the snake is still collision-free and does
    // not leave the game arena. Updates the arena.
    pub fn next(&mut self, arena: &mut Arena) -> bool {
        // obtain the head/tail element of the snake

        if self.body.len() < 2 {
            return false;
        }

        let mut head = self.body.pop_front().unwrap();
        let tail = self.body.pop_back().unwrap();
        self.body.push_front(head); // idiotic, but to avoid borrow check madness

        // compute next head position
        match self.direction {
            Direction::Up => {
                if head.pos.y == 0 {
                    return false;
                }
                head.pos.y = head.pos.y - 1;
            }
            Direction::Down => {
                if head.pos.y == arena.height - 1 {
                    return false;
                }
                head.pos.y = head.pos.y + 1;
            }
            Direction::Left => {
                if head.pos.x == 0 {
                    return false;
                }
                head.pos.x = head.pos.x - 1;
            }
            Direction::Right => {
                if head.pos.x == arena.width - 1 {
                    return false;
                }
                head.pos.x = head.pos.x + 1;
            }
        }
        if self.is_covered_by_snake(&head.pos) {
            return false; // collision
        }
        self.body.push_front(head);
        self.shift_characters_forward(tail.character);
        arena.set_by_pos(&tail.pos, CellContent::Empty);
        // set tail cell to empty. It is however
        // possible that the multiple snake segments are at the tail.
        // Therefore we (re)set the complete snake body later again.
        match arena.get_by_pos(&head.pos) {
            CellContent::Food(kanji_char) => {
                let new_tail = SnakeCell {
                    pos: tail.pos,
                    character: kanji_char,
                };
                arena.set_by_pos(&head.pos, CellContent::Empty);
                self.body.push_back(new_tail); // prolong the snake
            }
            _ => {}
        }
        for snake_pos in &self.body {
            arena.set_by_pos(&snake_pos.pos, CellContent::Snake(snake_pos.character));
        }

        return true;
    }
}
