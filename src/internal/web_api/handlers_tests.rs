use actix_web::{
    App,
    test::{self, TestRequest},
    web::Data,
};

use crate::internal::{
    interfaces::recon_tasks_aggregator::MockReconTaskAggregationServiceInterface,
    interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
    shared_reconciler_rust_libraries::models::{
        entities::recon_tasks_models::{
            ReconciliationConfigs, ReconFileMetaData, ReconFileType, ReconTaskDetails,
        },
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file_chunk_queue::FileChunkQueue,
    },
    web_api::handlers::get_task_details,
};

#[actix_web::test]
async fn test_get_task_details_calls_correct_dependencies_and_returns_success() {
    let mut app = test::init_service((move || {
        // Create some global state prior to running the handler thread
        let mut mock_recon_task_aggregation_service =
            Box::new(MockReconTaskAggregationServiceInterface::new());

        mock_recon_task_aggregation_service
            .expect_get_recon_task()
            .returning(|_y| Ok(get_dummy_recon_task_response_details()));

        let service: Box<dyn ReconTaskAggregationServiceInterface> =
            mock_recon_task_aggregation_service;

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(get_task_details)
    })())
        .await;

    let resp = TestRequest::get()
        .uri(&format!("/recon-tasks/123456"))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_task_details_when_invalid_request_returns_bad_request() {
    let mut app = test::init_service((move || {
        // Create some global state prior to running the handler thread
        let mut mock_recon_task_aggregation_service =
            Box::new(MockReconTaskAggregationServiceInterface::new());

        mock_recon_task_aggregation_service
            .expect_get_recon_task()
            .returning(|_y| {
                Err(AppError::new(
                    AppErrorKind::BadClientRequest,
                    "invalid request".to_string(),
                ))
            });

        let service: Box<dyn ReconTaskAggregationServiceInterface> =
            mock_recon_task_aggregation_service;

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(get_task_details)
    })())
        .await;

    let resp = TestRequest::get()
        .uri(&format!("/recon-tasks/123456"))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_upload_file_chunk_when_service_returns_error_returns_internal_error() {
    let mut app = test::init_service((move || {
        // Create some global state prior to running the handler thread
        let mut mock_recon_task_aggregation_service =
            Box::new(MockReconTaskAggregationServiceInterface::new());

        mock_recon_task_aggregation_service
            .expect_get_recon_task()
            .returning(|_y| {
                Err(AppError::new(
                    AppErrorKind::InternalError,
                    "Internal server error".to_string(),
                ))
            });

        let service: Box<dyn ReconTaskAggregationServiceInterface> =
            mock_recon_task_aggregation_service;

        App::new()
            .app_data(Data::new(service)) // add shared state
            .service(get_task_details)
    })())
        .await;

    let resp = TestRequest::get()
        .uri(&format!("/recon-tasks/123456"))
        .send_request(&mut app)
        .await;

    assert!(resp.status().is_server_error());
}

fn get_dummy_recon_task_response_details() -> ReconTaskResponseDetails {
    ReconTaskResponseDetails {
        task_id: String::from("task-1234"),
        task_details: ReconTaskDetails {
            id: String::from("task-1234"),
            primary_file_id: Some(String::from("src-file-1234")),
            comparison_file_id: Some(String::from("cmp-file-1234")),
            is_done: false,
            has_begun: true,
            comparison_pairs: vec![],
            recon_config: ReconciliationConfigs {
                should_check_for_duplicate_records_in_comparison_file: true,
                should_reconciliation_be_case_sensitive: true,
                should_ignore_white_space: true,
                should_do_reverse_reconciliation: true,
            },
            recon_results_queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: None,
            },
            primary_file_chunks_queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: None,
            },
            comparison_file_chunks_queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: None,
            },
        },
        primary_file_metadata: Some(ReconFileMetaData {
            id: String::from("src-file-1234"),
            file_name: String::from("src-file-1234"),
            row_count: 1000,
            column_delimiters: vec![],
            recon_file_type: ReconFileType::PrimaryFile,
            column_headers: vec![],
            file_hash: String::from("src-file-1234"),
        }),
        comparison_file_metadata: Some(ReconFileMetaData {
            id: String::from("cmp-file-1234"),
            file_name: String::from("cmp-file-1234"),
            row_count: 1000,
            column_delimiters: vec![],
            recon_file_type: ReconFileType::ComparisonFile,
            column_headers: vec![],
            file_hash: String::from("cmp-file-1234"),
        }),
    }
}
