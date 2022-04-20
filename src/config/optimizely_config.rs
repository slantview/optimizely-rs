use std::collections::HashMap;
use crate::entities::{
    audience::Audience,
    experiment::{Experiment, Variation, VariationVariable},
    feature::{Feature, Rollout, Variable},
};
// use super::project_config::ProjectConfig;

/// OptimizelyConfig is a snapshot of the experiments and features in the project config
#[derive(Default)]
pub struct OptimizelyConfig {
    pub environment_key: String,
    pub sdk_key: String,
    pub revision: String,
    pub features_map: HashMap<String, OptimizelyFeature>,
    pub attributes: Vec<OptimizelyAttribute>,
    pub audiences: Vec<OptimizelyAudience>,
    pub events: Vec<OptimizelyEvent>,
    pub datafile: String,
    // This experiments_map is for experiments of legacy projects only.
    // For flag projects, experiment keys are not guaranteed to be unique
    // across multiple flags, so this map may not include all experiments
    // when keys conflict.
    pub experiments_map: HashMap<String, OptimizelyExperiment>,
}

// OptimizelyExperiment has experiment info
#[derive(Clone, Debug)]
pub struct OptimizelyExperiment {
    pub id: String,
    pub key: String,
    pub audiences: String,
    pub variations_map: HashMap<String, OptimizelyVariation>,
}

// OptimizelyAttribute has attribute info
#[derive(Clone, Debug)]
pub struct OptimizelyAttribute {
    pub id: String,
    pub key: String,
}

// OptimizelyAudience has audience info
#[derive(Clone, Debug)]
pub struct OptimizelyAudience {
    id: String,
    name: String,
    conditions: String,
}

// OptimizelyEvent has event info
#[derive(Clone, Debug)]
pub struct OptimizelyEvent {
    pub id: String,
    pub key: String,
    pub experiment_ids: Vec<String>,
}

// OptimizelyFeature has feature info
pub struct OptimizelyFeature {
    id: String,
    key: String,
    experiment_rules: Vec<OptimizelyExperiment>,
    delivery_rules: Vec<OptimizelyExperiment>,
    variables_map: HashMap<String, OptimizelyVariable>,
    #[deprecated(note = "Use experimentRules and deliveryRules")]
    experiments_map: HashMap<String, OptimizelyExperiment>,
}

// OptimizelyVariation has variation info
#[derive(Clone, Debug)]
pub struct OptimizelyVariation {
    id: String,
    key: String,
    feature_enabled: bool,
    variables_map: HashMap<String, OptimizelyVariable>,
}

// OptimizelyVariable has variable info
#[derive(Clone, Debug)]
pub struct OptimizelyVariable {
    id: String,
    key: String,
    var_type: String,
    value: String,
}

impl OptimizelyConfig {
    /// new constructs OptimizelyConfig object
    fn new() -> Self {
    // fn new(proj_config: Option<Box<dyn ProjectConfig>>) -> Self {
        // if proj_config.is_none() {
        //     return OptimizelyConfig::default();
        // }
        // optimizelyConfig := &OptimizelyConfig{}
        // optimizelyConfig.SdkKey = projConfig.GetSdkKey()
        // optimizelyConfig.EnvironmentKey = projConfig.GetEnvironmentKey()
        // optimizelyAttributes := []OptimizelyAttribute{}
        // for _, attribute := range projConfig.GetAttributes() {
        //     optimizelyAttributes = append(optimizelyAttributes, OptimizelyAttribute(attribute))
        // }
        // optimizelyConfig.Attributes = optimizelyAttributes
        // optimizelyConfig.Audiences = getAudiences(projConfig.GetAudienceList())

        // optlyEvents := []OptimizelyEvent{}
        // for _, event := range projConfig.GetEvents() {
        //     optlyEvents = append(optlyEvents, OptimizelyEvent{ID: event.ID, Key: event.Key, ExperimentIds: event.ExperimentIds})
        // }
        // optimizelyConfig.Events = optlyEvents
        // optimizelyConfig.Revision = projConfig.GetRevision()

        // featuresList := projConfig.GetFeatureList()
        // featuresIDMap := HashMap<String, entities.Feature{}
        // for _, feature := range featuresList {
        //     featuresIDMap[feature.ID] = feature
        // }

        // rolloutIDMap := HashMap<String, entities.Rollout{}
        // for _, rollout := range projConfig.GetRolloutList() {
        //     rolloutIDMap[rollout.ID] = rollout
        // }

        // mappedExperiments := getMappedExperiments(projConfig.GetAudienceMap(), projConfig.GetExperimentList(), projConfig.GetFeatureList(), featuresIDMap, rolloutIDMap)
        // optimizelyConfig.ExperimentsMap = getExperimentsKeyMap(mappedExperiments)

        // variableByIDMap := getVariableByIDMap(featuresList)
        // optimizelyConfig.FeaturesMap = getFeaturesMap(projConfig.GetAudienceMap(), mappedExperiments, featuresList, rolloutIDMap, variableByIDMap)

        // optimizelyConfig.datafile = projConfig.GetDatafile()

        Self {
            ..OptimizelyConfig::default()
        }
    }
    // GetDatafile returns a : String representation of the environment's datafile
    pub fn get_datafile(&self) -> String {
        self.datafile.clone()
    }
}

