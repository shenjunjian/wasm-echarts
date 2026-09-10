use crate::data::{Sampling, StackStrategy};
use crate::option::OptionValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesType {
    Line,
    Bar,
    Pie,
    Scatter,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CoordSysKind {
    #[default]
    Cartesian,
    Polar,
    Radar,
    Single,
    Parallel,
    Calendar,
    Matrix,
    Geo,
    None,
}

impl CoordSysKind {
    pub fn from_option(s: Option<&str>) -> Self {
        match s.unwrap_or("cartesian2d") {
            "polar" => CoordSysKind::Polar,
            "radar" => CoordSysKind::Radar,
            "single" | "singleAxis" => CoordSysKind::Single,
            "parallel" => CoordSysKind::Parallel,
            "calendar" => CoordSysKind::Calendar,
            "matrix" => CoordSysKind::Matrix,
            "geo" => CoordSysKind::Geo,
            "none" => CoordSysKind::None,
            _ => CoordSysKind::Cartesian,
        }
    }

    pub fn is_cartesian(self) -> bool {
        self == CoordSysKind::Cartesian
    }
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
    pub stack_base: f64,
    pub stacked_value: f64,
}

#[derive(Debug, Clone)]
pub struct SeriesModel {
    pub index: usize,
    pub name: String,
    pub series_type: SeriesType,
    pub data: Vec<DataPoint>,
    pub x_axis_index: usize,
    pub y_axis_index: usize,
    pub coord_sys: CoordSysKind,
    pub polar_index: usize,
    pub radar_index: usize,
    pub geo_index: usize,
    pub calendar_index: usize,
    pub single_axis_index: usize,
    pub parallel_index: usize,
    pub matrix_index: usize,
    pub stack: Option<String>,
    pub stack_strategy: StackStrategy,
    pub sampling: Sampling,
}
