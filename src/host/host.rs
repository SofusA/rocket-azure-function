use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub bindings: Vec<Binding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Binding {
    pub auth_level: AuthLevel,
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    pub direction: Direction,
    pub name: String,
    #[serde(default)]
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthLevel {
    Anonymous
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TriggerType {
    HttpTrigger
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    In, Out
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Method {
    Get, Post, Delete, Put, Patch
}

