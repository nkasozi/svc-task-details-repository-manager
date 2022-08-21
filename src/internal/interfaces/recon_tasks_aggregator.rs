use async_trait::async_trait;
use mockall::automock;

use crate::internal::{
    models::view_models::requests::CreateReconTaskRequest,
    shared_reconciler_rust_libraries::models::{
        entities::app_errors::AppError,
        view_models::recon_task_response_details::ReconTaskResponseDetails,
    },
};

#[automock]
#[async_trait]
pub trait ReconTaskAggregationServiceInterface {
    async fn create_recon_task(
        &self,
        request: &CreateReconTaskRequest,
    ) -> Result<ReconTaskResponseDetails, AppError>;

    async fn get_recon_task(&self, task_id: &String) -> Result<ReconTaskResponseDetails, AppError>;
}
