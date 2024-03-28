#![cfg(feature = "boxed")]

use bumpalo::boxed::Box;
use bumpalo::Bump;

#[test]
fn into_raw_aliasing() {
    let bump = Bump::new();
    let boxed = Box::new_in(1, &bump);
    let raw = Box::into_raw(boxed);

    let mut_ref = unsafe { &mut *raw };
    dbg!(mut_ref);
}

#[cfg(feature = "serde")]
#[test]
fn test_box_serializes() {
    let bump = Bump::new();
    let boxed = Box::new_in(1, &bump);
    assert_eq!(serde_json::to_string(&boxed).unwrap(), "1");
    let boxed_str = Box::new_in("a", &bump);
    assert_eq!(serde_json::to_string(&boxed_str).unwrap(), "\"a\"");
}
