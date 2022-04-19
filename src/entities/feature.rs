use std::collections::HashMap;

use super::experiment::Experiment;

// Feature represents a feature flag
#[derive(Clone, Debug)]
pub struct Feature {
    pub id: String,
    pub key: String,
    pub feature_experiments: Vec<Experiment<String>>,
    pub experiment_ids: Vec<String>,
    pub rollout: Rollout,
    pub variable_map: HashMap<String, Variable>,
}

// Rollout represents a feature rollout
#[derive(Clone, Debug)]
pub struct Rollout {
    id: String,
    experiments: Vec<Experiment<String>>,
}

// Variable represents a feature variable
#[derive(Clone, Debug)]
pub struct Variable {
    default_value: String,
    id: String,
    key: String,
    var_type: VariablePub,
}

// Variablepub struct is the pub struct of feature variable
#[derive(Clone, Debug)]
pub enum VariablePub {
    // String - the feature-variable pub struct is string
    String(String),
    // Integer - the feature-variable pub struct is integer
    Integer(i64),
    // Double - the feature-variable pub struct is double
    Double(f64),
    // Boolean - the feature-variable pub struct is boolean
    Boolean(bool),
    // JSON - the feature-variable pub struct is json
    Json(String),
}
