#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisType {
    Category,
    Value,
}

#[derive(Debug, Clone)]
pub struct AxisModel {
    pub axis_type: AxisType,
    pub category_data: Vec<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl AxisModel {
    pub fn with_data_range(mut self, (min, max): (f64, f64)) -> Self {
        if self.min.is_none() {
            self.min = Some(min);
        }
        if self.max.is_none() {
            self.max = Some(max);
        }
        self
    }

    pub fn value_min(&self) -> f64 {
        self.min.unwrap_or(0.0)
    }

    pub fn value_max(&self) -> f64 {
        self.max.unwrap_or(1.0)
    }
}
