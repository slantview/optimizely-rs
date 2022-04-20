use super::feature::VariablePub;
use anyhow::anyhow;
use std::collections::HashMap;

const BUCKETING_ID_ATTRIBUTE_NAME: &str = "$opt_bucketing_id";

// UserContext holds information about a user
#[derive(Clone, Debug)]
pub struct UserContext {
    pub id: String,
    pub attributes: HashMap<String, VariablePub>,
}

impl UserContext {
    fn new(id: String, attributes: HashMap<String, VariablePub>) -> Self {
        Self { id, attributes }
    }
    // check_attribute_exists returns whether the specified attribute name exists in the attributes map.
    pub fn check_attribute_exists(&self, attr_name: String) -> bool {
        self.attributes.contains_key(&attr_name)
    }

    // get_string_attribute returns the string value for the specified attribute name in the attributes map. Returns error if not found.
    pub fn get_string_attribute(&self, attr_name: String) -> Option<String> {
        match self.attributes.get(&attr_name) {
            Some(attr) => match attr {
                VariablePub::String(value) => Some(value.clone()),
                _ => None,
            },
            None => None,
        }
    }

    // get_bool_attribute returns the bool value for the specified attribute name in the attributes map. Returns error if not found.
    pub fn get_bool_attribute(&self, attr_name: String) -> Option<bool> {
        match self.attributes.get(&attr_name) {
            Some(attr) => match attr {
                VariablePub::Boolean(value) => Some(*value),
                _ => None,
            },
            None => None,
        }
    }

    // get_float_attribute returns the float64 value for the specified attribute name in the attributes map. Returns error if not found.
    pub fn get_float_attribute(&self, attr_name: String) -> Option<f64> {
        match self.attributes.get(&attr_name) {
            Some(attr) => match attr {
                VariablePub::Double(value) => Some(*value),
                _ => None,
            },
            None => None,
        }
    }

    // GetIntAttribute returns the int64 value for the specified attribute name in the attributes map. Returns error if not found.
    pub fn get_int_attribute(&self, attr_name: String) -> Option<i64> {
        match self.attributes.get(&attr_name) {
            Some(attr) => match attr {
                VariablePub::Integer(value) => Some(*value),
                _ => None,
            },
            None => None,
        }
    }

    // get_attribute returns the value for the specified attribute name in the attributes map. Returns error if not found.
    pub fn get_attribute(&self, attr_name: String) -> Option<&VariablePub> {
        self.attributes.get(&attr_name)
    }

    // get_bucketing_id returns the bucketing ID to use for the given user
    pub fn get_bucketing_id(&self) -> Result<String, anyhow::Error> {
        // by default
        let mut bucketing_id = &self.id;

        // If the bucketing ID key is defined in attributes, than use that in place of the user ID
        if self.check_attribute_exists(BUCKETING_ID_ATTRIBUTE_NAME.to_owned()) {
            match self.get_string_attribute(BUCKETING_ID_ATTRIBUTE_NAME.to_owned()) {
                Some(value) => Ok(value),
                None => Err(anyhow!("invalid bucketing ID provided".to_owned())),
            }
        } else {
            Ok(self.id.clone())
        }
    }
}
