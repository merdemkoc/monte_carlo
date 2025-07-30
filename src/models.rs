use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    #[serde(rename = "task_id")]
    pub id: String,
    #[serde(rename = "task_name")]
    pub name: String,
    #[serde(rename = "predecessor")]
    pub predecessors: String,
    #[serde(rename = "optimistic")]
    pub optimistic: f64,
    #[serde(rename = "most_likely")]
    pub most_likely: f64,
    #[serde(rename = "pessimistic")]
    pub pessimistic: f64,
    #[serde(rename = "PERT_Expected")]
    pub pert_expected: f64,
    #[serde(rename = "PERT_Variance")]
    pub pert_variance: f64,
    #[serde(rename = "PERT_StdDev")]
    pub pert_stddev: f64,
}

#[derive(Debug, Clone)]
pub struct ProjectSchedule {
    pub tasks: HashMap<String, Task>,
    pub dependencies: HashMap<String, Vec<String>>,
    pub task_durations: HashMap<String, f64>,
    pub early_start: HashMap<String, f64>,
    pub early_finish: HashMap<String, f64>,
}

#[derive(Debug)]
pub struct SimulationResults {
    pub durations: Vec<f64>,
    pub mean: f64,
    pub median: f64,
    pub p80: f64,
    pub p95: f64,
    pub min: f64,
    pub max: f64,
    pub avg_base_duration: f64,
    pub avg_invisible_tasks: f64,
    pub avg_system_risk_factor: f64,
    pub critical_path: Vec<String>,
    pub critical_path_duration: f64,
}