use zbus::{Connection, Result, dbus_proxy};
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    value: i32,
}

#[dbus_proxy(
    interface = "org.zbus.DataIPC",
    default_service = "org.zbus.DataIPC",
    default_path = "/org/zbus/DataIPC"
)]
trait DataStruct {
    async fn send_string(&self, name: &str) -> Result<String>;
    async fn send_data(&self, data: &str) -> Result<String>;
}

fn print_description() {
    println!("-----------------------------");
    println!("s: send string");
    println!("d: send json data");
    println!("q: quit");
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = DataStructProxy::new(&connection).await?;

    loop {
        print_description();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Fail to read input");

        let trimmed_input = input.trim();

        match trimmed_input.to_lowercase().as_str() {
            "s" => {
                println!("Please enter the String to send");
                let mut send_input = String::new();
                io::stdin().read_line(&mut send_input)
                    .expect("Fail to read input");

                let reply = proxy.send_string(&send_input).await?;
                println!("reply: {reply}");
            }
            "d" => {
                let _data = Data {name: "Hyunsub".to_string(), value: 29};
                let serialized = serde_json::to_string(&_data).unwrap();
                println!("Send dault data struct with json string: {serialized}");

                let reply = proxy.send_data(&serialized).await?;
                println!("reply: {reply}");
            }
            "q" => {
                println!("Quit");
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }

    }
    Ok(())
}