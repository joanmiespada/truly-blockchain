use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{fmt, str::FromStr};
use validator::Validate;
//use web3::types::H160;

#[derive(Clone, Serialize, Validate, Deserialize, Debug)]
pub struct Contract {
    id: u16,
    creation_time: DateTime<Utc>,
    blockchain: String,
    address: Option<String>,
    owner_address: Option<String>,
    owner_secret: Option<String>,
    owner_cash: Option<String>,
    details: Option<String>,
    status: ContractStatus,
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!(self).to_string())
    }
}

impl Contract {
    pub fn new() -> Contract {
        Contract {
            id: 0,
            creation_time: Utc::now(),
            blockchain: "".to_string(),
            address: None,
            owner_address: None,
            owner_secret: None,
            owner_cash: None,
            details: None,
            status: ContractStatus::Disabled,
        }
    }

    pub fn new_c(
        id: u16,
        creation_time: DateTime<Utc>,
        blockchain: String,
        address: Option<String>,
        owner_address: Option<String>,
        owner_secret: Option<String>,
        owner_cash: Option<String>,
        details: Option<String>,
        status: ContractStatus,
    ) -> Contract {
        Contract {
            id,
            creation_time,
            blockchain,
            address,
            owner_address,
            owner_secret,
            owner_cash,
            details,
            status,
        }
    }

    pub fn id(&self) -> &u16 {
        &self.id
    }
    pub fn set_id(&mut self, val: &u16) {
        self.id = val.clone()
    }
    pub fn creation_time(&self) -> &DateTime<Utc> {
        &self.creation_time
    }
    pub fn set_creation_time(&mut self, val: &DateTime<Utc>) {
        self.creation_time = val.clone()
    }
    pub fn blockchain(&self) -> &String {
        &self.blockchain
    }
    pub fn set_blockchain(&mut self, val: &String) {
        self.blockchain = val.clone()
    }
    pub fn address(&self) -> &Option<String> {
        &self.address
    }
    pub fn set_address(&mut self, val: &String) {
        self.address = Some(val.clone())
    }
    pub fn owner_address(&self) -> &Option<String> {
        &self.owner_address
    }
    pub fn set_owner_address(&mut self, val: &String) {
        self.owner_address = Some(val.clone())
    }
    pub fn owner_secret(&self) -> &Option<String> {
        &self.owner_secret
    }
    pub fn set_owner_secret(&mut self, val: &String) {
        self.owner_secret = Some(val.clone())
    }
    pub fn owner_cash(&self) -> &Option<String> {
        &self.owner_cash
    }
    pub fn set_owner_cash(&mut self, val: &String) {
        self.owner_cash = Some(val.clone())
    }

    pub fn details(&self) -> &Option<String> {
        &self.details
    }
    pub fn set_details(&mut self, val: &String) {
        self.details = Some(val.clone())
    }
    pub fn status(&self) -> &ContractStatus {
        &self.status
    }
    pub fn set_status(&mut self, val: &ContractStatus) {
        self.status = val.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ContractStatus {
    Enabled,
    Disabled,
}

impl ContractStatus {
    pub fn is_disabled(&self) -> bool {
        match *self {
            ContractStatus::Disabled => true,
            _ => false,
        }
    }
}

impl fmt::Display for ContractStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContractStatus::Enabled => write!(f, "Enabled"),
            ContractStatus::Disabled => write!(f, "Disabled"),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseContractStatusError;
impl FromStr for ContractStatus {
    type Err = ParseContractStatusError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "Enabled" => Ok(ContractStatus::Enabled),
            "Disabled" => Ok(ContractStatus::Disabled),
            _ => Err(ParseContractStatusError),
        }
    }
}

impl fmt::Display for ParseContractStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error parsing contract status type")
    }
}
