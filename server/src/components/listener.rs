use crate::server::Server;
use crate::server_command::ServerCommand;
use crate::server_error::ServerError;
use bytes::BytesMut;
use tracing::{error, trace};

impl Server {
    pub async fn start_listener(&self) -> Result<(), ServerError> {
        loop {
            let mut buffer = BytesMut::zeroed(1024);
            let (length, address) = self.socket.recv_from(&mut buffer).await?;
            buffer.truncate(length);
            trace!("{:?} bytes received from {:?}", length, address);
            if let Err(error) = self
                .sender
                .send(ServerCommand::HandleRequest(buffer.freeze(), address))
                .await
            {
                error!("Error when handling the request: {:?}", error);
            }
        }
    }
}
