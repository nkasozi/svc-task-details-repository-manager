use async_trait::async_trait;
use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError, recon_tasks_models::ReconFileMetaData,
};

#[automock]
#[async_trait]
pub trait ReconFileDetailsRepositoryInterface: Send + Sync {
    async fn get_recon_file_details(&mut self, file_id: &String)
                                    -> Result<ReconFileMetaData, AppError>;
    async fn create_recon_file_details(
        &mut self,
        file_details: &ReconFileMetaData,
    ) -> Result<String, AppError>;
    async fn update_recon_file_details(
        &mut self,
        file_details: &ReconFileMetaData,
    ) -> Result<ReconFileMetaData, AppError>;
    async fn delete_recon_file_details(&mut self, file_id: &String) -> Result<bool, AppError>;
}
