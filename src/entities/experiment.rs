use super::condition_tree::TreeNode;
use std::collections::HashMap;

/// Variation represents a variation in the experiment
#[derive(Clone, Debug)]
pub struct Variation {
    pub id: String,
    pub variables: Vec<VariationVariable>,
    pub key: String,
    pub feature_enabled: bool,
}

/// Experiment represents an experiment
#[derive(Clone, Debug)]
pub struct Experiment<T> {
    pub audience_ids: Vec<String>,
    pub audience_conditions: T,
    pub id: String,
    pub layer_id: String,
    pub key: String,
    pub variations: HashMap<String, Variation>, // keyed by variation id
    pub variation_key_to_id_map: HashMap<String, String>,
    pub traffic_allocation: Vec<Range>,
    pub group_id: String,
    pub audience_condition_tree: Option<TreeNode<String>>,
    pub whitelist: HashMap<String, String>,
    pub is_feature_experiment: bool,
}

/// Range represents bucketing range that the specify entityID falls into
#[derive(Clone, Debug)]
pub struct Range {
    entity_id: String,
    end_of_range: i64,
}

// VariationVariable represents a Variable object from the Variation
#[derive(Clone, Debug)]
pub struct VariationVariable {
    id: String,
    value: String,
}
