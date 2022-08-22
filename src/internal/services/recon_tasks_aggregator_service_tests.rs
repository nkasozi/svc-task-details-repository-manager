use crate::internal::{
    interfaces::{
        recon_files_repository::MockReconFileDetailsRepositoryInterface,
        recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
        recon_tasks_repository::MockReconTaskDetailsRepositoryInterface,
        transformer::MockTransformerInterface,
    },
    models::view_models::requests::CreateReconTaskRequest,
    shared_reconciler_rust_libraries::models::{
        entities::{
            app_errors::{AppError, AppErrorKind},
            file_chunk_queue::FileChunkQueue,
            recon_tasks_models::{
                ComparisonPair, ReconFileMetaData, ReconFileType, ReconTaskDetails,
                ReconciliationConfigs,
            },
        },
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

use super::recon_tasks_aggregator_service::ReconTaskAggregationService;

#[actix_web::test]
async fn given_valid_create_recon_task_request_calls_correct_dependencies_returns_success() {
    //setup
    let (mock_recon_task_details_repo, mock_recon_file_details_repo, mock_transformer) =
        setup_dependencies();

    let service = ReconTaskAggregationService {
        recon_task_details_repo: mock_recon_task_details_repo,
        recon_file_details_repo: mock_recon_file_details_repo,
        transformer: mock_transformer,
    };

    let test_request = get_dummy_create_recon_task();

    let expected = String::from("task-1234");

    //act
    let result = service.create_recon_task(&test_request).await;

    //assert
    assert!(result.is_ok());
    assert_eq!(result.ok().unwrap().task_id, expected);
}

#[actix_web::test]
async fn given_that_invalid_create_recon_task_request_supplied_returns_error() {
    //setup
    let (mock_recon_task_details_repo, mock_recon_file_details_repo, mock_transformer) =
        setup_dependencies();

    let service = ReconTaskAggregationService {
        recon_task_details_repo: mock_recon_task_details_repo,
        recon_file_details_repo: mock_recon_file_details_repo,
        transformer: mock_transformer,
    };

    let mut test_request = get_dummy_create_recon_task();
    test_request.user_id = String::from("");

    //act
    let result = service.create_recon_task(&test_request).await;

    //assert
    assert!(result.is_err())
}

#[actix_web::test]
async fn given_that_errors_occurs_when_handling_create_recon_task_request_returns_error() {
    //setup
    let (mock_recon_task_details_repo, _, mock_transformer) = setup_dependencies();

    let mut mock_recon_file_details_repo = Box::new(MockReconFileDetailsRepositoryInterface::new());
    mock_recon_file_details_repo
        .expect_create_recon_file_details()
        .returning(|_y| {
            Err(AppError::new(
                AppErrorKind::ConnectionError,
                "unable to connect".to_string(),
            ))
        });

    let service = ReconTaskAggregationService {
        recon_task_details_repo: mock_recon_task_details_repo,
        recon_file_details_repo: mock_recon_file_details_repo,
        transformer: mock_transformer,
    };

    let test_request = get_dummy_create_recon_task();

    //act
    let result = service.create_recon_task(&test_request).await;

    //assert
    assert!(result.is_err())
}

fn setup_dependencies() -> (
    Box<MockReconTaskDetailsRepositoryInterface>,
    Box<MockReconFileDetailsRepositoryInterface>,
    Box<MockTransformerInterface>,
) {
    let mut mock_recon_file_details_repo = Box::new(MockReconFileDetailsRepositoryInterface::new());
    let mut mock_recon_task_details_repo = Box::new(MockReconTaskDetailsRepositoryInterface::new());
    let mut mock_transformer = Box::new(MockTransformerInterface::new());

    mock_recon_task_details_repo
        .expect_create_task_details()
        .returning(|_y| Ok(String::from("task-1234")));

    mock_recon_task_details_repo
        .expect_get_task_details()
        .returning(|_y| Ok(get_dummy_recon_task_details()));

    mock_recon_file_details_repo
        .expect_create_recon_file_details()
        .returning(|_y| Ok(String::from("file-1234")));

    mock_recon_file_details_repo
        .expect_get_recon_file_details()
        .returning(|_y| Ok(get_dummy_recon_file_metadata()));

    mock_transformer
        .expect_build_recon_task_details_response()
        .returning(|_, _, _| get_dummy_recon_task_response_details());

    mock_transformer
        .expect_get_comparison_file_details()
        .returning(|_| get_dummy_recon_file_metadata());

    mock_transformer
        .expect_get_primary_file_details()
        .returning(|_| get_dummy_recon_file_metadata());

    mock_transformer
        .expect_get_recon_task_details()
        .returning(|_, _, _| get_dummy_recon_task_details());

    return (
        mock_recon_task_details_repo,
        mock_recon_file_details_repo,
        mock_transformer,
    );
}

fn get_dummy_create_recon_task() -> CreateReconTaskRequest {
    CreateReconTaskRequest {
        user_id: String::from("test-user-id"),
        primary_file_name: String::from("test-src-file"),
        primary_file_hash: String::from("test-src-file-hash"),
        primary_file_row_count: 1000,
        comparison_file_name: String::from("test-cmp-file"),
        comparison_file_hash: String::from("test-src-file-hash"),
        comparison_file_row_count: 10,
        recon_configurations: ReconciliationConfigs {
            should_check_for_duplicate_records_in_comparison_file: false,
            should_reconciliation_be_case_sensitive: true,
            should_ignore_white_space: true,
            should_do_reverse_reconciliation: false,
        },
        comparison_pairs: vec![ComparisonPair {
            primary_file_column_index: 0,
            comparison_file_column_index: 0,
            is_row_identifier: true,
        }],
        primary_file_headers: vec![String::from("src-file-header-1")],
        primary_file_delimiters: vec![String::from(",")],
        comparison_file_headers: vec![String::from("cmp-file-header-1")],
        comparison_file_delimiters: vec![String::from(",")],
    }
}

fn get_dummy_recon_file_metadata() -> ReconFileMetaData {
    ReconFileMetaData {
        id: String::from("src-file-1234"),
        file_name: String::from("src-file-1234"),
        row_count: 1000,
        column_delimiters: vec![],
        recon_file_type: ReconFileType::PrimaryFile,
        column_headers: vec![String::from("header1"), String::from("header2")],
        file_hash: String::from("src-file-1234"),
        queue_info: FileChunkQueue {
            topic_id: String::from("test-topic"),
            last_acknowledged_id: Option::None,
        },
    }
}

fn get_dummy_recon_task_details() -> ReconTaskDetails {
    ReconTaskDetails {
        id: String::from("task-1234"),
        primary_file_id: String::from("src-file-1234"),
        comparison_file_id: String::from("cmp-file-1234"),
        is_done: false,
        has_begun: false,
        comparison_pairs: vec![ComparisonPair {
            primary_file_column_index: 0,
            comparison_file_column_index: 0,
            is_row_identifier: true,
        }],
        recon_config: ReconciliationConfigs {
            should_check_for_duplicate_records_in_comparison_file: true,
            should_reconciliation_be_case_sensitive: true,
            should_ignore_white_space: true,
            should_do_reverse_reconciliation: true,
        },
        recon_results_queue_info: FileChunkQueue {
            topic_id: String::from("test-topic"),
            last_acknowledged_id: Option::None,
        },
    }
}

fn get_dummy_recon_task_response_details() -> ReconTaskResponseDetails {
    ReconTaskResponseDetails {
        task_id: String::from("task-1234"),
        task_details: ReconTaskDetails {
            id: String::from("task-1234"),
            primary_file_id: String::from("src-file-1234"),
            comparison_file_id: String::from("cmp-file-1234"),
            is_done: false,
            has_begun: true,
            comparison_pairs: vec![new_same_column_index_comparison_pair(0)],
            recon_config: default_recon_configs(),
            recon_results_queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: Option::None,
            },
        },
        primary_file_metadata: ReconFileMetaData {
            id: String::from("src-file-1234"),
            file_name: String::from("src-file-1234"),
            row_count: 1000,
            column_delimiters: vec![],
            recon_file_type: ReconFileType::PrimaryFile,
            column_headers: vec![String::from("header1"), String::from("header2")],
            file_hash: String::from("src-file-1234"),
            queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: Option::None,
            },
        },
        comparison_file_metadata: ReconFileMetaData {
            id: String::from("cmp-file-1234"),
            file_name: String::from("cmp-file-1234"),
            row_count: 1000,
            column_delimiters: vec![String::from(",")],
            recon_file_type: ReconFileType::ComparisonFile,
            column_headers: vec![String::from("header1"), String::from("header2")],
            file_hash: String::from("cmp-file-1234"),
            queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: Option::None,
            },
        },
    }
}

fn default_recon_configs() -> ReconciliationConfigs {
    ReconciliationConfigs {
        should_check_for_duplicate_records_in_comparison_file: true,
        should_reconciliation_be_case_sensitive: true,
        should_ignore_white_space: true,
        should_do_reverse_reconciliation: true,
    }
}

fn new_same_column_index_comparison_pair(column_index: usize) -> ComparisonPair {
    ComparisonPair {
        primary_file_column_index: 0,
        comparison_file_column_index: 0,
        is_row_identifier: true,
    }
}
