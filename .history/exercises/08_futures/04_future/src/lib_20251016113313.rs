//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
        {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    } // non_send bị drop ở đây — không còn tồn tại

    yield_now().await; // an toàn vì không giữ Rc nữa
}
