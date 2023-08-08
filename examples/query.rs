use cube_client::apis::{configuration::Configuration, default_api as cube_api};
use cube_client::models::{V1LoadRequest, V1LoadRequestQuery, V1LoadRequestQueryTimeDimension};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cube_config = Configuration::default();
    cube_config.bearer_access_token = Some("my token".to_string());
    cube_config.base_path = "http://localhost:4000/cubejs-api".to_string();

    let mut query = V1LoadRequestQuery::new();
    query.limit = Some(5000);
    query.order = Some(vec![vec!["mints.timestamp".into(), "asc".into()]]);
    query.measures = Some(vec!["mints.count".to_string()]);

    let mut time_dimension = V1LoadRequestQueryTimeDimension::new("mints.timestamp".to_string());
    time_dimension.granularity = Some("day".to_string());

    query.time_dimensions = Some(vec![time_dimension]);

    let request = V1LoadRequest {
        query: Some(query),
        query_type: Some("multi".to_string()),
    };

    let response = cube_api::load_v1(&cube_config, Some(request)).await?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
