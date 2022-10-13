use std::sync::Mutex;

use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::internal::interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppError;
use crate::internal::web_api::handlers::{
    attach_comparison_file_to_task, attach_primary_file_to_task, create_task_details,
    get_task_details,
};
use crate::internal::web_api::utils;
use crate::internal::web_api::utils::setup_service;

pub async fn run_async() -> Result<(), std::io::Error> {
    //retrieve app settings from the env variables
    let app_settings = utils::read_app_settings();

    let app_listen_url = format!("{}:{}", app_settings.app_ip, app_settings.app_port);

    //just for logging purposes
    println!("App is listening on: {:?}", app_listen_url);

    HttpServer::new(async move
        {
            let service = match setup_service().await {
                Ok(s) => s,
                Err(_) => {
                    panic!("cant setup service")
                }
            };
            App::new() // add shared state
                .app_data(Data::new(Mutex::new(service)))
                .service(get_task_details)
                .service(create_task_details)
                .service(attach_primary_file_to_task)
                .service(attach_comparison_file_to_task)
        })
        .bind(app_listen_url)?
        .run()
        .await
}

