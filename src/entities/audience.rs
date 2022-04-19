use super::condition_tree::TreeNode;

/// Audience contains the audience definition
#[derive(Clone, Debug)]
pub struct Audience<T>  {
	pub id: String,
	pub name: String,
	pub condition_tree: Option<TreeNode<T>>,
	pub conditions: Vec<T>
}

/// Condition has condition info
#[derive(Clone, Debug)]
pub struct Condition<T> {
	pub name: String,
    pub match_expr: String,
	pub match_type: String,
	pub value: Option<T>,
	pub string_representation: String
}