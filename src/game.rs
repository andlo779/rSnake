use std::collections::VecDeque;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct Snake {
    pub body: VecDeque<Point>,
    pub dir: Direction,
    pub next_dir: Direction,
}

impl Snake {
    pub fn new(start: Point, dir: Direction) -> Self {
        let mut body = VecDeque::new();
        body.push_back(start);
        // Initial body length 3
        match dir {
            Direction::Up => {
                body.push_back(Point { x: start.x, y: start.y + 1 });
                body.push_back(Point { x: start.x, y: start.y + 2 });
            },
            Direction::Down => {
                body.push_back(Point { x: start.x, y: start.y - 1 });
                body.push_back(Point { x: start.x, y: start.y - 2 });
            },
            Direction::Left => {
                body.push_back(Point { x: start.x + 1, y: start.y });
                body.push_back(Point { x: start.x + 2, y: start.y });
            },
            Direction::Right => {
                body.push_back(Point { x: start.x - 1, y: start.y });
                body.push_back(Point { x: start.x - 2, y: start.y });
            },
        }
        Self {
            body,
            dir,
            next_dir: dir,
        }
    }

    pub fn head(&self) -> Point {
        *self.body.front().expect("Snake body cannot be empty")
    }
}

pub struct GameState {
    pub snake: Snake,
    pub food: Point,
    pub width: u16,
    pub height: u16,
    pub score: u32,
    pub is_over: bool,
}

pub struct UpdateResult {
    pub new_head: Point,
    pub old_tail: Option<Point>,
    pub food_eaten: bool,
    pub new_food_pos: Point,
}

impl GameState {
    pub fn new(width: u16, height: u16) -> Self {
        let start_pos = Point { x: width / 2, y: height / 2 };
        let mut state = Self {
            snake: Snake::new(start_pos, Direction::Right),
            food: Point { x: 0, y: 0 },
            width,
            height,
            score: 0,
            is_over: false,
        };
        state.spawn_food();
        state
    }

    pub fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let food = Point {
                x: rng.gen_range(1..self.width - 1),
                y: rng.gen_range(1..self.height - 1),
            };
            if !self.snake.body.contains(&food) {
                self.food = food;
                break;
            }
        }
    }

    pub fn update(&mut self) -> Option<UpdateResult> {
        if self.is_over {
            return None;
        }

        // Update direction
        self.snake.dir = self.snake.next_dir;

        let head = self.snake.head();
        let mut next_head = head;

        match self.snake.dir {
            Direction::Up => next_head.y = next_head.y.saturating_sub(1),
            Direction::Down => next_head.y += 1,
            Direction::Left => next_head.x = next_head.x.saturating_sub(1),
            Direction::Right => next_head.x += 1,
        }

        // Check for wall collisions
        if next_head.x == 0 || next_head.x >= self.width - 1 || next_head.y == 0 || next_head.y >= self.height - 1 {
            self.is_over = true;
            return None;
        }

        // Check for self collision
        if self.snake.body.contains(&next_head) {
            self.is_over = true;
            return None;
        }

        self.snake.body.push_front(next_head);

        let mut food_eaten = false;
        let mut old_tail = None;

        // Check for food collision
        if next_head == self.food {
            self.score += 10;
            self.spawn_food();
            food_eaten = true;
        } else {
            old_tail = self.snake.body.pop_back();
        }

        Some(UpdateResult {
            new_head: next_head,
            old_tail,
            food_eaten,
            new_food_pos: self.food,
        })
    }

    pub fn handle_input(&mut self, dir: Direction) {
        if dir != self.snake.dir.opposite() {
            self.snake.next_dir = dir;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_movement() {
        let mut state = GameState::new(20, 20);
        let head_before = state.snake.head();
        let result = state.update().unwrap();
        let head_after = state.snake.head();
        assert_eq!(head_after.x, head_before.x + 1);
        assert_eq!(head_after.y, head_before.y);
        assert_eq!(result.new_head, head_after);
        assert!(result.old_tail.is_some());
    }

    #[test]
    fn test_food_consumption() {
        let mut state = GameState::new(20, 20);
        let head = state.snake.head();
        state.food = Point { x: head.x + 1, y: head.y };
        let body_len_before = state.snake.body.len();
        let result = state.update().unwrap();
        assert_eq!(state.snake.body.len(), body_len_before + 1);
        assert_eq!(state.score, 10);
        assert!(result.food_eaten);
        assert!(result.old_tail.is_none());
    }
}
