use futures::{FutureExt, TryFutureExt};

async fn simple_sum(x: Vec<i32>) -> i32 {
    println!("Simple_sum is running!");
    x.into_iter().sum()
}

async fn vec_sum(x: Vec<i32>) -> Result<i32, u32> {
    if x.is_empty() {
        Err(1)
    } else {
        Ok(x.into_iter().fold(0, |a, i| a + i))
    }
}

async fn create_vec(start: i32, num: u32) -> Result<Vec<i32>, u32> {
    if num == 0 {
        Err(0)
    } else {
        let mut v = vec![];
        for i in 0..num {
            v.push(start + i as i32);
        }
        Ok(v)
    }
}

#[tokio::main]
async fn main() {
    let simple_answer = simple_sum(vec![1, 2, 3]);
    // simple_answer is now a lazy Future, and needs to be polled to completion.
    // Fortunately, .await does this automatically: it polls the future and waits for its
    // completion.
    println!("Simple sum hasn't run yet!  Let's try now:");
    assert_eq!(simple_answer.await, 6);

    // This works if you want to just await the completion, but what if you want to combine
    // and map the answer?  Simple, use future combinators.  There are ones for simple futures:
    let combo = simple_sum(vec![4, 5, 6]);
    let combo2 = combo.map(|sum| sum * 2);
    //However, you have to await to pull out the answer:

    println!("Simple sum combo hasn't run yet!  Let's try now:");
    assert_eq!(combo2.await, 30);

    // There are also combinations for Futures of Results that especially pull out the Ok
    // and Err values in the future:
    let answer = create_vec(1, 3)
        .and_then(vec_sum)
        .map_ok(|sum| sum * 2)
        .or_else(|_| async { Err(-1_i32) });

    assert_eq!(answer.await.unwrap(), 12);

    let answer = create_vec(0, 0)
        .and_then(vec_sum)
        .map_ok(|sum| sum * 2)
        .or_else(|_| async { Err(-1_i32) });

    assert_eq!(answer.await.unwrap_err(), -1);
}
