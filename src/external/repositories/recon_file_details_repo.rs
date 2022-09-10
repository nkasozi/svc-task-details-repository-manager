use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::recon_files_repository::ReconFileDetailsRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        recon_tasks_models::ReconFileMetaData,
    },
};

pub struct ReconFileDetailsRepositoryManager {
    pub connection_url: String,
    pub store_name: String,
}

#[async_trait]
impl ReconFileDetailsRepositoryInterface for ReconFileDetailsRepositoryManager {
    async fn get_recon_file_details(
        &self,
        file_id: &String,
    ) -> Result<ReconFileMetaData, AppError> {
        // Create the client
        let mut client = self.get_dapr_connection().await?;

        let get_response = client
            .get_state(self.store_name.clone(), String::from(file_id), None)
            .await;

        match get_response {
            Ok(s) => {
                let retrieval_result: Result<ReconFileMetaData, _> =
                    serde_json::from_slice(&s.data);
                match retrieval_result {
                    Ok(unmarshalled_file_details) => return Ok(unmarshalled_file_details),
                    Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
                };
            }
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }

    async fn create_recon_file_details(
        &self,
        file_details: &ReconFileMetaData,
    ) -> Result<String, AppError> {
        // Create the client
        let mut client = self.get_dapr_connection().await?;

        let key = file_details.id.clone();
        let val = serde_json::to_vec(&file_details).unwrap();

        // save key-value pair in the state store
        let save_result = client
            .save_state(self.store_name.clone(), vec![(key.clone(), val)])
            .await;

        match save_result {
            Ok(_s) => return Ok(key.clone()),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }

    async fn update_recon_file_details(
        &self,
        file_details: &ReconFileMetaData,
    ) -> Result<ReconFileMetaData, AppError> {
        //delete existing task task_details
        let _ = self.delete_recon_file_details(&file_details.id);

        //save new details
        let id = self.create_recon_file_details(file_details).await?;

        //return task details
        return self.get_recon_file_details(&id).await;
    }

    async fn delete_recon_file_details(&self, file_id: &String) -> Result<bool, AppError> {
        // Create the client
        let mut client = self.get_dapr_connection().await?;

        // delete a value from the state store
        let delete_result = client
            .delete_state(self.store_name.clone(), String::from(file_id), None)
            .await;

        match delete_result {
            Ok(_s) => return Ok(true),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }
}

impl ReconFileDetailsRepositoryManager {
    async fn get_dapr_connection(&self) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = self.connection_url.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        match client_connect_result {
            //connection succeeded
            Ok(s) => return Ok(s),
            //connection failed
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}
