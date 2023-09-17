use futures::StreamExt;
use gql::build_gql_client;
use graphql_client::GraphQLQuery;
use secrets::{get_all_secrets, Secrets};
use tokio::task;
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
	println!("Starting up...");
	let secrets = get_all_secrets();
	let Secrets {
		railway_project_ids,
		railway_api_token,
		..
	} = secrets;

	// initialise Vector Datadog process
	for project in railway_project_ids {
	let vars = deployment_logs::Variables {
		deployment_id: project,
		filter: Some(String::new()),
		limit: Some(500),
	};

	if let Ok((_ctx, mut stream)) =
		build_gql_client::<DeploymentLogs>(vars, railway_api_token.clone()).await
	{
		println!("A new task has been started!");
		while let Some(item) = stream.next().await {
			println!("Received an item: {:?}", item);
		}
	}
}
	println!("Exiting...");
	// create tokio::select loop that refreshes project ID list, pushes deployment logs, and then pushes the plugin logs
}
