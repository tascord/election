use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use log::warn;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use serde_json::Value;

use crate::ty::{FirstPreferences, Named, PrefDistribution, TcpPP, TwoCandidatePreferred};

pub trait Process: Sized {
    fn process(data: &[u8]) -> anyhow::Result<Vec<Self>>;
}

impl Process for PrefDistribution {
    fn process(data: &[u8]) -> anyhow::Result<Vec<Self>> {
        let data = csv(data)?;
        Ok(data
            .chunks(4)
            .par_bridge()
            .into_par_iter()
            .map(|c| -> anyhow::Result<PrefDistribution> {
                let first = c.first().ok_or(anyhow!("No data in row"))?;
                let last = c.last().ok_or(anyhow!("Not enough data in row"))?;

                let party = first.get("PartyAb").ok_or(anyhow!("No field 'PartyAb'"))?;
                let party = serde_json::from_value(Value::String(party.to_string()))?;

                let division = first
                    .get("DivisionNm")
                    .ok_or(anyhow!("No field 'DivisionNm'"))?;

                let division = serde_json::from_value(Value::String(division.to_string()))?;

                let pref_count = first
                    .get("CalculationValue")
                    .ok_or(anyhow!("No field 'CalculationValue'"))?
                    .parse()?;

                let trans_count = last
                    .get("CalculationValue")
                    .ok_or(anyhow!("No field 'CalculationValue'"))?
                    .parse()?;

                Ok(PrefDistribution {
                    division,
                    party,
                    preference_count: pref_count,
                    transfer_count: trans_count,
                })
            })
            .filter_map(|v| {
                v.inspect_err(|e| warn!("Broken row in file {}: {e:?}", Self::name()))
                    .ok()
            })
            .collect())
    }
}

impl Process for TwoCandidatePreferred {
    fn process(data: &[u8]) -> anyhow::Result<Vec<Self>> {
        let data = csv(data)?;
        Ok(data
            .chunks(2)
            .par_bridge()
            .into_par_iter()
            .map(|c| -> anyhow::Result<TwoCandidatePreferred> {
                let division = c
                    .first()
                    .and_then(|c| c.get("DivisionNm"))
                    .ok_or(anyhow!("No field 'DivisionNm'"))?;
                let division = serde_json::from_value(Value::String(division.to_string()))?;

                let parties = c.iter().map(|p| -> anyhow::Result<TcpPP> {
                    let party = p.get("PartyAb").ok_or(anyhow!("No field 'PartyAb'"))?;
                    let party = serde_json::from_value(Value::String(party.to_string()))?;

                    let ordinary = p
                        .get("OrdinaryVotes")
                        .ok_or(anyhow!("No field 'OrdinaryVotes'"))?
                        .parse()?;

                    let swing = p.get("Swing").ok_or(anyhow!("No field 'Swing'"))?.parse()?;
                    let ballot_position = p
                        .get("BallotPosition")
                        .ok_or(anyhow!("No field 'BallotPosition'"))?
                        .parse()?;

                    Ok(TcpPP {
                        party,
                        ordinary,
                        swing,
                        ballot_position,
                    })
                });

                let parties = parties.collect::<Result<Vec<_>, _>>()?;

                Ok(TwoCandidatePreferred {
                    division,
                    parties: parties
                        .into_iter()
                        .collect_tuple()
                        .ok_or(anyhow!("Scuffed"))?,
                })
            })
            .filter_map(|v| {
                v.inspect_err(|e| warn!("Broken row in file {}: {e:?}", Self::name()))
                    .ok()
            })
            .collect())
    }
}

impl Process for FirstPreferences {
    fn process(data: &[u8]) -> anyhow::Result<Vec<Self>> {
        let data = csv(data)?;
        Ok(data
            .into_iter()
            .par_bridge()
            .into_par_iter()
            .map(|c| -> anyhow::Result<FirstPreferences> {
                let party = c.get("PartyAb").ok_or(anyhow!("No field 'PartyAb'"))?;
                let party = serde_json::from_value(Value::String(party.to_string()))?;

                let ordinary = c
                    .get("OrdinaryVotes")
                    .ok_or(anyhow!("No field 'OrdinaryVotes'"))?
                    .parse()?;

                let absent = c
                    .get("AbsentVotes")
                    .ok_or(anyhow!("No field 'AbsentVotes'"))?
                    .parse()?;

                let provisional = c
                    .get("ProvisionalVotes")
                    .ok_or(anyhow!("No field 'ProvisionalVotes'"))?
                    .parse()?;

                let prepoll = c
                    .get("PrePollVotes")
                    .ok_or(anyhow!("No field 'PrePollVotes'"))?
                    .parse()?;

                let swing = c
                    .get("PostalVotes")
                    .ok_or(anyhow!("No field 'PostalVotes'"))?
                    .parse()?;

                let postal = c
                    .get("TotalSwing")
                    .ok_or(anyhow!("No field 'TotalSwing'"))?
                    .parse()?;

                Ok(FirstPreferences {
                    party,
                    ordinary,
                    absent,
                    provisional,
                    prepoll,
                    postal,
                    swing,
                })
            })
            .filter_map(|v| {
                v.inspect_err(|e| warn!("Broken row in file {}: {e:?}", Self::name()))
                    .ok()
            })
            .collect())
    }
}

fn csv(data: &[u8]) -> anyhow::Result<Vec<HashMap<String, String>>> {
    let binding = String::from_utf8(data.to_vec())?;
    let mut lines = binding.lines();

    // Skip first line if it's metadata (one cell)
    let first_line = lines.next().unwrap_or_default();
    let mut header_line = first_line;

    if !first_line.contains(',') {
        header_line = lines.next().unwrap_or_default();
    }

    let second_line = lines.next().unwrap_or_default();

    // Pick longer line as headers
    let headers = if second_line.len() > header_line.len() {
        second_line
    } else {
        header_line
    };

    let headers = {
        let mut values = Vec::new();
        let mut escape = false;
        let mut current = String::new();
        for c in headers.chars() {
            match (c, escape) {
                ('"', false) => escape = true,
                ('"', true) => escape = false,
                (',', false) => {
                    values.push(current.trim().to_string());
                    current = String::new();
                }
                (_, _) => current.push(c),
            }
        }
        if !current.is_empty() {
            values.push(current.trim().to_string());
        }
        values
    };

    let data = lines
        .par_bridge()
        .into_par_iter()
        .map(|v| {
            let mut values = Vec::new();
            let mut escape = false;
            let mut current = String::new();
            for c in v.chars() {
                match (c, escape) {
                    ('"', false) => escape = true,
                    ('"', true) => escape = false,
                    (',', false) => {
                        values.push(current.trim().to_string());
                        current = String::new();
                    }
                    (_, _) => current.push(c),
                }
            }
            if !current.is_empty() {
                values.push(current.trim().to_string());
            }
            let mut row = HashMap::new();
            for (header, value) in headers.iter().zip(values.iter()) {
                row.insert(header.clone(), value.clone());
            }
            row
        })
        .collect();

    Ok(data)
}
