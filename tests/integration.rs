use std::process::Output;
use std::time::Duration;

use support::{fixture_workspace, run, run_with_deadline};

mod support {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output, Stdio};
    use std::time::{Duration, Instant};
    use tempfile::{TempDir, tempdir};

    pub fn fixture_source_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
    }

    fn copy_dir_recursively(source: &Path, target: &Path) {
        fs::create_dir_all(target).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let dest = target.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_dir_recursively(&entry.path(), &dest);
            } else {
                fs::copy(entry.path(), dest).unwrap();
            }
        }
    }

    pub fn fixture_workspace() -> TempDir {
        let source = fixture_source_dir();
        let workspace = tempdir().unwrap();

        fs::copy(source.join("info.json"), workspace.path().join("info.json")).unwrap();
        copy_dir_recursively(
            &source.join("exercises"),
            &workspace.path().join("exercises"),
        );

        workspace
    }

    fn lualings_command(dir: &Path) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_lualings"));
        command.current_dir(dir);
        command
    }

    pub fn run(dir: &Path, args: &[&str]) -> Output {
        lualings_command(dir).args(args).output().unwrap()
    }

    pub fn run_with_deadline(dir: &Path, args: &[&str], deadline: Duration) -> Output {
        let mut child = lualings_command(dir)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let start = Instant::now();
        loop {
            if child.try_wait().unwrap().is_some() {
                break;
            }
            if start.elapsed() > deadline {
                child.kill().unwrap();
                panic!(
                    "internal timeout protection did not stop the script within the \
                    external test deadline of {deadline:?} - the test suite would \
                    otherwise have hung"
                );
            }
            std::thread::sleep(Duration::from_millis(50));
        }

        child.wait_with_output().unwrap()
    }
}

fn stdout_of(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}

#[test]
fn list_shows_fixture_exercises_in_declared_order() {
    let workspace = fixture_workspace();
    let output = run(workspace.path(), &["list"]);
    let stdout = stdout_of(&output);

    assert!(
        output.status.success(),
        "expected `list` to succeed, got: {output:?}"
    );

    let passes_pos = stdout
        .find("passes")
        .expect("expected 'passes' in list output");
    let fails_pos = stdout
        .find("fails")
        .expect("expected 'fails' in list output");
    let infinite_loop_pos = stdout
        .find("infinite_loop")
        .expect("expected 'infinite_loop' in list output");

    assert!(
        passes_pos < fails_pos && fails_pos < infinite_loop_pos,
        "expected declared info.json order (passes, fails, infinite_loop), got: {stdout}"
    );
}

#[test]
fn run_on_passing_exercise_reports_success() {
    let workspace = fixture_workspace();
    let output = run(workspace.path(), &["run", "passes"]);
    let stdout = stdout_of(&output);

    assert!(
        output.status.success(),
        "expected exit success, got: {output:?}"
    );
    assert!(
        stdout.contains("[PASS]"),
        "expected a pass marker, got: {stdout}"
    );
}

#[test]
fn run_on_failing_exercise_reports_failure_with_message() {
    let workspace = fixture_workspace();
    let output = run(workspace.path(), &["run", "fails"]);
    let stdout = stdout_of(&output);

    assert_eq!(
        output.status.code(),
        Some(lualings::cli::EXIT_CONTENT_FAILURE),
        "expected the content-failure exit code, got: {output:?}"
    );

    assert!(
        stdout.contains("[FAIL]"),
        "expected a fail marker, got: {stdout}"
    );
    assert!(
        stdout.contains("boom"),
        "expected the real error message, got: {stdout}"
    );
}

#[test]
fn run_on_nonexistent_exercise_reports_clean_error() {
    let workspace = fixture_workspace();
    let output = run(workspace.path(), &["run", "does_not_exist"]);
    let stderr = stderr_of(&output);

    assert_eq!(
        output.status.code(),
        Some(lualings::cli::EXIT_OPERATIONAL_ERROR),
        "expected the operational-error exit code, got: {output:?}"
    );
    assert!(
        stderr.contains("does_not_exist"),
        "expected the missing name in the error message, got: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panic") && !stderr.to_lowercase().contains("unwrap"),
        "expected a clean error message, not a raw Rust panic/trace, got: {stderr}"
    );
}

#[test]
fn run_on_infinite_lopp_reports_timeout_without_hanging() {
    let workspace = fixture_workspace();
    let deadline = lualings::lua_runner::DEFAULT_TIMEOUT_BUDGET + Duration::from_secs(5);

    let output = run_with_deadline(workspace.path(), &["run", "infinite_loop"], deadline);
    let stdout = stdout_of(&output);

    assert!(
        stdout.contains("[TIMEOUT]"),
        "expected an explicit timeout marker, got: {stdout}"
    );
    assert!(
        !stdout.contains("[FAIL]"),
        "a timeout must be reported distinctly from a plain failure, got: {stdout}"
    );
}

#[test]
fn progress_persists_across_separate_process_invocations() {
    let workspace = fixture_workspace();

    let first_run = run(workspace.path(), &["run", "passes"]);
    assert!(
        first_run.status.success(),
        "expected the first invocation to pass, got: {first_run:?}"
    );

    let second_run = run(workspace.path(), &["list"]);
    let stdout = stdout_of(&second_run);

    assert!(
        stdout.contains("[x] passes"),
        "expected 'passes' to be marked completed by a separate later invocation, got: {stdout}"
    );
}

#[test]
fn init_run_init_does_not_clobber_existing_progress() {
    let workspace = tempfile::tempdir().unwrap();

    let init_once = run(workspace.path(), &["init"]);
    assert!(
        init_once.status.success(),
        "expected the first `init` to succeed, got: {init_once:?}"
    );

    let run_once = run(workspace.path(), &["run", "variables1"]);
    assert!(
        run_once.status.success(),
        "expected 'variables1' to pass as currently authored, got: {run_once:?}"
    );

    let init_twice = run(workspace.path(), &["init"]);
    assert!(
        init_twice.status.success(),
        "expected the second `init` to succeed, got: {init_twice:?}"
    );

    let list_after = run(workspace.path(), &["list"]);
    let stdout = stdout_of(&list_after);

    assert!(
        stdout.contains("[x] variables1"),
        "expected progress to survive a second `init`, got: {stdout}"
    );
}
