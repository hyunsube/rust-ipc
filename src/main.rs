use zbus::{Connection, dbus_interface, Result};


struct Greeter;

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    // setup the server
    connection
        .object_server()
        .at("/org/zbus/MyGreeter", Greeter)
        .await?;
    // before requesting the name
    connection
        .request_name("org.zbus.MyGreeter")
        .await?;

    loop {
        // do something else, wait forever or timeout here:
        // handling D-Bus messages is done in the background
        std::future::pending::<()>().await;
    }
}