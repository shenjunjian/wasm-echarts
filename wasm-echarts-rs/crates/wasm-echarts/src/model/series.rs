use crate::option::OptionValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesType {
    Line,
    Bar,
    Pie,
    Scatter,
    Other,
}

impl SeriesType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "line" => SeriesType::Line,
            "bar" => SeriesType::Bar,
            "pie" => SeriesType::Pie,
            "scatter" => SeriesType::Scatter,
            _ => SeriesType::Other,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SeriesType::Line => "line",
            SeriesType::Bar => "bar",
            SeriesType::Pie => "pie",
            SeriesType::Scatter => "scatter",
            SeriesType::Other => "other",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub value: f64,
    pub x_value: Option<f64>,
    pub name: Option<String>,
    pub raw_index: usize,
    pub raw: OptionValue,
}

#[derive(Debug, Clone)]
pub struct SeriesModel {
    pub index: usize,
    pub name: String,
    pub series_type: SeriesType,
    pub data: Vec<DataPoint>,
}
