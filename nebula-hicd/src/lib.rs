#![cfg_attr(not(feature = "std"), no_std)]

use postcard_rpc::{endpoints, topics, TopicDirection};
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct SleepMillis {
    pub millis: u16,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct SleptMillis {
    pub millis: u16,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub enum LedState {
    Off,
    On,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub enum Asic {
    Bm1362,
    Bm1366,
    Bm1368,
    Bm1370,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct Chain {
    pub asic: Asic,
    pub cnt: u8,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct Info<'a> {
    pub version: &'a str,
    pub chain: Chain,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct Job {
    pub id: u32,
    pub version: u32,
    pub prev_tx_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub ntime: u32,
    pub nbits: u32,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct Share {
    pub job_id: u32,
    pub rolled_version: Option<u32>,
    pub rolled_ntime: Option<u32>,
    pub nonce: u32,
}

// ---

// Endpoints spoken by our device
//
// GetUniqueIdEndpoint is mandatory, the others are examples
endpoints! {
    list = ENDPOINT_LIST;
    | EndpointTy                | RequestTy     | ResponseTy            | Path                          |
    | ----------                | ---------     | ----------            | ----                          |
    | GetUniqueIdEndpoint       | ()            | u64                   | "poststation/unique_id/get"   |
    | RebootToPicoBoot          | ()            | ()                    | "picoboot/reset"              |
    | SleepEndpoint             | SleepMillis   | SleptMillis           | "nebula/sleep"                |
    | SetLedEndpoint            | LedState      | ()                    | "nebula/led/set"              |
    | GetInfoEndpoint           | ()            | Info<'a>              | "nebula/info"                 |
}

// incoming topics handled by our device
topics! {
    list = TOPICS_IN_LIST;
    direction = TopicDirection::ToServer;
    | TopicTy                   | MessageTy     | Path              |
    | -------                   | ---------     | ----              |
    | JobTopic                  | Job           | "/nebula/job"     |
    | StopTopic                 | ()            | "/nebula/stop"    |
}

// outgoing topics handled by our device
topics! {
    list = TOPICS_OUT_LIST;
    direction = TopicDirection::ToClient;
    | TopicTy                   | MessageTy     | Path                | Cfg                           |
    | -------                   | ---------     | ----                | ---                           |
    | AsicTempTopic             | i8            | "/nebula/asic_temp" |                               |
    | ShareTopic                | Share         | "/nebula/share"     |                               |
}
