use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// unsafe !impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY : we know no-one else is concurrently mutating self.value, because (!Sync)
        // SAFETY : we know we're not invalidating any references, because we never give any out.
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY : we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn bad1() {
        use std::sync::Arc;

        for _ in 0..10000 {
            let x: Arc<Cell<i32>> = Arc::new(Cell::new(42));

            let x1: Arc<Cell<i32>> = Arc::clone(&x);
            let t1 = std::thread::spawn(move || {
                x1.set(43);
            });

            let x2: Arc<Cell<i32>> = Arc::clone(&x);
            let t2 = std::thread::spawn(move || {
                x2.set(44);
            });
            t1.join().unwrap();
            t2.join().unwrap();

            assert_eq!(x.get(), 44);
        }
    }
}
