use std::io;
use mini_casher::client::Client;
use mini_casher::core::command::{Command};
use mini_casher::core::error::CashError;
use mini_casher::core::frames::Frame;


const CMD_MESSAGE: &str =
    "Enter the command:\r\n\
    - check the connection - `ping`\r\n\
    - get value by key - `get key`\r\n\
    - set a new value - `set key value`\r\n\
    - map length - `len`\r\n\
    - load all entity - `all`\r\n\
    - delete by key - `delete key`
    ";


#[tokio::main]
async fn main() {
    println!("{}", CMD_MESSAGE);
    let mut input = String::new();

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match Command::from_cmd(input.clone()) {
            Ok(command) => {
                let mut client = Client::connect("127.0.0.1:6379").await;

                match execute(command, &mut client).await {
                    Ok(frame) => println!("app-server response: {:?}", frame),
                    Err(e) => println!("failed: {:?}", e)
                }

            },
            Err(e) => println!("{:?}", e)
        }

        input.clear();
    }
}

async fn execute(cmd: Command, client: &mut Client) -> Result<Frame, CashError> {
    match cmd {
        Command::Get(cmd) => client.get(cmd.key()).await,
        Command::Set(cmd) => client.set(cmd.key(), cmd.value().clone()).await,
        Command::Delete(cmd) => client.delete(cmd.key()).await,
        Command::All => client.all().await,
        Command::Len => client.len().await,
        Command::Ping => client.ping().await
    }
}

