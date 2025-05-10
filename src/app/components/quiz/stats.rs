use crate::app::components::quiz::stats::flag_name_country::FlagNameCountryQuizStats;
use serde::{Deserialize, Serialize};

pub mod flag_name_country;

pub trait QuizStatsTrait {
    fn get_common(&self) -> &QuizStatsCommon;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizStatsCommon {
    pub started_at_ms: u128,
    pub finished_at_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuizStats {
    FlagNameCountry(FlagNameCountryQuizStats),
}

impl QuizStatsTrait for QuizStats {
    fn get_common(&self) -> &QuizStatsCommon {
        match self {
            QuizStats::FlagNameCountry(stats) => stats.get_common(),
        }
    }
}
