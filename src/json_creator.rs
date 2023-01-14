use serde::{Deserialize, Deserializer, Serialize};
use crate::regions::Region;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bic {
    #[serde(rename(deserialize = "@BIC"))]
    bic: String,
    #[serde(rename(serialize = "participantInfo", deserialize = "ParticipantInfo"))]
    participant_info: ParticipantInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    swifts: Option<Vec<Swift>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<Account>>
}

impl Bic {
    pub fn fix_fields(&mut self, regions: &Region) {
        self.fix_name_english();
        self.fix_locality_type();
        self.fix_region_name(regions);
    }
    fn fix_name_english(&mut self) {
        if let Some(name) = self.participant_info.name_english.take() {
            let name = name.replace("&quot;", "\"");
            self.participant_info.name_english = Some(name);
        }
    }
    fn fix_locality_type(&mut self) {
        if let Some(mut locality_type) = self.participant_info.locality_type.take() {
            locality_type.push('.');
            self.participant_info.locality_type = Some(locality_type);
        }
    }
    fn fix_region_name(&mut self, regions: &Region) {
        let region = regions.get_by_code(&self.participant_info.region_code);
        if let Ok(region_name) = region {
            self.participant_info.region_name = Some(region_name.to_string());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ParticipantInfo {
    #[serde(deserialize_with = "replace_quotes")]
    #[serde(rename(deserialize = "@NameP"))]
    name: String,
    #[serde(rename(serialize = "nameEnglish", deserialize = "@EnglName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    name_english: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "registrationNumber", deserialize = "@RegN"))]
    registration_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "parentBic", deserialize = "@PrntBIC"))]
    parent_bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ogrn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inn: Option<String>,
    #[serde(rename(serialize = "dateIn", deserialize = "@DateIn"))]
    date_in: String,
    #[serde(rename(serialize = "type", deserialize = "@PtType"))]
    participant_type: String,
    #[serde(rename(deserialize = "@UID"))]
    uid: String,
    #[serde(deserialize_with = "replace_status")]
    #[serde(rename(deserialize = "@ParticipantStatus"))]
    status: String,
    #[serde(rename(deserialize = "@Srvcs"))]
    services: String,
    #[serde(rename(serialize = "exchangeType", deserialize = "@XchType"))]
    exchange_type: String,
    #[serde(rename(serialize = "countryCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
    #[serde(rename(serialize = "regionCode", deserialize = "@Rgn"))]
    region_code: String,
    #[serde(rename(serialize = "regionName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    region_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "postalCode", deserialize = "@Ind"))]
    postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "localityType", deserialize = "@Tnp"))]
    locality_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "@Nnp"))]
    locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "@Adr"))]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<String>
}

fn replace_quotes<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.replace("&quot;", "\""))
}

fn replace_status<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.replace("PSAC", "ACTIVE"))
}

#[derive(Serialize, Deserialize, Debug)]
struct Swift {
    swift: String,
    #[serde(rename(serialize = "usedByDefault"))]
    used_by_default: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    account: String,
    #[serde(rename(serialize = "type"))]
    acc_type: String,
    #[serde(rename(serialize = "openDate"))]
    open_date: String,
    status: String,
    #[serde(rename(serialize = "controlKey"))]
    control_key: String
}

#[derive(Serialize)]
pub struct Data {
    bics: Vec<Bic>
}

impl Data {
    pub fn new() -> Self {
        Self {
            bics: vec![]
        }
    }

    pub fn add(&mut self, row: Bic) {
        self.bics.push(row);
    }
}
