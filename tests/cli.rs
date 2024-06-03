#[cfg(test)]
mod happy_path {
    use assert_cmd::Command;
    use predicates::prelude::predicate;

    /// Test that the CLI can render a single-flavor template file
    #[test]
    fn test_single() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd
            .args(["tests/fixtures/single/single.tera", "-f", "latte"])
            .assert();
        assert
            .success()
            .stdout(include_str!("fixtures/single/single.md"));
    }

    /// Test that the CLI can render a multi-flavor template file
    #[test]
    fn test_multi() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd.args(["tests/fixtures/multi/multi.tera"]).assert();
        assert
            .success()
            .stdout(include_str!("fixtures/multi/multi.md"));
    }

    /// Test that the CLI can render a multi-flavor matrix template
    #[test]
    fn test_multifile_render() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd
            .args(["--dry-run", "tests/fixtures/multifile.tera"])
            .assert();
        assert.success().stdout(predicate::str::contains(
            "catppuccin-macchiato-yellow-no-italics.ini",
        ));
    }

    /// Test that the CLI can render a template which uses `read_file`
    #[test]
    fn test_read_file() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd
            .args(["tests/fixtures/read_file/read_file.tera", "-f", "latte"])
            .assert();
        assert
            .success()
            .stdout(include_str!("fixtures/read_file/read_file.md"));
    }

    /// Test that the CLI can render a UTF-8 template file
    #[test]
    fn test_utf8() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd.args(["tests/fixtures/encodings/utf8.tera"]).assert();
        assert
            .success()
            .stdout(predicate::str::contains("it worked!"));
    }

    /// Test that the CLI can render a UTF-8 with BOM template file
    #[test]
    fn test_utf8_bom() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd.args(["tests/fixtures/encodings/utf8bom.tera"]).assert();
        assert
            .success()
            .stdout(predicate::str::contains("it worked!"));
    }

    /// Test that the CLI can render a UTF-16 BE template file
    #[test]
    fn test_utf16be() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd.args(["tests/fixtures/encodings/utf16be.tera"]).assert();
        assert
            .success()
            .stdout(predicate::str::contains("it worked!"));
    }

    /// Test that the CLI can render a UTF-16 LE template file
    #[test]
    fn test_utf16le() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        let assert = cmd.args(["tests/fixtures/encodings/utf16le.tera"]).assert();
        assert
            .success()
            .stdout(predicate::str::contains("it worked!"));
    }
}

#[cfg(test)]
mod sad_path {
    use assert_cmd::Command;
    use predicates::prelude::predicate;

    #[test]
    fn nonexistent_template_file() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        cmd.arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Template file does not exist"));
    }

    #[test]
    fn invalid_flavor() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        cmd.arg("examples/demo/input.tera")
            .args(["--flavor", "invalid"]);
        cmd.assert().failure().stderr(predicate::str::contains(
            "error: invalid value 'invalid' for '--flavor <FLAVOR>'",
        ));
    }

    #[test]
    fn template_contains_invalid_syntax() {
        let mut cmd = Command::cargo_bin("whiskers").expect("binary exists");
        cmd.arg("examples/errors.tera").args(["-f", "mocha"]);
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Error: Template is invalid"));
    }
}