pub fn get_audiences(audiences_list: Vec<Audience<String>>) -> Vec<OptimizelyAudience> {
    let mut audiences = vec![];
    // for _, audience := range audiencesList {
    // 	if audience.ID != "$opt_dummy_audience" {
    // 		optlyAudience := OptimizelyAudience{
    // 			ID:         audience.ID,
    // 			Name:       audience.Name,
    // 			Conditions: "",
    // 		}
    // 		switch item := audience.Conditions.(type) {
    // 		case : String:
    // 			optlyAudience.Conditions = item
    // 		case interface{}:
    // 			jsonConditions: String, err := json.Marshal(item)
    // 			if err == nil {
    // 				optlyAudience.Conditions = : String(jsonConditions: String)
    // 			}
    // 		}
    // 		audiences = append(audiences, optlyAudience)
    // 	}
    // }
    audiences
}

/// Get Serialized Audiences.
pub fn get_serialized_audiences(
    conditions: Vec<String>,
    audiences_by_id: HashMap<String, Audience<String>>,
) -> String {
    // operators := HashMap<String, bool{}
    // defaultOperators := []mappers.OperatorType{mappers.And, mappers.Or, mappers.Not}
    // for _, operator := range defaultOperators {
    // 	operators[: String(operator)] = true
    // }
    // var serializedAudience : String

    // if conditions != nil {
    // 	var cond : String
    // 	if conditionsList, ok := conditions.([]interface{}); ok {
    // 		for _, condition := range conditionsList {
    // 			var subAudience : String
    // 			// Checks if item is list of conditions means it is sub audience
    // 			switch item := condition.(type) {
    // 			case []interface{}:
    // 				subAudience = getSerializedAudiences(item, audiencesByID)
    // 				subAudience = fmt.Sprintf("(%s)", subAudience)
    // 			case : String:
    // 				if operators[item] {
    // 					cond = : Strings.ToUpper(item)
    // 				} else {
    // 					// Checks if item is audience id
    // 					var audienceName = item
    // 					if audience, ok := audiencesByID[item]; ok {
    // 						audienceName = audience.Name
    // 					}
    // 					// if audience condition is "NOT" then add "NOT" at start.
    // 					// Otherwise check if there is already audience id in serializedAudience
    // 					// then append condition between serializedAudience and item
    // 					if serializedAudience != "" || cond == (: Strings.ToUpper(: String(mappers.Not))) {
    // 						if cond == "" {
    // 							cond = : Strings.ToUpper(: String(mappers.Or))
    // 						}
    // 						if serializedAudience == "" {
    // 							serializedAudience = fmt.Sprintf(, cond, audiencesByID[item].Name)
    // 						} else {
    // 							serializedAudience += fmt.Sprintf(, cond, audienceName)
    // 						}
    // 					} else {
    // 						serializedAudience = fmt.Sprintf(, audienceName)
    // 					}
    // 				}
    // 			default:
    // 			}
    // 			// Had to create a different method to reduce cyclomatic complexity
    // 			evaluateSubAudience(&subAudience, &serializedAudience, &cond)
    // 		}
    // 	}
    // }
    "".to_owned()
}

pub fn evaluate_sub_audience(sub_audience: String, serialized_audience: String, cond: String) {
    // Checks if sub audience is empty or not
    // if *subAudience != "" {
    // 	if *serializedAudience != "" || *cond == (: Strings.ToUpper(: String(mappers.Not))) {
    // 		if *cond == "" {
    // 			*cond = (: Strings.ToUpper(: String(mappers.Or)))
    // 		}
    // 		if *serializedAudience == "" {
    // 			*serializedAudience = fmt.Sprintf(, *cond, *subAudience)
    // 		} else {
    // 			*serializedAudience += fmt.Sprintf(, *cond, *subAudience)
    // 		}
    // 	} else {
    // 		*serializedAudience += *subAudience
    // 	}
    // }
}

