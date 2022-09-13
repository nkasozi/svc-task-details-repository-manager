use crate::external::repositories::recon_file_details_repo::ReconFileDetailsRepositoryManager;
use crate::external::repositories::recon_task_details_repo::ReconTaskDetailsRepositoryManager;
use crate::internal::interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface;
use crate::internal::services::core_logic::transfomer::Transformer;
use crate::internal::services::recon_tasks_aggregator_service::ReconTaskAggregationService;

const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_DAPR_STORE_NAME: &'static str = "statestore";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

#[derive(Clone, Debug)]
pub struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_state_store_name: String,

    pub dapr_grpc_server_address: String,
}

pub async fn setup_service() -> Result<Box<dyn ReconTaskAggregationServiceInterface>, std::io::Error> {
    let app_settings = read_app_settings();

    let recon_repo = ReconTaskDetailsRepositoryManager::new(
        app_settings.dapr_grpc_server_address.clone(),
        app_settings.dapr_state_store_name.clone(),
    ).await?;

    let service: Box<dyn ReconTaskAggregationServiceInterface> =
        Box::new(ReconTaskAggregationService {
            recon_task_details_repo: Box::new(recon_repo),

            recon_file_details_repo: Box::new(ReconFileDetailsRepositoryManager {
                connection_url: app_settings.dapr_grpc_server_address.clone(),
                store_name: app_settings.dapr_state_store_name.clone(),
            }),

            transformer: Box::new(Transformer {}),
        });

    Ok(service)
}

pub fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: std::env::var("APP_PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string()),

        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),

        dapr_grpc_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),

        dapr_state_store_name: std::env::var("DAPR_RECON_TASKS_STORE_NAME")
            .unwrap_or(DEFAULT_DAPR_STORE_NAME.to_string()),
    }
}