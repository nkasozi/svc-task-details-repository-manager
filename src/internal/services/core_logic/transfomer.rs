use uuid::Uuid;

use crate::internal::{
    interfaces::transformer::TransformerInterface,
    models::view_models::requests::CreateReconTaskRequest,
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

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn build_recon_task_details_response(
        &self,
        task_details: ReconTaskDetails,
        primary_file_metadata: ReconFileMetaData,
        comparison_file_metadata: ReconFileMetaData,
    ) -> ReconTaskResponseDetails {
        return ReconTaskResponseDetails {
            task_id: task_details.id.clone(),
            task_details,
            primary_file_metadata: primary_file_metadata,
            comparison_file_metadata,
            results_queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: Option::None,
            },
        };
    }

    fn get_src_file_details(&self, request: &CreateReconTaskRequest) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: self.generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.primary_file_name.clone(),
            row_count: request.primary_file_row_count,
            recon_file_type: ReconFileType::SourceReconFile,
            file_hash: request.primary_file_hash.clone(),
            column_delimiters: request.primary_file_delimiters.clone(),
            column_headers: request.primary_file_headers.clone(),
            queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: Option::None,
            },
        };
    }

    fn get_comparison_file_details(&self, request: &CreateReconTaskRequest) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: self.generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.comparison_file_name.clone(),
            row_count: request.comparison_file_row_count,
            recon_file_type: ReconFileType::ComparisonReconFile,
            file_hash: request.comparison_file_hash.clone(),
            column_delimiters: request.comparison_file_delimiters.clone(),
            column_headers: request.comparison_file_headers.clone(),
            queue_info: FileChunkQueue {
                topic_id: String::from("test-topic"),
                last_acknowledged_id: Option::None,
            },
        };
    }

    fn get_recon_task_details(
        &self,
        src_file_id: &String,
        cmp_file_id: &String,
        request: &CreateReconTaskRequest,
    ) -> ReconTaskDetails {
        return ReconTaskDetails {
            id: self.generate_uuid(RECON_TASKS_STORE_PREFIX),
            primary_file_id: String::from(src_file_id),
            comparison_file_id: String::from(cmp_file_id),
            is_done: false,
            has_begun: true,
            comparison_pairs: request.comparison_pairs.clone(),
            recon_config: request.recon_configurations.clone(),
        };
    }
}

impl Transformer {
    fn generate_uuid(&self, prefix: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let full_id = String::from(format!("{}-{}", prefix, id));
        return full_id;
    }
}
