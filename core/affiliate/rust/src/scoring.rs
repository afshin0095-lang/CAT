//! 5-dimension program scoring engine.
//!
//! Inspired by Affitor's `affiliate-program-search` scoring framework.
//! Evaluates affiliate programs on 5 weighted dimensions to produce a
//! composite score that helps the decision engine rank programs.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgramScore {
    pub program_name: String,
    pub earning_potential: f64,
    pub content_potential: f64,
    pub market_demand: f64,
    pub competition: f64,
    pub trust_factor: f64,
}

impl ProgramScore {
    /// Weighted composite score (0.0 - 10.0).
    pub fn overall(&self) -> f64 {
        self.earning_potential * 0.30
            + self.content_potential * 0.25
            + self.market_demand * 0.20
            + self.competition * 0.15
            + self.trust_factor * 0.10
    }

    pub fn verdict(&self) -> ProgramVerdict {
        let score = self.overall();
        if score >= 7.5 { ProgramVerdict::StrongPick }
        else if score >= 5.5 { ProgramVerdict::WorthTesting }
        else { ProgramVerdict::Skip }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProgramVerdict {
    StrongPick,
    WorthTesting,
    Skip,
}

impl std::fmt::Display for ProgramVerdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramVerdict::StrongPick => write!(f, "Strong Pick"),
            ProgramVerdict::WorthTesting => write!(f, "Worth Testing"),
            ProgramVerdict::Skip => write!(f, "Skip"),
        }
    }
}

/// Score earning potential based on commission structure.
pub fn score_earning_potential(
    commission_percent: f64,
    is_recurring: bool,
    avg_product_price_minor: i64,
) -> f64 {
    let mut score = 0.0;
    score += (commission_percent / 10.0).min(4.0);
    if is_recurring { score += 3.0; }
    let price_dollars = avg_product_price_minor as f64 / 100.0;
    score += if price_dollars >= 100.0 { 3.0 }
             else if price_dollars >= 50.0 { 2.0 }
             else if price_dollars >= 20.0 { 1.0 }
             else { 0.5 };
    score.min(10.0)
}

/// Score content potential based on product characteristics.
pub fn score_content_potential(
    has_free_tier: bool,
    is_visual_product: bool,
    content_angle_count: u32,
) -> f64 {
    let mut score = 0.0;
    if has_free_tier { score += 3.0; }
    if is_visual_product { score += 3.0; }
    score += (content_angle_count as f64).min(4.0);
    score.min(10.0)
}

/// Sort programs by overall score (descending).
pub fn rank_programs(scores: &mut [ProgramScore]) {
    scores.sort_by(|a, b| b.overall().partial_cmp(&a.overall()).unwrap_or(std::cmp::Ordering::Equal));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_commission_recurring_expensive_is_strong() {
        let score = ProgramScore {
            program_name: "HeyGen".into(),
            earning_potential: 9.0,
            content_potential: 8.5,
            market_demand: 8.0,
            competition: 6.0,
            trust_factor: 8.0,
        };
        assert!(score.overall() >= 7.5);
        assert_eq!(score.verdict(), ProgramVerdict::StrongPick);
    }

    #[test]
    fn low_scores_produce_skip() {
        let score = ProgramScore {
            program_name: "BadProgram".into(),
            earning_potential: 3.0,
            content_potential: 2.0,
            market_demand: 4.0,
            competition: 5.0,
            trust_factor: 3.0,
        };
        assert!(score.overall() < 5.5);
        assert_eq!(score.verdict(), ProgramVerdict::Skip);
    }

    #[test]
    fn earning_potential_scoring() {
        let s = score_earning_potential(30.0, true, 10000);
        assert!(s >= 8.0);
        let s2 = score_earning_potential(5.0, false, 1000);
        assert!(s2 < 3.0);
    }

    #[test]
    fn ranking_sorts_descending() {
        let mut scores = vec![
            ProgramScore { program_name: "C".into(), earning_potential: 5.0, content_potential: 5.0, market_demand: 5.0, competition: 5.0, trust_factor: 5.0 },
            ProgramScore { program_name: "A".into(), earning_potential: 9.0, content_potential: 9.0, market_demand: 9.0, competition: 9.0, trust_factor: 9.0 },
            ProgramScore { program_name: "B".into(), earning_potential: 7.0, content_potential: 7.0, market_demand: 7.0, competition: 7.0, trust_factor: 7.0 },
        ];
        rank_programs(&mut scores);
        assert_eq!(scores[0].program_name, "A");
        assert_eq!(scores[1].program_name, "B");
        assert_eq!(scores[2].program_name, "C");
    }
}
