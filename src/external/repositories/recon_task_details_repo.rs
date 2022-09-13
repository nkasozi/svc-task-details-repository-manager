use async_trait::async_trait;
use dapr::{Client, dapr::dapr::proto::runtime::v1::dapr_client::DaprClient};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::recon_tasks_repository::ReconTaskDetailsRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        recon_tasks_models::ReconTaskDetails,
    },
};

pub struct ReconTaskDetailsRepositoryManager {
    pub connection_url: String,
    pub store_name: String,
    pub client: Client<DaprClient<TonicChannel>>,
}

#[async_trait]
impl ReconTaskDetailsRepositoryInterface for ReconTaskDetailsRepositoryManager {
    async fn get_task_details(&mut self, task_id: &String) -> Result<ReconTaskDetails, AppError> {
        let get_response = self.client
            .get_state(self.store_name.clone(), task_id.clone(), None)
            .await;

        return match get_response {
            Ok(s) => {
                let retrieval_result: Result<ReconTaskDetails, _> = serde_json::from_slice(&s.data);

                match retrieval_result {
                    Ok(unmarshalled_task_details) => Ok(unmarshalled_task_details),
                    Err(e) => Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
                }
            }
            Err(e) => Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        };
    }

    async fn create_task_details(
        &mut self,
        task_details: &ReconTaskDetails,
    ) -> Result<String, AppError> {
        let key = task_details.id.clone();
        let val = serde_json::to_vec(&task_details).unwrap();

        // save key-value pair in the state store
        let save_result = self.client
            .save_state(self.store_name.clone(), vec![(key.clone(), val)])
            .await;

        return match save_result {
            Ok(_s) => Ok(key.clone()),
            Err(e) => Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        };
    }

    async fn update_task_details(
        &mut self,
        task_details: &ReconTaskDetails,
    ) -> Result<ReconTaskDetails, AppError> {
        //delete existing task task_details
        let _ = self.delete_task_details(&task_details.id);

        //save new details
        let id = self.create_task_details(task_details).await?;

        //return task details
        return self.get_task_details(&id).await;
    }

    async fn delete_task_details(&mut self, task_details_id: &String) -> Result<bool, AppError> {


        // delete a value from the state store
        let delete_result = self.client
            .delete_state(self.store_name.clone(), String::from(task_details_id), None)
            .await;

        return match delete_result {
            Ok(_s) => Ok(true),
            Err(e) => Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        };
    }
}

impl ReconTaskDetailsRepositoryManager {
    pub(crate) async fn new(connection_url: String, store_name: String) -> Result<Self, std::io::Error> {
        let client_connect_result = ReconTaskDetailsRepositoryManager::connect_to_dapr(&connection_url).await;

        //handle the connection result
        return match client_connect_result {
            //connection succeeded
            Ok(s) => {
                Ok(ReconTaskDetailsRepositoryManager {
                    connection_url,
                    store_name,
                    client: s,
                })
            }

            //connection failed
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "unable to connect to dapr")),
        };
    }

    async fn connect_to_dapr(connection_url: &String) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = connection_url.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        return match client_connect_result {
            //connection succeeded
            Ok(s) => Ok(s),
            //connection failed
            Err(e) => Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        };
    }
}

#[cfg(test)]
mod tests {
    #[actix_rt::test]
    async fn given_valid_create_recon_task_request_calls_correct_dependencies_returns_success() {}
}
