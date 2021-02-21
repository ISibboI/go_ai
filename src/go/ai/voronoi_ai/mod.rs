use crate::go::board::GoCoordinates;
use crate::go::GoGame;
use crate::go::ai::GoAI;
use rand_pcg::Pcg64Mcg;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use std::time::SystemTime;

pub struct VoronoiAI {
    game: GoGame,
    random: Pcg64Mcg,
}

impl VoronoiAI {
    pub fn new() -> Self {
        Self {game: GoGame::new(), random: SeedableRng::seed_from_u64(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())}
    }
}

impl GoAI for VoronoiAI {
    fn set_game(&mut self, game: GoGame) {
        self.game = game;
    }

    fn best_move(&mut self) -> Option<GoCoordinates> {
        let mut best_moves = Vec::new();
        let mut best_diff = i64::min_value();

        for x in 0..9 {
            for y in 0..9 {
                let coordinates = GoCoordinates::new(x, y);

                if self.game.play_stone(coordinates).is_ok() {
                    let (black_voronoi, white_voronoi) = self.game.current_board().voronoi_score();
                    let black_voronoi = black_voronoi as i64;
                    let white_voronoi = white_voronoi as i64;
                    let diff = white_voronoi - black_voronoi;

                    if diff > best_diff {
                        best_diff = diff;
                        best_moves.clear();
                    }
                    if diff >= best_diff {
                        best_moves.push(coordinates);
                    }

                    self.game.undo().unwrap();
                }
            }
        }

        best_moves.choose(&mut self.random).copied()
    }
}