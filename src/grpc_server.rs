use tonic::{transport::Server, Request, Response, Status};
use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse};
use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;
use tokio+stream::wrappers::ReceiverStream;
use tokio::sync::mpsc::{Receiver, Sender};
use services::{payment_service_server::{PaymentService. PaymentServiceServer}, PaymentRequest, PaymentResponse, transaction_service_server::{TransactionService, TransactionServiceServer}, TransactionRequest, TransactionResponse};
pub mod services {
    tonic::include_proto!("services");
}

#[derive(Default)]
pub struct MyPaymentService {}

#[tonic::async_trait]
impl PaymentService for MyPaymentService{
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>
    ) -> Result<Response<PaymentResponse>, Status> {
        Ok(Response::new(PaymentResponse { success: true }))
    }
}

#[tokio:main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;
    Ok(())
}