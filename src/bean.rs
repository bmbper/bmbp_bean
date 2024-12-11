use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BmbpComboVo {
    pub label: String,
    pub value: String,
    pub children: Vec<BmbpComboVo>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CountVo {
    pub label: String,
    pub code: String,
    pub value: String,
    pub count: i64,
    pub children: Vec<CountVo>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct IdNameVo {
    pub id: String,
    pub name: String,
}
