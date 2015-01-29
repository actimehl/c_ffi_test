#[allow(unstable)]
extern crate libc;
use libc::c_char;

extern {
    fn fn_pointer(data: *mut c_char);
    fn fn_fixedarray(data: *mut [c_char; 10]);
}

fn main() {
    // fixed size array right size
    let mut data_ok: [c_char; 10] = [65, 66, 67, 68, 69, 70, 71, 72, 73, 74];
    unsafe {
        fn_pointer(data_ok.as_mut_ptr()); // ok
        fn_fixedarray(&mut data_ok); // ok
    }

    // fixed size array to long
    let mut data_tolong: [c_char; 11] = [75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85];
    unsafe {
        fn_pointer(data_tolong.as_mut_ptr()); // ok
        // fn_fixedarray(&mut data_tolong); // error but should be ok if it could be casted correctly
    }

    // fixed size array to short
    let mut data_toshort: [c_char; 5] = [86, 87, 88, 89, 90];
    unsafe {
        fn_pointer(data_toshort.as_mut_ptr()); // kaboom, memory corruption!
        // fn_fixedarray(&mut data_toshort); // error and no safe cast should fix it
    } 
}
