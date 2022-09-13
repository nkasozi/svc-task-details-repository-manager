use async_trait::async_trait;
use dapr::{Client, dapr::dapr::proto::runtime::v1::dapr_client::DaprClient};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::recon_files_repository::ReconFileDetailsRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        recon_tasks_models::ReconFileMetaData,
    },
};

pub struct ReconFileDetailsRepositoryManager {
    pub store_name: String,
    pub client: Client<DaprClient<TonicChannel>>,
}

#[async_trait]
impl ReconFileDetailsRepositoryInterface for ReconFileDetailsRepositoryManager {
    async fn get_recon_file_details(
        &mut self,
        file_id: &String,
    ) -> Result<ReconFileMetaData, AppError> {
        let get_response = self.client
            .get_state(self.store_name.clone(), String::from(file_id), None)
            .await;

        return match get_response {
            Ok(s) => {
                let retrieval_result: Result<ReconFileMetaData, _> =
                    serde_json::from_slice(&s.data);

                match retrieval_result {
                    Ok(unmarshalled_file_details) => Ok(unmarshalled_file_details),
                    Err(e) => Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
                }
            }
            Err(e) => Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        };
    }

    async fn create_recon_file_details(
        &mut self,
        file_details: &ReconFileMetaData,
    ) -> Result<String, AppError> {
        let key = file_details.id.clone();
        let val = serde_json::to_vec(&file_details).unwrap();

        // save key-value pair in the state store
        let save_result = self.client
            .save_state(self.store_name.clone(), vec![(key.clone(), val)])
            .await;

        return match save_result {
            Ok(_s) => Ok(key.clone()),
            Err(e) => Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        };
    }

    async fn update_recon_file_details(
        &mut self,
        file_details: &ReconFileMetaData,
    ) -> Result<ReconFileMetaData, AppError> {
        //delete existing task task_details
        let _ = self.delete_recon_file_details(&file_details.id);

        //save new details
        let id = self.create_recon_file_details(file_details).await?;

        //return task details
        return self.get_recon_file_details(&id).await;
    }

    async fn delete_recon_file_details(&mut self, file_id: &String) -> Result<bool, AppError> {

        // delete a value from the state store
        let delete_result = self.client
            .delete_state(self.store_name.clone(), String::from(file_id), None)
            .await;

        return match delete_result {
            Ok(_s) => Ok(true),
            Err(e) => Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        };
    }
}

impl ReconFileDetailsRepositoryManager {
    pub(crate) fn new(store_name: String, client: Client<DaprClient<TonicChannel>>) -> Self {
        //handle the connection result
        return ReconFileDetailsRepositoryManager {
            store_name,
            client,
        };
    }
}
