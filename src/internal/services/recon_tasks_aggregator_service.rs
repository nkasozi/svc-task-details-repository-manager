use async_trait::async_trait;
use validator::Validate;

use crate::internal::{
    interfaces::{
        recon_files_repository::ReconFileDetailsRepositoryInterface,
        recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
        recon_tasks_repository::ReconTaskDetailsRepositoryInterface,
        transformer::TransformerInterface,
    },
    models::view_models::requests::{
        AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest,
    },
    shared_reconciler_rust_libraries::models::{
        entities::app_errors::{AppError, AppErrorKind},
        view_models::recon_task_response_details::{FileResponseSummary, ReconTaskResponseDetails},
    },
};

pub struct ReconTaskAggregationService {
    pub recon_task_details_repo: Box<dyn ReconTaskDetailsRepositoryInterface>,
    pub recon_file_details_repo: Box<dyn ReconFileDetailsRepositoryInterface>,
    pub transformer: Box<dyn TransformerInterface>,
}

#[async_trait]
impl ReconTaskAggregationServiceInterface for ReconTaskAggregationService {
    async fn create_recon_task(
        &self,
        request: &CreateReconTaskRequest,
    ) -> Result<ReconTaskResponseDetails, AppError> {
        //validate request
        match request.validate() {
            Ok(_) => (),
            Err(e) => {
                return Err(AppError::new(
                    AppErrorKind::BadClientRequest,
                    e.to_string().replace("\n", " , "),
                ));
            }
        }

        //save recon task details
        let recon_task_details = self.transformer.get_recon_task_details(&request);

        let task_id = self
            .recon_task_details_repo
            .create_task_details(&recon_task_details)
            .await?;

        //retrieve saved details
        return self.get_recon_task(&task_id).await;
    }

    async fn get_recon_task(&self, task_id: &String) -> Result<ReconTaskResponseDetails, AppError> {
        //validate request
        if task_id.is_empty() {
            return Err(AppError::new(
                AppErrorKind::BadClientRequest,
                String::from("please supply a taskID"),
            ));
        }

        //fetch details from repository
        let task_details = self
            .recon_task_details_repo
            .get_task_details(task_id)
            .await?;

        //fetch src file from repository
        let src_file_metadata  = match task_details.primary_file_id.clone() {
            Some(primary_file_id) => {
                let primary_file_metadata = self.recon_file_details_repo.get_recon_file_details(&primary_file_id).await?;
                Some(primary_file_metadata)
            },
            None => None
        };

        //fetch cmp file from repository
        let cmp_file_metadata = match task_details.comparison_file_id.clone() {
            Some(comparison_file_id) => {
                let comparison_file_metadata = self.recon_file_details_repo.get_recon_file_details(&comparison_file_id).await?;
                Some(comparison_file_metadata)
            },
            None => None
        };



        //convert details to view model
        let task_details_response: ReconTaskResponseDetails = self
            .transformer
            .build_recon_task_details_response(task_details, src_file_metadata, cmp_file_metadata);

        //return success
        return Ok(task_details_response);
    }

    async fn attach_primary_file_to_task(
        &self,
        request: &AttachPrimaryFileRequest,
    ) -> Result<FileResponseSummary, AppError> {
        //transform into primary file details
        let primary_file_details = self.transformer.get_primary_file_details(&request);

        //save the file details
        let primary_file_id = self
            .recon_file_details_repo
            .create_recon_file_details(&primary_file_details)
            .await?;

        //retrieve saved task details
        let recon_task_details = self.get_recon_task(&request.task_id.clone()).await?;
        let mut recon_task = recon_task_details.task_details;
        recon_task.primary_file_id = Some(primary_file_id.clone());

        //update the task
        let _ = self
            .recon_task_details_repo
            .update_task_details(&recon_task)
            .await?;

        Ok(FileResponseSummary {
            file_id: primary_file_id.clone(),
            task_id: request.task_id.clone(),
        })
    }

    async fn attach_comparison_file_to_task(
        &self,
        request: &AttachComparisonFileRequest,
    ) -> Result<FileResponseSummary, AppError> {
        //transform into primary file details
        let comparison_file_details = self.transformer.get_comparison_file_details(&request);

        //save the file details
        let comparison_file_id = self
            .recon_file_details_repo
            .create_recon_file_details(&comparison_file_details)
            .await?;

        //retrieve saved task details
        let recon_task_details = self.get_recon_task(&request.task_id.clone()).await?;
        let mut recon_task = recon_task_details.task_details;
        recon_task.comparison_file_id = Some(comparison_file_id.clone());

        //update the task
        let _ = self
            .recon_task_details_repo
            .update_task_details(&recon_task)
            .await?;

        Ok(FileResponseSummary {
            file_id: comparison_file_id.clone(),
            task_id: request.task_id.clone(),
        })
    }
}
