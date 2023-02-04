use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=winntdef.h");
    println!("cargo:rerun-if-changed=build.rs");

    let bindings = bindgen::Builder::default()
        .header("winntdef.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .size_t_is_usize(true)
        .impl_debug(true)
        .generate()?;

    let out_path = PathBuf::from("src");
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}