use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct PageVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub page_num: usize,
    pub page_size: usize,
    pub params: Option<T>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BatchVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub batch_vo: Vec<T>,
}
