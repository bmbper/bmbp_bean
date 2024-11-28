use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct PageData<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub page_num: u32,
    pub page_size: u32,
    pub total: u32,
    pub data: Vec<T>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct RespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct VecRespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<Vec<T>>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct PageRespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<PageData<T>>,
}
