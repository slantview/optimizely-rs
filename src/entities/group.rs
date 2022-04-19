use super::experiment::Range;

// Group represents a grouping of entities and their traffic allocation ranges
pub struct Group{
	id: String,
	traffic_allocation: Vec<Range>,
	policy: String
}