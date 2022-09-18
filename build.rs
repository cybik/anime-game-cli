fn main() {
    cxx_build::bridge("src/cxxlib.rs")
        .flag_if_supported("-std=c++14")
        .compile("agcli-bridge");

    println!("cargo:rerun-if-changed=src/src/commands/mod.rs");
}