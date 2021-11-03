fn main() {
  println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.13");
  println!("cargo:rerun-if-changed=migrations");
  tauri_build::build();
}
