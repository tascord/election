use log::{LevelFilter, warn};
use std::{
    collections::HashMap,
    env::{self, args},
    fs,
};
use ty::*;

pub mod download;
pub mod process;
pub mod ty;

const FE_YEARS: &[(usize, usize)] = &[
    (2022, 27966),
    (2019, 24310),
    (2016, 20499),
    (2013, 17496),
    (2010, 15508),
    (2007, 13745),
    (2004, 12246),
];

fn logger() {
    let mut builder = pretty_env_logger::formatted_builder();
    builder.filter_level(if env::var("RUST_LOG").is_ok() {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    });
    builder.init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger();

    if args().any(|a| a.contains("-c")) {
        let _ = fs::remove_dir_all("cache")
            .inspect_err(|e| warn!("Failed to delete cache folder: {e:?}"));
    }

    let mut data = HashMap::<
        usize,
        (
            Vec<FirstPreferences>,
            Vec<TwoCandidatePreferred>,
            Vec<PrefDistribution>,
        ),
    >::new();

    let _ = fs::create_dir("cache");
    for (yr, code) in FE_YEARS {
        let cache_path = format!("cache/{}.json", yr);
        let data_entry = if let Ok(cached) = std::fs::read_to_string(&cache_path) {
            serde_json::from_str(&cached)?
        } else {
            let fp = FirstPreferences::get(*code).await?;
            let tcp = TwoCandidatePreferred::get(*code).await?;
            let pd = PrefDistribution::get(*code).await?;
            let entry = (fp, tcp, pd);
            std::fs::create_dir_all("cache")?;
            std::fs::write(&cache_path, serde_json::to_string(&entry)?)?;
            entry
        };
        data.insert(*yr, data_entry);
    }

    Ok(())
}
