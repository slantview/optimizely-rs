use std::collections::HashMap;

use anyhow::Error;

use crate::entities::{attribute::Attribute, audience::Audience, event::Event, experiment::{Experiment, Variation}, feature::{Feature, Variable, Rollout}, group::Group};

/// ProjectConfig represents the project's experiments and feature flags and contains methods for accessing the them.
pub trait ProjectConfig where Self: Sized {
	fn get_datafile() -> String;
	fn get_account_id() -> String;
	fn get_anonymize_ip() -> bool;
	fn get_attribute_id(id: String) -> String; // returns "" if there is no id
	fn get_attribute_by_key(key: String) -> Option<Attribute>;
	fn get_audience_list() -> (Vec<Audience<String>>);
	fn get_audience_by_id(id: String) -> Option<Audience<String>>;
	fn get_audience_map() -> Vec<Audience<String>>;
	fn get_bot_filtering() -> bool;
	fn get_events() -> Vec<Event>;
	fn get_event_by_key(key: String) -> Option<Event>;
	fn get_experiment_by_key(key: String) -> Option<Experiment<String>>;
	fn get_feature_by_key(key: String) -> Option<Feature>;
	fn get_variable_by_key(feature_key: String, variable_key: String) -> Option<Variable>;
	fn get_experiment_list() -> Vec<Experiment<String>>;
	fn get_rollout_list() -> Vec<Rollout>;
	fn get_feature_list() -> Vec<Feature>;
	fn get_group_by_id(id: String) -> Option<Group>;
	fn get_project_id() -> String;
	fn get_revision() -> String;
	fn send_flag_decisions() -> bool;
	fn get_sdk_key() -> String;
	fn get_environment_key() -> String;
	fn get_attributes() -> Vec<Attribute>;
	fn get_flag_variations_map() -> HashMap<String, Variation>;
}

/// ProjectConfigManager maintains an instance of the ProjectConfig
pub trait ProjectConfigManager<T> where T: ProjectConfig, Self: Sized {
	fn get_config() -> Option<T>;
	// fn get_optimizely_config() -> Option<OptimizelyConfig>;
	fn remove_on_project_config_update(id: i64) -> Result<(), Error>;
	// fn on_project_config_update(callback: fn(notification::ProjectConfigUpdateNotification)) -> Option<i64>;
}