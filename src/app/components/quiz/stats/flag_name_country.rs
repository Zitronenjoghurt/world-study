use crate::app::components::quiz::stats::{QuizStatsCommon, QuizStatsTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagNameCountryQuizStats {
    pub common: QuizStatsCommon,
}

impl QuizStatsTrait for FlagNameCountryQuizStats {
    fn get_common(&self) -> &QuizStatsCommon {
        &self.common
    }
}
