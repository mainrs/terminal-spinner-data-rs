use anyhow::Result;
use heck::ShoutySnakeCase;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

// Deserialization type.
#[derive(Debug, Deserialize)]
struct JsonSpinnerData {
    pub frames: Vec<String>,
    pub interval: u64,
}

// Deserialization type.
type Spinners = HashMap<String, JsonSpinnerData>;

fn spinner_name_to_const_name(name: impl AsRef<str>) -> String {
    name.as_ref().to_shouty_snake_case()
}

fn main() -> Result<()> {
    // Read in spinner data.
    let file_content = fs::read_to_string("./spinners.json")?;
    let spinners: Spinners = serde_json::from_str(&file_content)?;

    // Build the constants that hold each spinner data.
    let spinner_const_items = spinners
        .iter()
        .map(|(name, data)| {
            let name = format_ident!("{}", spinner_name_to_const_name(name));
            let frames = &data.frames;
            let interval = data.interval;

            quote! {
                pub const #name: crate::SpinnerData<'static> = crate::SpinnerData {
                    frames: &[
                        #(#frames),*
                    ],
                    interval: #interval,
                };
            }
        })
        .collect::<Vec<_>>();

    let module_to_write = quote! {
        #(#spinner_const_items)*
    };

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("spinners.rs");
    fs::write(&dest_path, module_to_write.to_string())?;

    // Only re-run if the actual spinner data has changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=spinners.json");
    Ok(())
}
