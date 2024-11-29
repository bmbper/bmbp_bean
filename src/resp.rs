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

impl<T, M> RespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
    M: ToString,
{
    pub fn ok_msg(data: Vec<T>, msg: M) -> Self<T> {
        RespVo {
            code: 0,
            data: Some(data),
            msg: msg.to_string(),
        }
    }
    pub fn err_msg(msg: M) -> Self<T> {
        RespVo {
            code: -1,
            data: None,
            msg: msg.to_string(),
        }
    }
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

impl<T, M> VecRespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
    M: ToString,
{
    pub fn ok_msg(data: Vec<T>, msg: M) -> VecRespVo<T> {
        VecRespVo {
            code: 0,
            data: Some(data),
            msg: msg.to_string(),
        }
    }
    pub fn err_msg(msg: M) -> VecRespVo<T> {
        VecRespVo {
            code: -1,
            data: None,
            msg: msg.to_string(),
        }
    }
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
