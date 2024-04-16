fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    lalrpop::Configuration::new().set_in_dir(".").set_out_dir(out_dir).process().unwrap();
}