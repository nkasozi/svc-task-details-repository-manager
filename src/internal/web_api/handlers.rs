use actix_web::{
    get, HttpResponse,
    post,
    web::{self, Path},
};

use crate::internal::{
    interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
    models::view_models::requests::{
        AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest,
        GetTaskDetailsRequest,
    },
    shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind,
};
use crate::internal::web_api::utils;

#[get("/recon-tasks/{task_id}")]
pub(crate) async fn get_task_details(
    get_task_details_request: Path<GetTaskDetailsRequest>,
) -> HttpResponse {
    let mut service: Box<dyn ReconTaskAggregationServiceInterface> = match utils::setup_service().await {
        Ok(s) => s,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{}", err)),
    };

    let task_id = &get_task_details_request.task_id;
    let recon_task_details = service.get_recon_task(task_id).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::NotFound => HttpResponse::NotFound().json(format!("{}", err)),
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}

#[post("/recon-tasks")]
pub(crate) async fn create_task_details(
    task_details: web::Json<CreateReconTaskRequest>,
) -> HttpResponse {
    let mut service = match utils::setup_service().await {
        Ok(s) => s,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{}", err)),
    };

    let recon_task_details = service.create_recon_task(&task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}


#[post("/recon-tasks/attach-primary-file")]
pub(crate) async fn attach_primary_file_to_task(
    task_details: web::Json<AttachPrimaryFileRequest>,
) -> HttpResponse {
    let mut service = match utils::setup_service().await {
        Ok(s) => s,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{}", err)),
    };

    let recon_task_details = service.attach_primary_file_to_task(&task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}

#[post("/recon-tasks/attach-comparison-file")]
pub(crate) async fn attach_comparison_file_to_task(
    task_details: web::Json<AttachComparisonFileRequest>,
) -> HttpResponse {
    let mut service = match utils::setup_service().await {
        Ok(s) => s,
        Err(err) => return HttpResponse::InternalServerError().json(format!("{}", err)),
    };

    let recon_task_details = service
        .attach_comparison_file_to_task(&task_details.0)
        .await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}


