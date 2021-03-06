extern crate gameserver_sandbox_rs as sandbox;

use sandbox::data::repositories::Repository;

use sandbox::app::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let repo = Repository::new().unwrap();
    server::start_server(repo).await
}
