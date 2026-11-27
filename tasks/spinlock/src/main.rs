use spinlock::SpinLock;

fn main() {
    let counter = SpinLock::new(0);

    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..1_000 {
                    *counter.lock() += 1
                }
            });
        }
    });

    let guard = counter.lock();

    assert_eq!(*guard, 4_000);
}
