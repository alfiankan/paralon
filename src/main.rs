use std::env;

mod manager;
mod api;
mod sourcers;
mod trigger;
mod puller;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    if args[1] == String::from("pull-down") {

        match env::var("MINIO_ENDPOINT") {
            Ok(_) => {
                println!("{}", "Pulling down from s3");
                puller::minio::pull_down_data_minio().await.expect("TODO: panic message");
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

    }

    if args[1] == String::from("pull-up") {
        println!("{}", "Pulling up from s3")
    }

    if args[1] == String::from("run-pipe") {
        println!("{}", "Running PIPE")
    }
}
