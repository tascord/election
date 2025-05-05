use std::str::FromStr;

use log::{info, trace};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use to_and_fro::ToAndFro;

use crate::{download::Download, process::Process};

/// https://results.aec.gov.au/27966/Website/Downloads/HouseFirstPrefsByPartyDownload-27966.csv
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FirstPreferences {
    pub party: PartyAffilliation,
    pub ordinary: u64,
    pub absent: u64,
    pub provisional: u64,
    pub prepoll: u64,
    pub postal: u64,
    pub swing: f32,
}

impl FirstPreferences {
    pub fn total(&self) -> u128 {
        [
            self.ordinary,
            self.absent,
            self.provisional,
            self.prepoll,
            self.postal,
        ]
        .iter()
        .fold(0u128, |a, b| a + *b as u128)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TcpPP {
    pub party: PartyAffilliation,
    pub ordinary: u64,
    pub swing: f32,
    pub ballot_position: u16,
}

/// https://results.aec.gov.au/27966/Website/Downloads/HouseTcpByCandidateByPollingPlaceDownload-27966.csv
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwoCandidatePreferred {
    pub division: Division,
    pub parties: (TcpPP, TcpPP),
}

/// https://results.aec.gov.au/27966/Website/Downloads/HouseDopByDivisionDownload-27966.csv
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrefDistribution {
    pub division: Division,
    pub party: PartyAffilliation,
    pub preference_count: usize,
    pub transfer_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PartyPerformance {
    total_first_preferences: u128,
    seats_won: u32,
    preference_flow_strength: f32,
    swing_volatility: f32,
}

#[derive(ToAndFro, Serialize)]
pub enum PartyAffilliation {
    LP,   // Liberal
    LNP,  // Liberal National Party of Queensland
    NP,   // The Nationals
    CLP,  // Country Liberal Party (NT)
    ALP,  // Australian Labor Party
    XEN,  // Centre Alliance
    KAP,  // Katter's Australian Party (KAP)
    GRN,  // The Greens
    UAPP, // United Australia Party
    IND,  // Independent
    ON,   // Pauline Hanson's One Nation
    LDP,  // Liberal Democratic Party
    CYA,  // Australian Federation Party
    AJP,  // Animal Justice Party
    IMO,  // Informed Medical Options Party
    GAP,  // The Great Australian Party
    WAP,  // WESTERN AUSTRALIA PARTY
    VNS,  // Victorian Socialists
    AUC,  // Australian Christians
    SOPA, // FUSION: Science, Pirate, Secular, Climate Emergency
    CEC,  // Australian Citizens Party
    TNL,  // TNL
    DHJP, // Derryn Hinch's Justice Party
    SAL,  // Socialist Alliance
    AUVA, // Australian Values Party
    JLN,  // Jacqui Lambie Network
    ASP,  // Shooters, Fishers and Farmers Party
    IAP,  // Indigenous - Aboriginal Party of Australia
    SPP,  // Sustainable Australia Party - Stop Overdevelopment / Corruption
    AUP,  // Australian Progressives
    DPDA, // Drew Pavlou Democratic Alliance
    TLOC, // The Local Party of Australia
    AUD,  // Australian Democrats
    HMP,  // Legalise Cannabis Australia
    REAS, // Reason Australia
    NAFD, // Non Affiliated
}

#[derive(ToAndFro, Serialize, Deserialize)]
pub enum State {
    ACT,
    NSW,
    NT,
    QLD,
    SA,
    TAS,
    VIC,
    WA,
}

#[derive(ToAndFro, Serialize)]
pub enum Division {
    Bean,
    Canberra,
    Fenner,
    Banks,
    Barton,
    Bennelong,
    Berowra,
    Blaxland,
    Bradfield,
    Calare,
    Chifley,
    Cook,
    Cowper,
    Cunningham,
    Dobell,
    EdenMonaro,
    Farrer,
    Fowler,
    Gilmore,
    Grayndler,
    Greenway,
    Hughes,
    Hume,
    Hunter,
    KingsfordSmith,
    Lindsay,
    Lyne,
    Macarthur,
    Mackellar,
    Macquarie,
    McMahon,
    Mitchell,
    NewEngland,
    Newcastle,
    NorthSydney,
    Page,
    Parkes,
    Parramatta,
    Paterson,
    Reid,
    Richmond,
    Riverina,
    Robertson,
    Shortland,
    Sydney,
    Warringah,
    Watson,
    Wentworth,
    Werriwa,
    Whitlam,
    Lingiari,
    Solomon,
    Blair,
    Bonner,
    Bowman,
    Brisbane,
    Capricornia,
    Dawson,
    Dickson,
    Fadden,
    Fairfax,
    Fisher,
    Flynn,
    Forde,
    Griffith,
    Groom,
    Herbert,
    Hinkler,
    Kennedy,
    Leichhardt,
    Lilley,
    Longman,
    Maranoa,
    McPherson,
    Moncrieff,
    Moreton,
    Oxley,
    Petrie,
    Rankin,
    Ryan,
    WideBay,
    Wright,
    Adelaide,
    Barker,
    Boothby,
    Grey,
    Hindmarsh,
    Kingston,
    Makin,
    Mayo,
    Spence,
    Sturt,
    Bass,
    Braddon,
    Clark,
    Franklin,
    Lyons,
    Aston,
    Ballarat,
    Bendigo,
    Bruce,
    Calwell,
    Casey,
    Chisholm,
    Cooper,
    Corangamite,
    Corio,
    Deakin,
    Dunkley,
    Flinders,
    Fraser,
    Gellibrand,
    Gippsland,
    Goldstein,
    Gorton,
    Hawke,
    Higgins,
    Holt,
    Hotham,
    Indi,
    Isaacs,
    Jagajaga,
    Kooyong,
    LaTrobe,
    Lalor,
    Macnamara,
    Mallee,
    Maribyrnong,
    McEwen,
    Melbourne,
    Menzies,
    Monash,
    Nicholls,
    Scullin,
    Wannon,
    Wills,
    Brand,
    Burt,
    Canning,
    Cowan,
    Curtin,
    Durack,
    Forrest,
    Fremantle,
    Hasluck,
    Moore,
    OConnor,
    Pearce,
    Perth,
    Swan,
    Tangney,
}

impl<'de> Deserialize<'de> for Division {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Division::from_str(&s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
            .map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for PartyAffilliation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match PartyAffilliation::from_str(&s) {
            Ok(pa) => Ok(pa),
            _ => {
                trace!("Party {} not identified, maybe they should have custom flowing logic?", s);
                Ok(Self::NAFD)
            }
        }
    }
}


pub trait Get: Download + Process {
    async fn get(code: usize) -> anyhow::Result<Vec<Self>> {
        Self::process(&Self::download(code).await?)
    }
}

impl<T> Get for T where T: Download + Process {}

pub trait Named {
    fn name() -> String;
}

impl Named for TwoCandidatePreferred {
    fn name() -> String {
        "Two Candidate Preferred".to_owned()
    }
}

impl Named for FirstPreferences {
    fn name() -> String {
        "First Preference".to_owned()
    }
}

impl Named for PrefDistribution {
    fn name() -> String {
        "Preference Distribution".to_owned()
    }
}
