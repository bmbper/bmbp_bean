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
pub struct BmbpCountVo {
    pub label: String,
    pub code: String,
    pub value: String,
    pub count: i64,
    pub children: Vec<BmbpCountVo>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BmbpIdNameVo {
    pub id: String,
    pub name: String,
}
