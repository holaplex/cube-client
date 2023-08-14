use chrono::NaiveDate;
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LoadRequestQueryTimeDimension {
    pub dimension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "dateRange", skip_serializing_if = "Option::is_none")]
    pub date_range: Option<(NaiveDate, NaiveDate)>,
}

impl V1LoadRequestQueryTimeDimension {
    pub fn new(dimension: String) -> V1LoadRequestQueryTimeDimension {
        V1LoadRequestQueryTimeDimension {
            dimension,
            granularity: None,
            date_range: None,
        }
    }
}