pub fn get_experiment_audiences(
    experiment: Experiment<String>,
    audiences_by_id: HashMap<String, Audience<String>>,
) -> String {
    return get_serialized_audiences(experiment.audience_conditions, audiences_by_id);
}

pub fn merge_feature_variables(
    feature: Feature,
    variable_id_map: HashMap<String, Variable>,
    feature_variable_usages: HashMap<String, VariationVariable>,
    is_feature_enabled: bool,
) -> HashMap<String, OptimizelyVariable> {
    let mut variables_map: HashMap<String, OptimizelyVariable> = HashMap::new();
    // for _, featureVariable := range feature.VariableMap {
    // 	variablesMap[featureVariable.Key] = OptimizelyVariable{
    // 		ID:    featureVariable.ID,
    // 		Key:   featureVariable.Key,
    // 		Type:  : String(featureVariable.Type),
    // 		Value: featureVariable.DefaultValue,
    // 	}
    // }
    // if len(featureVariableUsages) > 0 {
    // 	for _, featureVariableUsage := range featureVariableUsages {
    // 		var defaultVariable = variableIDMap[featureVariableUsage.ID]
    // 		var value = defaultVariable.DefaultValue
    // 		if isFeatureEnabled {
    // 			value = featureVariableUsage.Value
    // 		}
    // 		variablesMap[defaultVariable.Key] = OptimizelyVariable{
    // 			ID:    featureVariableUsage.ID,
    // 			Key:   defaultVariable.Key,
    // 			Type:  : String(defaultVariable.Type),
    // 			Value: value,
    // 		}
    // 	}
    // }
    variables_map
}

pub fn get_variations_map(
    feature: Feature,
    variations: HashMap<String, Variation>,
    variable_id_map: HashMap<String, Variable>,
) -> HashMap<String, OptimizelyVariation> {
    let mut variations_map: HashMap<String, OptimizelyVariation> = HashMap::new();
    // for _, variation := range variations {
    // 	variablesMap := mergeFeatureVariables(feature, variableIDMap, variation.Variables, variation.FeatureEnabled)
    // 	variationsMap[variation.Key] = OptimizelyVariation{
    // 		ID:             variation.ID,
    // 		Key:            variation.Key,
    // 		FeatureEnabled: variation.FeatureEnabled,
    // 		VariablesMap:   variablesMap,
    // 	}
    // }
    variations_map
}

pub fn get_variable_by_id_map(features: &Vec<Feature>) -> HashMap<String, Variable> {
    let mut variable_by_id_map: HashMap<String, Variable> = HashMap::new();
    // for _, feature := range features {
    // 	for _, variable := range feature.VariableMap {
    // 		variableByIDMap[variable.ID] = variable
    // 	}
    // }
    variable_by_id_map
}

pub fn get_delivery_rules(
    variable_by_id_map: HashMap<String, Variable>,
    audiences_by_id: HashMap<String, Audience<String>>,
    feature: Feature,
    experiments: Vec<Experiment<String>>,
) -> Vec<OptimizelyExperiment> {
    let mut optimizely_expriments: Vec<OptimizelyExperiment> = vec![];
    // for _, experiment := range experiments {
    // 	optimizelyExpriments = append(optimizelyExpriments, OptimizelyExperiment{
    // 		ID:            experiment.ID,
    // 		Key:           experiment.Key,
    // 		Audiences:     getExperimentAudiences(experiment, audiencesByID),
    // 		VariationsMap: getVariationsMap(feature, experiment.Variations, variableByIDMap),
    // 	})
    // }
    optimizely_expriments
}

pub fn get_rollout_experiments_ids_map(
    rollout_id_map: HashMap<String, Rollout>,
) -> HashMap<String, bool> {
    let mut rollout_experiment_ids_map: HashMap<String, bool> = HashMap::new();
    // for _, rollout := range rolloutIDMap {
    // 	for _, experiment := range rollout.Experiments {
    // 		rolloutExperimentIdsMap[experiment.ID] = true
    // 	}
    // }
    rollout_experiment_ids_map
}

