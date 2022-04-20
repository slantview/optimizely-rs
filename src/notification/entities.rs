use std::{collections::HashMap, any::Any};

use crate::entities::user_context::UserContext;

// Type is the type of notification
pub enum Type {
    // Decision notification type
    Decision,
    // Track notification type
    Track,
    // ProjectConfigUpdate notification type
    ProjectConfigUpdate,
    // LogEvent notification type
    LogEvent,
}

impl Type {
    fn as_str(&self) -> &'static str {
        match self {
            Type::Decision => "decision",
            Type::Track => "track",
            Type::ProjectConfigUpdate => "project_config_update",
            Type::LogEvent => "log_event_notification",
        }
    }
}

// DecisionNotificationType is the type of decision notification
pub enum DecisionNotificationType {
    // ABTest is used when the decision is returned as part of evaluating an ab test
    ABTest,
    // Feature is used when the decision is returned as part of evaluating a feature
    Feature,
    // FeatureTest is used when the decision is returned as part of evaluating a feature test
    FeatureTest,
    // FeatureVariable is used when the decision is returned as part of evaluating a feature with a variable
    FeatureVariable,
    // AllFeatureVariables is used when the decision is returned as part of evaluating a feature with all variables
    AllFeatureVariables,
    // Flag is used when the decision is returned using decide api
    Flag,
}

impl DecisionNotificationType {
    fn as_str(&self) -> &'static str {
        match self {
            DecisionNotificationType::ABTest => "ab-test",
            DecisionNotificationType::Feature => "feature",
            DecisionNotificationType::FeatureTest => "feature-test",
            DecisionNotificationType::FeatureVariable => "feature-variable",
            DecisionNotificationType::AllFeatureVariables => "all-feature-variables",
            DecisionNotificationType::Flag => "flag",
        }
    }
}

// DecisionNotification is a notification triggered when a decision is made for either a feature or an experiment
pub struct DecisionNotification {
    decision_type: DecisionNotificationType,
    user_context: UserContext,
    decision_info: HashMap<String, Box<dyn Any>>,
}

// TrackNotification is a notification triggered when track is called
pub struct TrackNotification {
    event_key: String,
    user_context: UserContext,
    event_tags: HashMap<String, Box<dyn Any>>,
    conversion_event: Box<dyn Any>,
}

// ProjectConfigUpdateNotification is a notification triggered when a project config is updated
pub struct ProjectConfigUpdateNotification {
    notification_type: Type,
    revision: String,
}

// LogEventNotification is the notification triggered before log event is dispatched.
pub struct LogEventNotification {
    notification_type: Type,
    log_event: Box<dyn Any>,
}
