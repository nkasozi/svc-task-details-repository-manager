use uuid::Uuid;

use crate::internal::{
    interfaces::transformer::TransformerInterface,
    models::view_models::requests::{
        AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest,
    },
    shared_reconciler_rust_libraries::models::{
        entities::{
            file_chunk_queue::FileChunkQueue,
            recon_tasks_models::{ReconFileMetaData, ReconFileType, ReconTaskDetails},
        },
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

const RECON_FILE_STORE_PREFIX: &'static str = "RECON-FILE";
const RECON_TASKS_STORE_PREFIX: &'static str = "RECON-TASK";
const PRIMARY_FILE_QUEUE_PREFIX: &'static str = "PRIMARY-FILE-QUEUE";
const COMPARISON_FILE_QUEUE_PREFIX: &'static str = "COMPARISON-FILE-QUEUE";
const RECON_RESULTS_QUEUE_PREFIX: &'static str = "RECON-RESULTS-QUEUE";

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn build_recon_task_details_response(
        &self,
        task_details: ReconTaskDetails,
        primary_file_metadata: Option<ReconFileMetaData>,
        comparison_file_metadata: Option<ReconFileMetaData>,
    ) -> ReconTaskResponseDetails {
        return ReconTaskResponseDetails {
            task_id: task_details.id.clone(),
            task_details,
            primary_file_metadata,
            comparison_file_metadata,
        };
    }

    fn get_primary_file_details(&self, request: &AttachPrimaryFileRequest) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: self.generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.primary_file_name.clone(),
            row_count: request.primary_file_row_count,
            recon_file_type: ReconFileType::PrimaryFile,
            file_hash: request.primary_file_hash.clone(),
            column_delimiters: request.primary_file_delimiters.clone(),
            column_headers: request.primary_file_headers.clone(),
        };
    }

    fn get_comparison_file_details(
        &self,
        request: &AttachComparisonFileRequest,
    ) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: self.generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.comparison_file_name.clone(),
            row_count: request.comparison_file_row_count,
            recon_file_type: ReconFileType::ComparisonFile,
            file_hash: request.comparison_file_hash.clone(),
            column_delimiters: request.comparison_file_delimiters.clone(),
            column_headers: request.comparison_file_headers.clone(),
        };
    }

    fn get_recon_task_details(&self, request: &CreateReconTaskRequest) -> ReconTaskDetails {
        let task_id = self.generate_uuid(RECON_TASKS_STORE_PREFIX);
        return ReconTaskDetails {
            id: task_id.clone(),
            primary_file_id: None,
            comparison_file_id: None,
            is_done: false,
            has_begun: true,
            comparison_pairs: request.comparison_pairs.clone(),
            recon_config: request.recon_configurations.clone(),
            recon_results_queue_info: self.generate_queue_topic(RECON_RESULTS_QUEUE_PREFIX, &task_id),
            primary_file_chunks_queue_info: self.generate_queue_topic(PRIMARY_FILE_QUEUE_PREFIX, &task_id),
            comparison_file_chunks_queue_info: self.generate_queue_topic(COMPARISON_FILE_QUEUE_PREFIX, &task_id),
        };
    }
}

impl Transformer {
    fn generate_uuid(&self, prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }

    fn generate_queue_topic(&self, prefix: &str, task_id: &String) -> FileChunkQueue {
        let uuid = String::from(format!("{}-{}", prefix, task_id));
        FileChunkQueue {
            topic_id: uuid,
            last_acknowledged_id: Option::None,
        }
    }
}