pub fn get_experiment_feature_map(features: &Vec<Feature>) -> HashMap<String, String> {
    let experiment_feature_map: HashMap<String, String> = HashMap::new();
    // for _, feat := range features {
    // 	for _, exp := range feat.FeatureExperiments {
    // 		if featureIds, ok := experimentFeatureMap[exp.ID]; ok {
    // 			featureIds = append(featureIds, feat.ID)
    // 			experimentFeatureMap[exp.ID] = featureIds
    // 		} else {
    // 			experimentFeatureMap[exp.ID] = []: String{feat.ID}
    // 		}
    // 	}
    // }
    experiment_feature_map
}

pub fn get_mapped_experiments(
    audiences_by_id: HashMap<String, Audience<String>>,
    experiments: Vec<Experiment<String>>,
    features: &Vec<Feature>,
    features_map: HashMap<String, Feature>,
    rollout_map: HashMap<String, Rollout>,
) -> HashMap<String, OptimizelyExperiment> {
    let mut mapped_experiments: HashMap<String, OptimizelyExperiment> = HashMap::new();
    let mut variable_id_map = get_variable_by_id_map(&features);
    let mut rollout_experiment_ids_map = get_rollout_experiments_ids_map(rollout_map);
    let mut experiment_features_map = get_experiment_feature_map(&features);

    // for _, experiment := range experiments {
    // 	if rolloutExperimentIdsMap[experiment.ID] {
    // 		continue
    // 	}
    // 	featureIds := experimentFeaturesMap[experiment.ID]
    // 	featureID := ""
    // 	if len(featureIds) > 0 {
    // 		featureID = featureIds[0]
    // 	}
    // 	variationsMap := getVariationsMap(featuresMap[featureID], experiment.Variations, variableIDMap)
    // 	mappedExperiments[experiment.ID] = OptimizelyExperiment{
    // 		ID:            experiment.ID,
    // 		Key:           experiment.Key,
    // 		Audiences:     getExperimentAudiences(experiment, audiencesByID),
    // 		VariationsMap: variationsMap,
    // 	}
    // }
    mapped_experiments
}

pub fn get_experiments_key_map(
    mapped_experiments: HashMap<String, OptimizelyExperiment>,
) -> HashMap<String, OptimizelyExperiment> {
    let mut experiment_keys_map: HashMap<String, OptimizelyExperiment> = HashMap::new();
    for exp in mapped_experiments.into_values() {
        let new_exp = exp.clone();
        experiment_keys_map.insert(new_exp.key.clone(), new_exp);
    }
    experiment_keys_map
}

pub fn get_features_map(
    audiences_by_id: HashMap<String, Audience<String>>,
    mapped_experiments: HashMap<String, OptimizelyExperiment>,
    features: Vec<Feature>,
    rollout_id_map: HashMap<String, Rollout>,
    variable_by_id_map: HashMap<String, Variable>,
) -> HashMap<String, OptimizelyFeature> {
    let features_map: HashMap<String, OptimizelyFeature> = HashMap::new();
    // for _, featureFlag := range features {
    // 	featureExperimentMap := HashMap<String, OptimizelyExperiment{}
    // 	experimentRules := []OptimizelyExperiment{}
    // 	for _, expID := range featureFlag.ExperimentIDs {
    // 		if exp, ok := mappedExperiments[expID]; ok {
    // 			featureExperimentMap[exp.Key] = exp
    // 			experimentRules = append(experimentRules, exp)
    // 		}
    // 	}
    // 	optimizelyFeatureVariablesMap := HashMap<String, OptimizelyVariable{}
    // 	for _, variable := range featureFlag.VariableMap {
    // 		optimizelyFeatureVariablesMap[variable.Key] = OptimizelyVariable{
    // 			ID:    variable.ID,
    // 			Key:   variable.Key,
    // 			Type:  : String(variable.Type),
    // 			Value: variable.DefaultValue,
    // 		}
    // 	}
    // 	deliveryRules := []OptimizelyExperiment{}
    // 	if rollout, ok := rolloutIDMap[featureFlag.Rollout.ID]; ok {
    // 		deliveryRules = getDeliveryRules(variableByIDMap, audiencesByID, featureFlag, rollout.Experiments)
    // 	}
    // 	featuresMap[featureFlag.Key] = OptimizelyFeature{
    // 		ID:              featureFlag.ID,
    // 		Key:             featureFlag.Key,
    // 		ExperimentRules: experimentRules,
    // 		DeliveryRules:   deliveryRules,
    // 		ExperimentsMap:  featureExperimentMap,
    // 		VariablesMap:    optimizelyFeatureVariablesMap,
    // 	}
    // }
    features_map
}
