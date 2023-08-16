use crate::models::{
    V1LoadRequestQuery, V1LoadRequestQueryFilterItem, V1LoadRequestQueryTimeDimension,
};

#[derive(Debug, Clone, Default)]
pub struct Query {
    pub query: V1LoadRequestQuery,
}

impl Query {
    pub fn new() -> Self {
        Self {
            query: V1LoadRequestQuery::new(),
        }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.query.limit = Some(limit);
        self
    }

    pub fn order(mut self, column: &str, direction: &str) -> Self {
        self.query
            .order
            .get_or_insert_with(Vec::new)
            .push(vec![column.into(), direction.into()]);
        self
    }

    pub fn measures(mut self, measures: Vec<String>) -> Self {
        self.query.measures = Some(measures);
        self
    }
    pub fn dimensions(mut self, dimensions: Vec<String>) -> Self {
        self.query.dimensions = Some(dimensions);
        self
    }

    pub fn time_dimensions(mut self, dimension: V1LoadRequestQueryTimeDimension) -> Self {
        self.query
            .time_dimensions
            .get_or_insert_with(Vec::new)
            .push(dimension);
        self
    }

    pub fn filter_member(mut self, filter: V1LoadRequestQueryFilterItem) -> Self {
        self.query.filters.get_or_insert_with(Vec::new).push(filter);
        self
    }

    pub fn build(self) -> V1LoadRequestQuery {
        self.query
    }
}
