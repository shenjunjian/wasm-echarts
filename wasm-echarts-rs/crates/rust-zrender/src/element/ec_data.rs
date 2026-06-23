//! ECData：echarts 层反查元数据（对齐 zrender element 上的 ecData）

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EcData {
    pub series_index: Option<i32>,
    pub data_index: Option<i32>,
    pub data_type: Option<String>,
}

impl EcData {
    pub fn new(series_index: i32, data_index: i32) -> Self {
        Self {
            series_index: Some(series_index),
            data_index: Some(data_index),
            data_type: None,
        }
    }

    pub fn with_data_type(mut self, data_type: impl Into<String>) -> Self {
        self.data_type = Some(data_type.into());
        self
    }
}
