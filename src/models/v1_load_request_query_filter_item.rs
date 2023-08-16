#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LoadRequestQueryFilterItem {
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<String>,
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(rename = "or", skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<serde_json::Value>>,
    #[serde(rename = "and", skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<serde_json::Value>>,
}

impl V1LoadRequestQueryFilterItem {
    pub fn new() -> Self {
        V1LoadRequestQueryFilterItem {
            member: None,
            operator: None,
            values: None,
            or: None,
            and: None,
        }
    }

    pub fn member(mut self, member: &str) -> Self {
        self.member = Some(member.to_string());
        self
    }

    pub fn operator(mut self, operator: &str) -> Self {
        self.operator = Some(operator.to_string());
        self
    }

    pub fn values(mut self, values: Vec<String>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn or(mut self, or: Vec<serde_json::Value>) -> Self {
        self.or = Some(or);
        self
    }

    pub fn and(mut self, and: Vec<serde_json::Value>) -> Self {
        self.and = Some(and);
        self
    }
}
