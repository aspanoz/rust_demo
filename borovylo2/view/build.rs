fn main() {
   slint_build::compile("ui/main.slint").unwrap();
   slint_build::print_rustc_flags().unwrap();
   std::fs::copy("../src/vars.rs", "src/vars.rs").unwrap();
}
