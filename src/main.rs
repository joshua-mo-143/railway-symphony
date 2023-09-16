use futures::StreamExt;
use gql::build_gql_client;
use graphql_client::GraphQLQuery;

mod gql;
mod secrets;
mod spawner;

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "gql/railway.graphql",
	query_path = "gql/schema.graphql",
	response_derives = "Debug, Serialize, Clone"
)]
pub struct DeploymentLogs;

#[tokio::main]
async fn main() {
	let vars = deployment_logs::Variables {
		deployment_id: "Insert your deployment ID here".to_string(),
		filter: Some(String::new()),
		limit: Some(500),
	};
	if let Ok((_ctx, _stream)) = build_gql_client::<DeploymentLogs>(vars).await {};

	// initialise Vector Datadog process

	// create HTTP client
	// create websocket client

	// create tokio::select loop that refreshes project ID list, pushes deployment logs, and then pushes the plugin logs
}
