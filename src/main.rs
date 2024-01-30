use ashpd::desktop::Color;
use ashpd::desktop::global_shortcuts::GlobalShortcuts;

async fn run() -> ashpd::Result<()> {
    let gs = GlobalShortcuts::new()
        .await?;
    println!("{:?}", gs);
    let session = gs.create_session().await;
    println!("{:?}", session);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    run().await.unwrap();
}
