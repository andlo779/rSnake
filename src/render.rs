use std::io::{stdout, Write, Stdout};
use crossterm::{
    execute,
    cursor,
    style::{self, Color, Print},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use crate::game::{GameState, UpdateResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MenuOption {
    StartGame,
    ShowHighScore,
    Quit,
}

pub struct Renderer {
    stdout: Stdout,
    is_first_render: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            is_first_render: true,
        }
    }

    pub fn setup(&mut self) -> Result<(), std::io::Error> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn set_first_render(&mut self) {
        self.is_first_render = true;
    }

    pub fn render_menu(&mut self, selected_option: MenuOption) -> Result<(), std::io::Error> {
        self.stdout.queue(Clear(ClearType::All))?;
        let (width, height) = terminal::size()?;
        
        let title = " rSNAKE ";
        let options = [
            (MenuOption::StartGame, " Start Game "),
            (MenuOption::ShowHighScore, " High Score "),
            (MenuOption::Quit, " Quit "),
        ];

        let mid_y = height / 2;
        let mid_x = width / 2;

        self.stdout.queue(style::SetForegroundColor(Color::Green))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (title.len() / 2) as u16, mid_y - 4))?.queue(Print(title))?;

        for (i, (opt, label)) in options.iter().enumerate() {
            if *opt == selected_option {
                self.stdout.queue(style::SetBackgroundColor(Color::White))?;
                self.stdout.queue(style::SetForegroundColor(Color::Black))?;
            } else {
                self.stdout.queue(style::SetBackgroundColor(Color::Reset))?;
                self.stdout.queue(style::SetForegroundColor(Color::White))?;
            }
            self.stdout.queue(cursor::MoveTo(mid_x - (label.len() / 2) as u16, mid_y - 1 + (i as u16 * 2)))?.queue(Print(label))?;
        }

        self.stdout.queue(style::SetBackgroundColor(Color::Reset))?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn render_high_score(&mut self, score: u32) -> Result<(), std::io::Error> {
        self.stdout.queue(Clear(ClearType::All))?;
        let (width, height) = terminal::size()?;
        
        let title = " HIGH SCORE ";
        let score_text = format!(" Current Record: {} ", score);
        let back_text = " Press any key to return to menu ";

        let mid_y = height / 2;
        let mid_x = width / 2;

        self.stdout.queue(style::SetForegroundColor(Color::Yellow))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (title.len() / 2) as u16, mid_y - 2))?.queue(Print(title))?;
        self.stdout.queue(style::SetForegroundColor(Color::White))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (score_text.len() / 2) as u16, mid_y))?.queue(Print(score_text))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (back_text.len() / 2) as u16, mid_y + 2))?.queue(Print(back_text))?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn render(&mut self, state: &GameState, update: Option<&UpdateResult>) -> Result<(), std::io::Error> {
        if self.is_first_render || update.is_none() {
            self.stdout.queue(Clear(ClearType::All))?;

            // Draw border
            self.stdout.queue(style::SetForegroundColor(Color::White))?;
            for x in 0..state.width {
                self.stdout.queue(cursor::MoveTo(x, 0))?.queue(Print("═"))?;
                self.stdout.queue(cursor::MoveTo(x, state.height - 1))?.queue(Print("═"))?;
            }
            for y in 0..state.height {
                self.stdout.queue(cursor::MoveTo(0, y))?.queue(Print("║"))?;
                self.stdout.queue(cursor::MoveTo(state.width - 1, y))?.queue(Print("║"))?;
            }
            // Draw corners
            self.stdout.queue(cursor::MoveTo(0, 0))?.queue(Print("╔"))?;
            self.stdout.queue(cursor::MoveTo(state.width - 1, 0))?.queue(Print("╗"))?;
            self.stdout.queue(cursor::MoveTo(0, state.height - 1))?.queue(Print("╚"))?;
            self.stdout.queue(cursor::MoveTo(state.width - 1, state.height - 1))?.queue(Print("╝"))?;

            // Draw food
            self.stdout.queue(style::SetForegroundColor(Color::Red))?;
            self.stdout.queue(cursor::MoveTo(state.food.x, state.food.y))?.queue(Print("●"))?;

            // Draw snake
            self.stdout.queue(style::SetForegroundColor(Color::Green))?;
            for (i, point) in state.snake.body.iter().enumerate() {
                self.stdout.queue(cursor::MoveTo(point.x, point.y))?;
                if i == 0 {
                    self.stdout.queue(Print("█"))?; // Head
                } else {
                    self.stdout.queue(Print("■"))?; // Body
                }
            }

            // Draw score
            self.stdout.queue(style::SetForegroundColor(Color::White))?;
            self.stdout.queue(cursor::MoveTo(2, state.height))?.queue(Print(format!("Score: {}", state.score)))?;

            self.is_first_render = false;
        } else if let Some(upd) = update {
            // Targeted update
            
            // 1. Clear old tail
            if let Some(tail) = upd.old_tail {
                self.stdout.queue(cursor::MoveTo(tail.x, tail.y))?.queue(Print(" "))?;
            }

            // 2. Draw previous head as body
            if state.snake.body.len() > 1 {
                let prev_head = state.snake.body[1];
                self.stdout.queue(style::SetForegroundColor(Color::Green))?;
                self.stdout.queue(cursor::MoveTo(prev_head.x, prev_head.y))?.queue(Print("■"))?;
            }

            // 3. Draw new head
            self.stdout.queue(style::SetForegroundColor(Color::Green))?;
            self.stdout.queue(cursor::MoveTo(upd.new_head.x, upd.new_head.y))?.queue(Print("█"))?;

            // 4. Update food and score if eaten
            if upd.food_eaten {
                self.stdout.queue(style::SetForegroundColor(Color::Red))?;
                self.stdout.queue(cursor::MoveTo(upd.new_food_pos.x, upd.new_food_pos.y))?.queue(Print("●"))?;
                
                self.stdout.queue(style::SetForegroundColor(Color::White))?;
                self.stdout.queue(cursor::MoveTo(2, state.height))?.queue(Print(format!("Score: {}", state.score)))?;
            }
        }

        self.stdout.flush()?;
        Ok(())
    }

    pub fn render_game_over(&mut self, state: &GameState) -> Result<(), std::io::Error> {
        let text = " GAME OVER ";
        let score_text = format!(" Final Score: {} ", state.score);
        let back_text = " Press any key to return to menu ";
        
        let mid_y = state.height / 2;
        let mid_x = state.width / 2;

        self.stdout.queue(style::SetForegroundColor(Color::Yellow))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (text.len() / 2) as u16, mid_y - 1))?.queue(Print(text))?;
        self.stdout.queue(style::SetForegroundColor(Color::White))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (score_text.len() / 2) as u16, mid_y))?.queue(Print(score_text))?;
        self.stdout.queue(cursor::MoveTo(mid_x - (back_text.len() / 2) as u16, mid_y + 1))?.queue(Print(back_text))?;
        
        self.stdout.flush()?;
        Ok(())
    }
}
