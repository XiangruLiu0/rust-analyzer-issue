mod bindings {
    // rust-analyzer reports: failed to load file `UNRESOLVED_ENV_VAR`rust-analyzermacro-error
    include!(env!("BINDINGS"));
}

#[no_mangle]
pub extern "C" fn main() {
    let a = 10;
    let b = 20;
    let sum = unsafe { bindings::add(a, b) };
    println!("Sum is: {}", sum);
}
