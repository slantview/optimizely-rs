use super::{audience::Audience, user_context::UserContext};
use std::collections::HashMap;

/// TreeNode in a condition tree
#[derive(Clone, Debug)]
pub struct TreeNode<T> {
    pub item: Option<T>,
    pub operator: String,
    pub nodes: Vec<TreeNode<T>>,
}

/// TreeParameters represents parameters of a tree
#[derive(Clone, Debug)]
pub struct TreeParameters<T> {
    pub user: Option<UserContext>,
    pub audience_map: HashMap<String, Audience<T>>,
}

impl<T> TreeParameters<T> {
    /// new returns TreeParameters object
    fn new(user: Option<UserContext>, audience_map: HashMap<String, Audience<T>>) -> Self {
        Self {
            user,
            audience_map,
        }
    }
}
