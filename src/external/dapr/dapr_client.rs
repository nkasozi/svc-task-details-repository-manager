use dapr::{Client, dapr::dapr::proto::runtime::v1::dapr_client::DaprClient};
use tonic::transport::Channel as TonicChannel;

use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind};

pub async fn connect_to_dapr(connection_url: &String) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
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