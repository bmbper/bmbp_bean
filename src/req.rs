use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct BmbpPageReq<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub page_num: usize,
    pub page_size: usize,
    pub params: Option<T>,
}
