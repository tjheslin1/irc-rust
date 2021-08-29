use irc_rust::threadpool::PoolCreationError;
use irc_rust::threadpool::ThreadPool;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn test_threadpool_err_on_zero_sized_pool() {
    let empty_thread_pool = ThreadPool::new(0)
        .expect_err("Expected `Err` but got `Ok` when creating an empty ThreadPool.");

    let expected_err = PoolCreationError {
        message: "Error creating ThreadPool, cannot create a pool with zero threads!".to_string(),
    };

    assert_eq!(empty_thread_pool, expected_err);
}

#[test]
fn test_threadpool_creates_desired_number_of_workers() {
    let thread_pool = match ThreadPool::new(5) {
        Ok(pool) => pool,
        Err(e) => panic!("Expected `ThreadPool` but got `Err`: {}", e),
    };

    assert_eq!(thread_pool.workers.len(), 5);
}

#[test]
fn test_threadpool_runs_job() {
    let thread_pool = match ThreadPool::new(1) {
        Ok(pool) => pool,
        Err(e) => panic!("Expected `ThreadPool` but got `Err`: {}", e),
    };

    let (tx, rx) = mpsc::channel();
    let f = move || tx.send(()).unwrap();

    thread_pool.execute(f);

    assert_eq!(rx.recv_timeout(Duration::from_secs(1)), Ok(()));
}
