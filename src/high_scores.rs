use std::fs;
use std::path::PathBuf;

pub struct HighScoreManager {
    file_path: PathBuf,
    high_score: u32,
}

impl HighScoreManager {
    pub fn new() -> Self {
        let path = PathBuf::from("high_score.txt");
        let high_score = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(0)
        } else {
            0
        };

        Self {
            file_path: path,
            high_score,
        }
    }

    pub fn get_high_score(&self) -> u32 {
        self.high_score
    }

    pub fn update_high_score(&mut self, score: u32) -> bool {
        if score > self.high_score {
            self.high_score = score;
            let _ = fs::write(&self.file_path, score.to_string());
            true
        } else {
            false
        }
    }
}
