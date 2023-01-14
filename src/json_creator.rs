use serde::Serialize;

#[derive(Serialize)]
struct Bic {
    bic: String,
    #[serde(rename(serialize = "participantInfo"))]
    participant_info: ParticipantInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    swifts: Option<Vec<Swift>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<Vec<Account>>
}

#[derive(Serialize)]
struct ParticipantInfo {
    name: String,
    #[serde(rename(serialize = "nameEnglish"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    name_english: Option<String>,
    #[serde(rename(serialize = "registrationNumber"))]
    registration_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_bic: Option<String>,
    ogrn: String,
    inn: String,
    #[serde(rename(serialize = "parentBic"))]
    #[serde(rename(serialize = "dateIn"))]
    date_in: String,
    #[serde(rename(serialize = "type"))]
    participant_type: String,
    uid: String,
    status: String,
    services: String,
    #[serde(rename(serialize = "exchangeType"))]
    exchange_type: String,
    #[serde(rename(serialize = "countryCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
    #[serde(rename(serialize = "regionCode"))]
    region_code: String,
    #[serde(rename(serialize = "regionName"))]
    region_name: String,
    #[serde(rename(serialize = "postalCode"))]
    postal_code: String,
    #[serde(rename(serialize = "localityType"))]
    locality_type: String,
    locality: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<String>
}

#[derive(Serialize)]
struct Swift {
    swift: String,
    #[serde(rename(serialize = "usedByDefault"))]
    used_by_default: bool
}

#[derive(Serialize)]
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
struct Data {
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

pub fn gen_json() -> String {
    let mut data = Data::new();

    data.add(Bic{
        bic: "043015000".to_string(),
        participant_info: ParticipantInfo {
            name: "РКЦ МИЛЬКОВО".to_string(),
            name_english: None,
            registration_number: "".to_string(),
            ogrn: "".to_string(),
            inn: "".to_string(),
            parent_bic: Some("043002001".to_string()),
            date_in: "1994-01-20".to_string(),
            participant_type: "10".to_string(),
            uid: "3015000000".to_string(),
            status: "ACTIVE".to_string(),
            services: "3".to_string(),
            exchange_type: "1".to_string(),
            country_code: None,
            region_code: "30".to_string(),
            region_name: "Камчатский край".to_string(),
            postal_code: "684300".to_string(),
            locality_type: "с.".to_string(),
            locality: "Мильково".to_string(),
            address: "ул Ленинская, 33".to_string(),
            phones: None,
        },
        swifts: None,
        accounts: None,
    });

    let json_str = serde_json::to_string(&data).unwrap();
    println!("Json: {}", json_str);
    json_str
}