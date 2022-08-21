use mockall::automock;

use crate::internal::{
    models::view_models::requests::CreateReconTaskRequest,
    shared_reconciler_rust_libraries::models::{
        entities::recon_tasks_models::{ReconFileMetaData, ReconTaskDetails},
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

#[automock]
pub trait TransformerInterface: Send + Sync {
    fn build_recon_task_details_response(
        &self,
        task_details: ReconTaskDetails,
        primary_file_metadata: ReconFileMetaData,
        comparison_file_metadata: ReconFileMetaData,
    ) -> ReconTaskResponseDetails;

    fn get_src_file_details(&self, request: &CreateReconTaskRequest) -> ReconFileMetaData;

    fn get_comparison_file_details(&self, request: &CreateReconTaskRequest) -> ReconFileMetaData;

    fn get_recon_task_details(
        &self,
        src_file_id: &String,
        cmp_file_id: &String,
        request: &CreateReconTaskRequest,
    ) -> ReconTaskDetails;
}
