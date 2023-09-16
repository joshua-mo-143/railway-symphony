use crate::spawner::tokio_spawner::TokioSpawner;
use anyhow::Result;
use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue, Message};
use futures::StreamExt;
use graphql_client::GraphQLQuery;
use graphql_ws_client::graphql::{GraphQLClient, StreamingOperation};
use graphql_ws_client::{AsyncWebsocketClient, GraphQLClientClientBuilder, SubscriptionStream};

pub async fn build_gql_client<T: GraphQLQuery + Send + Sync + Unpin + 'static>(
	variables: T::Variables,
) -> Result<(
	AsyncWebsocketClient<GraphQLClient, Message>,
	SubscriptionStream<GraphQLClient, StreamingOperation<T>>,
)>
where
	<T as GraphQLQuery>::Variables: Send + Sync + Unpin,
	<T as GraphQLQuery>::ResponseData: std::fmt::Debug,
{
	let mut req = "wss://backboard.railway.app/graphql/v2"
		.into_client_request()
		.unwrap();

	req.headers_mut().insert(
		"Sec-Websocket-Protocol",
		HeaderValue::from_str("graphql-transport-ws").unwrap(),
	);
	req.headers_mut().insert(
		"Authorization",
		HeaderValue::from_str("Bearer Your API Key here").unwrap(),
	);

	let (ws_stream, _) = connect_async(req)
		.await
		.expect("Connect to Railway GraphQL");

	let (sink, stream) = ws_stream.split::<Message>();

	let mut ctx = GraphQLClientClientBuilder::new()
		.build(stream, sink, TokioSpawner::current())
		.await
		.unwrap();

	let stream = ctx
		.streaming_operation(StreamingOperation::<T>::new(variables))
		.await
		.unwrap();

	Ok((ctx, stream))
}
