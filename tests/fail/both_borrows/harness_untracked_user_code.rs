//@revisions: stack tree
//@[tree]compile-flags: -Zmiri-tree-borrows
//@compile-flags: --test -Zmiri-disable-harness-borrow-tracking

//! Even with `-Zmiri-disable-harness-borrow-tracking`, memory allocated by a test
//! function is still borrow-tracked and aliasing violations in it are detected.

#[test]
fn aliasing_violation() {
    let mut x = 42i32;
    let raw = &mut x as *mut i32;
    let r = unsafe { &mut *raw };
    unsafe { *raw = 1 };
    let _val = *r;
    //~[stack]^ ERROR: /read access .* tag does not exist in the borrow stack/
    //~[tree]| ERROR: /read access through .* is forbidden/
}
