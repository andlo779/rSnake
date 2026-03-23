mod game;
mod render;
mod high_scores;

use crate::game::{GameState, Direction};
use crate::render::{Renderer, MenuOption};
use crate::high_scores::HighScoreManager;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};

enum GameMode {
    Menu,
    Playing,
    HighScore,
    GameOver,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new();
    renderer.setup()?;

    let mut high_score_manager = HighScoreManager::new();
    let mut game_mode = GameMode::Menu;
    let mut selected_option = MenuOption::StartGame;
    
    // Initial state
    let (width, height) = crossterm::terminal::size()?;
    let mut game = GameState::new(width, height - 2);

    'main_loop: loop {
        match game_mode {
            GameMode::Menu => {
                renderer.render_menu(selected_option)?;
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Up | KeyCode::Char('w') => {
                                selected_option = match selected_option {
                                    MenuOption::StartGame => MenuOption::Quit,
                                    MenuOption::ShowHighScore => MenuOption::StartGame,
                                    MenuOption::Quit => MenuOption::ShowHighScore,
                                };
                            }
                            KeyCode::Down | KeyCode::Char('s') => {
                                selected_option = match selected_option {
                                    MenuOption::StartGame => MenuOption::ShowHighScore,
                                    MenuOption::ShowHighScore => MenuOption::Quit,
                                    MenuOption::Quit => MenuOption::StartGame,
                                };
                            }
                            KeyCode::Enter => {
                                match selected_option {
                                    MenuOption::StartGame => {
                                        let (w, h) = crossterm::terminal::size()?;
                                        game = GameState::new(w, h - 2);
                                        renderer.set_first_render();
                                        game_mode = GameMode::Playing;
                                    }
                                    MenuOption::ShowHighScore => game_mode = GameMode::HighScore,
                                    MenuOption::Quit => break 'main_loop,
                                }
                            }
                            KeyCode::Char('q') | KeyCode::Esc => break 'main_loop,
                            _ => {}
                        }
                    }
                }
            }
            GameMode::HighScore => {
                renderer.render_high_score(high_score_manager.get_high_score())?;
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(_) = event::read()? {
                        game_mode = GameMode::Menu;
                    }
                }
            }
            GameMode::Playing => {
                let mut last_tick = Instant::now();
                let tick_rate = Duration::from_millis(100);

                renderer.render(&game, None)?;

                'game_loop: loop {
                    // 1. Handle Input
                    while event::poll(Duration::from_millis(0))? {
                        if let Event::Key(key) = event::read()? {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => {
                                    game_mode = GameMode::Menu;
                                    break 'game_loop;
                                }
                                KeyCode::Up | KeyCode::Char('w') => game.handle_input(Direction::Up),
                                KeyCode::Down | KeyCode::Char('s') => game.handle_input(Direction::Down),
                                KeyCode::Left | KeyCode::Char('a') => game.handle_input(Direction::Left),
                                KeyCode::Right | KeyCode::Char('d') => game.handle_input(Direction::Right),
                                _ => {}
                            }
                        }
                    }

                    // 2. Update state if needed
                    if !game.is_over && last_tick.elapsed() >= tick_rate {
                        let update = game.update();
                        last_tick = Instant::now();
                        
                        // 3. Render
                        if game.is_over {
                            high_score_manager.update_high_score(game.score);
                            renderer.render_game_over(&game)?;
                            game_mode = GameMode::GameOver;
                            break 'game_loop;
                        } else {
                            renderer.render(&game, update.as_ref())?;
                        }
                    }
                }
            }
            GameMode::GameOver => {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(_) = event::read()? {
                        game_mode = GameMode::Menu;
                    }
                }
            }
        }
    }

    renderer.cleanup()?;
    println!("Thanks for playing rSnake!");
    Ok(())
}
