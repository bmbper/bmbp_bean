use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct RespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub code: String,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> RespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub fn query_ok(data: T, msg: &str) -> RespVo<T> {
        RespVo {
            code: "0".to_string(),
            data: Some(data),
            msg: msg.to_string(),
        }
    }
}

impl<T> RespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub fn ok_msg<M>(data: T, msg: M) -> RespVo<T>
    where
        M: ToString,
    {
        RespVo {
            code: "0".to_string(),
            data: Some(data),
            msg: msg.to_string(),
        }
    }
    pub fn err_msg<M>(msg: M) -> RespVo<T>
    where
        M: ToString,
    {
        RespVo {
            code: "-1".to_string(),
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
    pub code: String,
    pub msg: String,
    pub data: Option<Vec<T>>,
}

impl<T> VecRespVo<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub fn ok_msg<M>(data: Vec<T>, msg: M) -> VecRespVo<T>
    where
        M: ToString,
    {
        VecRespVo {
            code: "0".to_string(),
            data: Some(data),
            msg: msg.to_string(),
        }
    }
    pub fn err_msg<M>(msg: M) -> VecRespVo<T>
    where
        M: ToString,
    {
        VecRespVo {
            code: "-1".to_string(),
            data: None,
            msg: msg.to_string(),
        }
    }
}
