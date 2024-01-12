use std::net::ToSocketAddrs;
use tokio::net::UnixStream;

pub async fn connect_uds(path: &str) -> Result<ShardedClient, ClientError> {
    let addr = path.to_socket_addrs()?.next().ok_or(ClientError::InvalidAddress)?;
    let stream = UnixStream::connect(addr).await?;
    let client = ShardedClient::new(stream);
    Ok(client)
}
```

In this modified code:
- We import the necessary dependencies, including `std::net::ToSocketAddrs` and `tokio::net::UnixStream`.
- The `connect_uds` function takes a `path` parameter as input, which represents the path to the Unix domain socket.
- We use the `to_socket_addrs` method to convert the `path` to a `SocketAddr` type. If the conversion fails or the `SocketAddr` is not found, we return an `InvalidAddress` error.
- We establish a connection to the Unix domain socket using `UnixStream::connect` and await the result. If the connection fails, we return the corresponding error.
- Finally, we create a new `ShardedClient` instance with the established stream and return it as a successful result.

Please note that this code assumes the existence of the `ShardedClient` struct and the `ClientError` enum, which should be defined elsewhere in the `sharded_client.rs` file.

Additionally, we need to update the code in the `router/src/main.rs` file to use the modified `connect_uds` function. Specifically, we need to update the line where the `ShardedClient` is connected to the master Unix socket. Replace the existing code with the following:

```rust
let mut sharded_client = connect_uds(&master_shard_uds_path).await.map_err(RouterError::Connection)?;
