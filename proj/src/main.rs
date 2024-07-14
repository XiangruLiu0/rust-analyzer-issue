fn main() {
    println!("Hello, world!");
}

mod bindings {
    // rust-analyzer reports: failed to load file `UNRESOLVED_ENV_VAR`rust-analyzermacro-error
    include!(env!("BINDING_PATH"));
}
