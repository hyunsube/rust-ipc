use zbus::{Connection, dbus_interface, Result};


struct DataHandler; // Name for server

#[dbus_interface(name = "org.zbus.DataHandler")]
impl DataHandler {
    async fn say_hello(&self, name: &str) -> String { // We will call this method as PascalCase from snake_case --> SayHello
        format!("Hello {}!", name) // Handling part
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    // setup the server
    connection
        .object_server()
        .at("/org/zbus/DataHandler", DataHandler)
        .await?;
    // before requesting the name
    connection
        .request_name("org.zbus.DataHandler")
        .await?;

    loop {
        // do something else, wait forever or timeout here:
        // handling D-Bus messages is done in the background
    }
}