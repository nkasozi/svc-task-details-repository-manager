use actix_web::{
    get, HttpResponse,
    post,
    web::{self, Path},
};

use crate::internal::{interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface, models::view_models::requests::{
    AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest,
    GetTaskDetailsRequest,
}};
use crate::internal::shared_reconciler_rust_libraries::web_api::utils::{internal_server_error, ok_or_error};
use crate::internal::web_api::utils::setup_service;

#[get("/recon-tasks/{task_id}")]
pub(crate) async fn get_task_details(
    get_task_details_request: Path<GetTaskDetailsRequest>,
) -> HttpResponse {
    let mut service: Box<dyn ReconTaskAggregationServiceInterface> = match setup_service().await {
        Ok(s) => s,
        Err(err) => return internal_server_error(err),
    };

    let task_id = &get_task_details_request.task_id;
    let response = service.get_recon_task(task_id).await;
    return ok_or_error(response);
}

#[post("/recon-tasks")]
pub(crate) async fn create_task_details(
    task_details: web::Json<CreateReconTaskRequest>,
) -> HttpResponse {
    let mut service = match setup_service().await {
        Ok(s) => s,
        Err(err) => return internal_server_error(err),
    };

    let response = service.create_recon_task(&task_details.0).await;
    return ok_or_error(response);
}


#[post("/recon-tasks/attach-primary-file")]
pub(crate) async fn attach_primary_file_to_task(
    task_details: web::Json<AttachPrimaryFileRequest>,
) -> HttpResponse {
    let mut service = match setup_service().await {
        Ok(s) => s,
        Err(err) => return internal_server_error(err),
    };

    let response = service.attach_primary_file_to_task(&task_details.0).await;
    return ok_or_error(response);
}

#[post("/recon-tasks/attach-comparison-file")]
pub(crate) async fn attach_comparison_file_to_task(
    task_details: web::Json<AttachComparisonFileRequest>,
) -> HttpResponse {
    let mut service = match setup_service().await {
        Ok(s) => s,
        Err(err) => return internal_server_error(err),
    };

    let response = service
        .attach_comparison_file_to_task(&task_details.0)
        .await;
    return ok_or_error(response);
}


