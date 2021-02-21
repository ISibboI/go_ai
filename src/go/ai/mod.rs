use crate::go::board::GoCoordinates;
use crate::go::GoGame;
use rand_pcg::Pcg64Mcg;
use rand::SeedableRng;
use rand::distributions::{Distribution, Uniform};
use std::time::SystemTime;

pub mod voronoi_ai;

pub trait GoAI {
    fn set_game(&mut self, game: GoGame);

    fn best_move(&mut self) -> Option<GoCoordinates>;
}

pub struct RandomAI {
    game: GoGame,
    random: Pcg64Mcg,
}

impl RandomAI {
    pub fn new() -> Self {
        Self {game: GoGame::new(), random: SeedableRng::seed_from_u64(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())}
    }
}

impl GoAI for RandomAI {
    fn set_game(&mut self, game: GoGame) {
        self.game = game;
    }

    fn best_move(&mut self) -> Option<GoCoordinates> {
        let dist = Uniform::from(0..9);
        Some(GoCoordinates::new(dist.sample(&mut self.random), dist.sample(&mut self.random)))
    }
}