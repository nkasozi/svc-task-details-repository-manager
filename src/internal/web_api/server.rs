use actix_web::{web::Data, App, HttpServer};

use crate::{
    external::repositories::{
        recon_file_details_repo::ReconFileDetailsRepositoryManager,
        recon_task_details_repo::ReconTaskDetailsRepositoryManager,
    },
    internal::{
        interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
        services::{
            core_logic::transfomer::Transformer,
            recon_tasks_aggregator_service::ReconTaskAggregationService,
        },
        web_api::handlers::{
            attach_comparison_file_to_task, attach_primary_file_to_task, create_task_details,
            get_task_details,
        },
    },
};

const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_DAPR_STORE_NAME: &'static str = "statestore";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

#[derive(Clone, Debug)]
struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_state_store_name: String,

    pub dapr_grpc_server_address: String,
}

pub async fn run_async() -> Result<(), std::io::Error> {
    //retrieve app settings from the env variables
    let app_settings = read_app_settings();

    let app_listen_url = format!("{}:{}", app_settings.app_ip, app_settings.app_port);

    //just for logging purposes
    println!("App is listening on: {:?}", app_listen_url);

    HttpServer::new(move || {
        // Create some global state prior to running the handler threads
        let service = setup_service(app_settings.clone());

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(get_task_details)
            .service(create_task_details)
            .service(attach_primary_file_to_task)
            .service(attach_comparison_file_to_task)
    })
    .bind(app_listen_url)?
    .run()
    .await
}

fn setup_service(app_settings: AppSettings) -> Box<dyn ReconTaskAggregationServiceInterface> {
    let service: Box<dyn ReconTaskAggregationServiceInterface> =
        Box::new(ReconTaskAggregationService {
            recon_task_details_repo: Box::new(ReconTaskDetailsRepositoryManager {
                connection_url: app_settings.dapr_grpc_server_address.clone(),
                store_name: app_settings.dapr_state_store_name.clone(),
            }),

            recon_file_details_repo: Box::new(ReconFileDetailsRepositoryManager {
                connection_url: app_settings.dapr_grpc_server_address.clone(),
                store_name: app_settings.dapr_state_store_name.clone(),
            }),

            transformer: Box::new(Transformer {}),
        });
    service
}

fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: std::env::var("APP_PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string()),

        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),

        dapr_grpc_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),

        dapr_state_store_name: std::env::var("DAPR_RECON_TASKS_STORE_NAME")
            .unwrap_or(DEFAULT_DAPR_STORE_NAME.to_string()),
    }
}
