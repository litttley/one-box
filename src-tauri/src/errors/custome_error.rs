use image::ImageError;
use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum CustomeErrors {
    #[error("自定错误：{0}")]
    CustomError(String),

    #[error("窗口图标加载异常:{0}")]
    IconError(String),

    #[error("加载前端资源异常:{0}")]
    AssetResolverError(String),
    // #[error("json序列化异常")]
    // ImageLoadeError(#[from] ImageError),
}
