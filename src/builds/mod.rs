mod delegate;
pub use delegate::generate_delegate_build_string;

mod delegate_impl;
pub use delegate_impl::generate_delegate_impl_build_string;

#[macro_export]
macro_rules! build_delegate {
    () => {
        fn main() {
            println!("cargo:rerun-if-changed=./build.rs");
            println!("cargo:rerun-if-changed=./INPUT");

            let out_dir = ::std::env::var_os("OUT_DIR").expect("Could not read en var `OUT_DIR`");
            let out_path = ::std::path::Path::new(&out_dir);

            let input = $crate::builds::generate_delegate_build_string("./INPUT");
            ::std::fs::write(out_path.join("lib.rs"), input).expect("Could not write expanded macro.");
        }
    };
}

#[macro_export]
macro_rules! build_delegate_impl {
    () => {
        fn main() {
            println!("cargo:rerun-if-changed=./build.rs");
            println!("cargo:rerun-if-changed=../INPUT");

            let out_dir = ::std::env::var_os("OUT_DIR").expect("Could not read en var `OUT_DIR`");
            let out_path = ::std::path::Path::new(&out_dir);

            let input = $crate::builds::generate_delegate_impl_build_string("../INPUT");
            ::std::fs::write(out_path.join("lib.rs"), input).expect("Could not write expanded macro.");
        }
    };
}
