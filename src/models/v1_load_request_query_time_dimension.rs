use either::Either;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V1LoadRequestQueryTimeDimension {
    pub dimension: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    #[serde(rename = "dateRange", skip_serializing_if = "Option::is_none")]
    pub date_range: Option<Value>,
}

impl V1LoadRequestQueryTimeDimension {
    pub fn new(dimension: String) -> V1LoadRequestQueryTimeDimension {
        V1LoadRequestQueryTimeDimension {
            dimension,
            granularity: None,
            date_range: None,
        }
    }

    pub fn granularity(&mut self, granularity: &str) -> &mut Self {
        self.granularity = Some(granularity.to_string());
        self
    }

    pub fn date_range(&mut self, input: Either<String, Vec<String>>) -> &mut Self {
        match input {
            Either::Left(interval) => {
                self.date_range = Some(Value::String(interval));
            }
            Either::Right(arr) if arr.len() == 2 => {
                let start = &arr[0];
                let end = &arr[1];
                self.date_range = Some(Value::Array(vec![
                    Value::String(start.clone()),
                    Value::String(end.clone()),
                ]));
            }
            _ => self.date_range = None,
        };
        self
    }
}
