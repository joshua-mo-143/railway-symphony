use std::env;

fn get_secret(secret: &str, required: bool) -> String {
	if !required {
		match env::var(secret) {
			Ok(res) => res,
			Err(_) => "None".to_string(),
		}
	} else {
		env::var(secret).unwrap_or_else(|_| panic!("Expected mandatory variable: {secret}"))
	}
}

pub struct Secrets {
	railway_project_ids: Vec<String>,
	railway_api_token: String,
	vector_bin_path: String,
	logtail_token: String,
	datadog_token: String,
	datadog_site: String,
}

pub fn get_all_secrets() -> Secrets {
	let railway_project_ids = get_secret("RAILWAY_PROJECT_IDS", true);
	let railway_api_token = get_secret("RAILWAY_API_TOKEN", true);
	let vector_bin_path = get_secret("VECTOR_BIN_PATH", true);

	let railway_project_ids: Vec<String> = railway_project_ids.as_str().split(',').map(|x| x.to_owned()).collect();
	let logtail_token = get_secret("LOGTAIL_TOKEN", false);
	let datadog_token = get_secret("DATADOG_TOKEN", false);
	let datadog_site = get_secret("DATADOG_SITE", false);
	
	Secrets {
		railway_project_ids,
		railway_api_token,
		vector_bin_path,
		logtail_token,
		datadog_token,
		datadog_site,
	}
}
