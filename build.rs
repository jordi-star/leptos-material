//! Install Rollup.js to bundle all material-web components into one JS file we can include using our `UseMaterialWebComponents` component.
use std::{
    env,
    fs::{File, OpenOptions},
    io,
    process::Command,
};

const IMPORTS_JS_FILE_NAME: &str = "imports.js";
const OUTPUT_BUNDLE_FILE_NAME: &str = "output_bundle.js";
fn main() {
    // Only re-run if new features added.
    println!("cargo::rerun-if-changed=Cargo.toml,build.rs");
    let imports_file_path = format!("{}/{}", env::var("OUT_DIR").unwrap(), IMPORTS_JS_FILE_NAME);
    println!("cargo::warning={:?}", imports_file_path);
    let mut imports_file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&imports_file_path)
        .unwrap_or_else(|_| panic!("Unable to open {} file for writing",
            imports_file_path));
    let output_path = format!(
        "{}/{}",
        env::var("OUT_DIR").unwrap(),
        OUTPUT_BUNDLE_FILE_NAME
    );

    let npm_is_installed = run_command("npm -v").unwrap();
    assert!(npm_is_installed.success());
    let _ =
        run_command("npm install rollup @rollup/plugin-node-resolve --global").unwrap();
    let install_mwc = run_command("npm install @material/web --save-dev").unwrap();
    assert!(install_mwc.success());

    // Import typography stylesheet
    add_import("typography/md-typescale-styles", &mut imports_file);
    // Import components
    if is_feature_enabled("checkbox") {
        add_import("checkbox/checkbox", &mut imports_file);
    }
    if is_feature_enabled("textfield") {
        add_import("textfield/filled-text-field", &mut imports_file);
        add_import("textfield/outlined-text-field", &mut imports_file);
    }
    if is_feature_enabled("icon") {
        add_import("icon/icon", &mut imports_file);
    }
    if is_feature_enabled("button") {
        add_import("button/outlined-button", &mut imports_file);
        add_import("button/elevated-button", &mut imports_file);
        add_import("button/filled-button", &mut imports_file);
        add_import("button/filled-tonal-button", &mut imports_file);
        add_import("button/text-button", &mut imports_file);
    }
    if is_feature_enabled("iconbutton") {
        add_import("iconbutton/icon-button", &mut imports_file);
    }
    if is_feature_enabled("elevation") {
        add_import("elevation/elevation", &mut imports_file);
    }
    if is_feature_enabled("progress") {
        add_import("progress/circular-progress", &mut imports_file);
        add_import("progress/linear-progress", &mut imports_file);
    }
    if is_feature_enabled("select") {
        add_import("select/filled-select", &mut imports_file);
        add_import("select/outlined-select", &mut imports_file);
        add_import("select/select-option", &mut imports_file);
    }
    if is_feature_enabled("chips") {
        add_import("chips/assist-chip", &mut imports_file);
        add_import("chips/filter-chip", &mut imports_file);
        add_import("chips/suggestion-chip", &mut imports_file);
        add_import("chips/input-chip", &mut imports_file);
    }
    add_typescale_styles(&mut imports_file);
    assert!(imports_file.sync_all().is_ok());
    // Node modules are installed in the CARGO_MANIFEST_DIR instead of the OUT_DIR to prevent completely re-installing @material/web
    // each time the build script is run.
    let run_rollup = run_command(&format!(
        r#"npx rollup -p 'node-resolve={{modulePaths: ["{}/node_modules/"]}}' {} -o {} --format iife"#,
        env::var("CARGO_MANIFEST_DIR").unwrap(), imports_file_path, output_path
    ))
    .unwrap();
    assert!(run_rollup.success());
}

pub fn run_command(command: &str) -> Result<std::process::ExitStatus, io::Error> {
    let terminal = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        "sh"
    };
    let terminal_command_arg = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };
    Command::new(terminal)
        .arg(terminal_command_arg)
        .arg(command)
        .spawn()
        .unwrap()
        .wait()
}

fn is_feature_enabled(feature: &str) -> bool {
    let feature = String::from(feature).to_uppercase().replace('-', "_");
    env::var(format!("CARGO_FEATURE_{}", feature)).is_ok()
}

fn add_import<T: std::io::Write>(to_import: &str, file: &mut T) {
    writeln!(file, "import '@material/web/{}.js';", to_import).unwrap_or_else(|_| panic!("Error adding import '{}' to imports.js",
        to_import));
}

fn add_typescale_styles<T: std::io::Write>(file: &mut T) {
    let _ = writeln!(
        file,
        "import {{styles as typescaleStyles}} from '@material/web/typography/md-typescale-styles.js';\ndocument.adoptedStyleSheets.push(typescaleStyles.styleSheet)",
    );
}
