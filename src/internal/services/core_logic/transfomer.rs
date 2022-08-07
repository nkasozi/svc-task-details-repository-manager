use uuid::Uuid;

use crate::internal::{
    interfaces::transformer::TransformerInterface,
    models::view_models::{requests::CreateReconTaskRequest, responses::ReconTaskResponseDetails},
    shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{
        ReconFileMetaData, ReconFileType, ReconTaskDetails,
    },
};

const RECON_FILE_STORE_PREFIX: &'static str = "RECON-FILE";
const RECON_TASKS_STORE_PREFIX: &'static str = "RECON-TASK";

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn build_recon_task_details_response(
        &self,
        task_details: ReconTaskDetails,
        source_file_metadata: ReconFileMetaData,
        comparison_file_metadata: ReconFileMetaData,
    ) -> ReconTaskResponseDetails {
        return ReconTaskResponseDetails {
            task_id: task_details.id.clone(),
            task_details,
            source_file_metadata,
            comparison_file_metadata,
        };
    }

    fn get_src_file_details(&self, request: &CreateReconTaskRequest) -> ReconFileMetaData {
        return ReconFileMetaData {
            id: self.generate_uuid(RECON_FILE_STORE_PREFIX),
            file_name: request.source_file_name.clone(),
            row_count: request.source_file_row_count,
            recon_file_type: ReconFileType::SourceReconFile,
            file_hash: request.source_file_hash.clone(),
            column_delimiters: request.source_file_delimiters.clone(),
            column_headers: request.source_file_headers.clone(),
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
            source_file_id: String::from(src_file_id),
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
