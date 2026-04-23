use {regex::Regex, std::process::Command};

#[test]
fn single_dependency() {
  let output = Command::new(env!("CARGO_BIN_EXE_cargo-path"))
    .args(["path", "serde"])
    .output()
    .unwrap();

  assert!(output.status.success());

  let stdout = str::from_utf8(&output.stdout).unwrap();

  let regex =
    Regex::new(r"^.*/\.cargo/registry/src/index\.crates\.io-[0-9a-f]*/serde-1\.0\.228\n$").unwrap();

  assert!(
    regex.is_match(stdout),
    "regex mismatch: {stdout} ~= {regex}",
  );
}
