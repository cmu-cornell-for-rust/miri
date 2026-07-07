//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
//@compile-flags: -Zmiri-disable-harness-borrow-tracking

//! Without a test harness, `-Zmiri-disable-harness-borrow-tracking` exempts allocations
//! made without any user-relevant frame on the stack (e.g. runtime startup, or threads
//! executing only non-local code) from borrow tracking; everything user code does is
//! still fully tracked.

fn main() {
    let mut v = vec![1, 2, 3];
    let r = &mut v[0];
    *r = 4;
    assert_eq!(v, [4, 2, 3]);
    // Also touch machinery that was allocated before `main` started.
    let args: Vec<String> = std::env::args().collect();
    assert!(!args.is_empty());

    // This Box is allocated by a non-local frame on a thread with no user-relevant frame
    // below it, so it is attributed to the runtime and *not* tracked: the aliasing
    // violation below goes undetected. This is exactly the unsoundness the flag opts into.
    let mut b: Box<i32> = std::thread::spawn(Box::<i32>::default).join().unwrap();
    let raw = &mut *b as *mut i32;
    let r = unsafe { &mut *raw };
    unsafe { *raw = 1 };
    let _val = *r;
}
