pub mod storage;

use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let db = rt.block_on(storage::new()).unwrap();
    drop(rt);
    db.close();
}
