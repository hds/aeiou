use tracing::debug;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    use tracing_subscriber::prelude::*;
    let layer = ari_subscriber::Layer::new();
    tracing_subscriber::registry().with(layer).init();


    spawn_named("tokio-oneshot", tokio_oneshot()).await.unwrap();
}

async fn tokio_oneshot() {
    debug!("creating oneshot");
    let (tx, rx) = tokio::sync::oneshot::channel::<u64>();
    debug!("oneshot created");

    let jh = spawn_named("receiver", async move {
        debug!("awaiting message");
        let msg = rx.await.unwrap();
        debug!(msg, "message received");
    });

    spawn_named("sender", async move {
        debug!("sending message");
        tx.send(5).unwrap();
        debug!("message sent");
    });

    debug!(?jh, "awaiting receiver task");
    jh.await.unwrap();
    debug!("good-bye");
}

use std::future::Future;

#[track_caller]
fn spawn_named<Fut>(name: &str, f: Fut) -> tokio::task::JoinHandle<<Fut as Future>::Output>
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    tokio::task::Builder::new()
        .name(name)
        .spawn(f)
        .unwrap_or_else(|_| panic!("spawning task '{name}' failed"))
}


