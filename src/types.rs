use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Record {
    pub r#type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>, // should be Integer type representing value * 10_000 (4 points after comma)
                             // amount: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisputeState {
    None,
    Dispute,
    ChargeBack,
}

#[derive(Clone, Debug)]
pub struct Dispute {
    pub r#type: String, // only deposits can be disputed?
    pub client: u16,
    pub amount: f64,
    pub disputed: DisputeState,
}

#[derive(Debug)]
pub struct Account {
    pub amount: f64,
    pub held: f64,
    pub frozen: bool,
}

#[derive(Serialize)]
pub struct AccountOutput {
    pub client: u16,
    pub amount: f64,
    pub total: f64,
    pub held: f64,
    pub frozen: bool,
}

pub type Client = u16;
pub type Tx = u32;
pub type Accounts = HashMap<Client, Account>;

pub type Disputes = HashMap<Tx, Dispute>;
