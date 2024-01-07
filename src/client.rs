use zbus::{Connection, Result, dbus_proxy};
use std::io;
#[dbus_proxy(
    interface = "org.zbus.DataIPC",
    default_service = "org.zbus.DataIPC",
    default_path = "/org/zbus/DataIPC"
)]
trait DataStruct {
    async fn send_string(&self, name: &str) -> Result<String>;
}

fn print_description() {
    println!("-----------------------------");
    println!("s: send string");
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