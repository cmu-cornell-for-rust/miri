//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
//@compile-flags: --test -Zmiri-disable-harness-borrow-tracking

//! Check that the test harness (whose allocations are not borrow-tracked with
//! `-Zmiri-disable-harness-borrow-tracking`) runs normally. The harness code constantly
//! retags pointers into its own untracked allocations, so this exercises the untracked
//! code paths of the aliasing model.

#[test]
fn do_some_allocations() {
    let mut v: Vec<String> = Vec::new();
    for i in 0..10 {
        v.push(i.to_string());
    }
    v.remove(0);
    assert_eq!(v.len(), 9);
}

#[test]
fn aliasing_in_user_code_works() {
    let mut x = 42i32;
    let raw = &mut x as *mut i32;
    let r = unsafe { &mut *raw };
    *r = 1;
    // `raw`'s tag is still valid, so we may use it again after `r` is dead.
    unsafe { *raw = 2 };
    assert_eq!(x, 2);
}
