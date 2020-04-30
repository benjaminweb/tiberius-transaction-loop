extern crate tiberius;
extern crate tokio;
use futures::future::FutureResult;
use futures::stream::Stream;
use futures::Future;
use tiberius::SqlConnection;
use tokio::runtime::current_thread;

// https://github.com/steffengy/tiberius/issues/49#issuecomment-377310318
// https://github.com/steffengy/tiberius/issues/51#issuecomment-380084937

fn folder(
    t: tiberius::Transaction<std::boxed::Box<dyn tiberius::BoxableIo>>,
) -> impl Future<
    Item = tiberius::Transaction<std::boxed::Box<dyn tiberius::BoxableIo>>,
    Error = tiberius::Error,
> {
    futures::stream::iter_ok(vec![1, 2, 3]).fold(t, |tx, v| {
        tx.exec("SQL QUERY", &[&v]).and_then(|(_, tx)| Ok(tx))
    })
}

fn exec_query(conn_str: &str) {
    let future = SqlConnection::connect(conn_str)
        .and_then(|conn| conn.transaction())
        .and_then(|t| folder(t))
        .and_then(|trans| trans.commit())
        .map_err(|err| panic!(err));
    current_thread::block_on_all(future).unwrap();
}

fn main() {
    println!("Hello, world!");
}
