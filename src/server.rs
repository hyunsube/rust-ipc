use std::{error::Error, future::pending};
use zbus::{ConnectionBuilder, dbus_interface};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    value: i32,
}

struct DataStruct {
}

#[dbus_interface(name = "org.zbus.DataIPC")]
impl DataStruct {
    // Can be `async` as well.
    fn send_string(&mut self, data: &str) -> String {
        println!("String data has been received!. {data}");
        format!("Receive string: {}", data)
    }

    fn send_data(&mut self, serialized: &str) -> String {
        let deserialized: Data = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        format!("Receive json string")
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data_struct = DataStruct {};

    let _conn = ConnectionBuilder::session()?
        .name("org.zbus.DataIPC")?
        .serve_at("/org/zbus/DataIPC", data_struct)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}