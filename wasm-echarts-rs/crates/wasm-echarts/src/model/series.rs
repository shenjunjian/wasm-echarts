use std::cell::RefCell;
use std::collections::HashSet;

use crate::data::{Sampling, StackStrategy};
use crate::option::OptionValue;

thread_local! {
    static CUSTOM_SERIES_TYPES: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub fn register_custom_series_type(ty: impl Into<String>) {
    CUSTOM_SERIES_TYPES.with(|set| {
        set.borrow_mut().insert(ty.into());
    });
}

pub fn is_custom_series_type(ty: &str) -> bool {
    CUSTOM_SERIES_TYPES.with(|set| set.borrow().contains(ty))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesType {
    Line,
    Bar,
    Pie,
    Scatter,
    Radar,
    Map,
    Tree,
    Treemap,
    Graph,
    Chord,
    Gauge,
    Funnel,
    Parallel,
    Sankey,
    Boxplot,
    Candlestick,
    EffectScatter,
    Lines,
    Heatmap,
    PictorialBar,
    ThemeRiver,
    Sunburst,
    Custom,
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
            "radar" => SeriesType::Radar,
            "map" => SeriesType::Map,
            "tree" => SeriesType::Tree,
            "treemap" => SeriesType::Treemap,
            "graph" => SeriesType::Graph,
            "chord" => SeriesType::Chord,
            "gauge" => SeriesType::Gauge,
            "funnel" => SeriesType::Funnel,
            "parallel" => SeriesType::Parallel,
            "sankey" => SeriesType::Sankey,
            "boxplot" => SeriesType::Boxplot,
            "candlestick" => SeriesType::Candlestick,
            "effectScatter" => SeriesType::EffectScatter,
            "lines" => SeriesType::Lines,
            "heatmap" => SeriesType::Heatmap,
            "pictorialBar" => SeriesType::PictorialBar,
            "themeRiver" => SeriesType::ThemeRiver,
            "sunburst" => SeriesType::Sunburst,
            "custom" => SeriesType::Custom,
            other if is_custom_series_type(other) => SeriesType::Custom,
            _ => SeriesType::Other,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            SeriesType::Line => "line",
            SeriesType::Bar => "bar",
            SeriesType::Pie => "pie",
            SeriesType::Scatter => "scatter",
            SeriesType::Radar => "radar",
            SeriesType::Map => "map",
            SeriesType::Tree => "tree",
            SeriesType::Treemap => "treemap",
            SeriesType::Graph => "graph",
            SeriesType::Chord => "chord",
            SeriesType::Gauge => "gauge",
            SeriesType::Funnel => "funnel",
            SeriesType::Parallel => "parallel",
            SeriesType::Sankey => "sankey",
            SeriesType::Boxplot => "boxplot",
            SeriesType::Candlestick => "candlestick",
            SeriesType::EffectScatter => "effectScatter",
            SeriesType::Lines => "lines",
            SeriesType::Heatmap => "heatmap",
            SeriesType::PictorialBar => "pictorialBar",
            SeriesType::ThemeRiver => "themeRiver",
            SeriesType::Sunburst => "sunburst",
            SeriesType::Custom => "custom",
            SeriesType::Other => "other",
        }
    }

    pub fn is_cartesian_plot(self) -> bool {
        matches!(
            self,
            SeriesType::Line
                | SeriesType::Bar
                | SeriesType::Scatter
                | SeriesType::Candlestick
                | SeriesType::Boxplot
                | SeriesType::Heatmap
                | SeriesType::PictorialBar
                | SeriesType::EffectScatter
                | SeriesType::Custom
        )
    }

    pub fn uses_category_x_default(self) -> bool {
        matches!(
            self,
            SeriesType::Line
                | SeriesType::Bar
                | SeriesType::Candlestick
                | SeriesType::Boxplot
                | SeriesType::PictorialBar
                | SeriesType::ThemeRiver
        )
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
