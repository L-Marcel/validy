use async_trait::async_trait;
use axum::extract::FromRef;
use std::sync::Arc;

#[async_trait]
pub trait MockedService: Send + Sync {
	async fn email_exists(&self, email: &str) -> bool;
	fn sync_email_exists(&self, email: &str) -> bool;
}

pub struct ImplMockedService {}
#[async_trait]
impl MockedService for ImplMockedService {
	async fn email_exists(&self, email: &str) -> bool {
		email == "test@gmail.com"
	}
	fn sync_email_exists(&self, email: &str) -> bool {
		email == "test@gmail.com"
	}
}

pub struct AppContainer {
	pub service: Arc<dyn MockedService>,
}

#[derive(Clone)]
pub struct AppState(pub Arc<AppContainer>);

impl FromRef<AppState> for Arc<dyn MockedService> {
	fn from_ref(state: &AppState) -> Self {
		state.0.service.clone()
	}
}

pub async fn get_state(service: Arc<dyn MockedService>) -> AppState {
	let container = AppContainer { service };
	AppState(Arc::new(container))
}
