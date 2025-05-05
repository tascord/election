use crate::ty::{FirstPreferences, PrefDistribution, TwoCandidatePreferred};

pub trait Download: Sized {
    async fn download(code: usize) -> anyhow::Result<Vec<u8>>;
}

impl Download for TwoCandidatePreferred {
    async fn download(code: usize) -> anyhow::Result<Vec<u8>> {
        download(code, "HouseTcpByCandidateByPollingPlaceDownload").await
    }
}

impl Download for PrefDistribution {
    async fn download(code: usize) -> anyhow::Result<Vec<u8>> {
        download(code, "HouseDopByDivisionDownload").await
    }
}
impl Download for FirstPreferences {
    async fn download(code: usize) -> anyhow::Result<Vec<u8>> {
        download(code, "HouseFirstPrefsByPartyDownload").await
    }
}

// https://results.aec.gov.au/27966/Website/Downloads/HouseTcpByCandidateByPollingPlaceDownload-27966.csv
// https://results.aec.gov.au/15508/Website/Downloads/HouseTcpByCandidateByPollingPlaceDownload-15508.csv
async fn download(code: usize, file_name: &str) -> anyhow::Result<Vec<u8>> {
    Ok(reqwest::get(format!(
        "https://results.aec.gov.au/{code}/Website/Downloads/{file_name}-{code}.csv"
    ))
    .await?
    .bytes()
    .await?
    .to_vec())
}
