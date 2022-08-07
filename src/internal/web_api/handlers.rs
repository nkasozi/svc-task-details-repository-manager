use actix_web::{
    get, post,
    web::{self, Data, Path},
    HttpResponse,
};

use crate::internal::{
    interfaces::recon_tasks_aggregator::ReconTaskAggregationServiceInterface,
    models::view_models::requests::{CreateReconTaskRequest, GetTaskDetailsRequest},
    shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind,
};

#[get("/task-details/{task_id}")]
async fn get_task_details(
    get_task_details_request: Path<GetTaskDetailsRequest>,
    service: Data<Box<dyn ReconTaskAggregationServiceInterface>>,
) -> HttpResponse {
    let task_id = &get_task_details_request.task_id;
    let recon_task_details = service.get_recon_task(task_id).await;
    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::NotFound => HttpResponse::NotFound().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}

#[post("/task-details")]
async fn create_task_details(
    task_details: web::Json<CreateReconTaskRequest>,
    service: Data<Box<dyn ReconTaskAggregationServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service.create_recon_task(&task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}
