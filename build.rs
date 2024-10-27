fn main() {
    // Re-run build if the locales directory changes to get the latest translations
    println!("cargo:rerun-if-changed=locales");
}
