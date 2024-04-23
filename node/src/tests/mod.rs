//! High level node tests, similar to spawning `tanssi-node --dev` and inspecting output logs.

use {
    crate::cli::Cli,
    sc_cli::{Runner, SubstrateCli},
    sc_service::TaskManager,
    std::time::Duration,
};

mod panics;

// Create a runner for tests
fn create_runner() -> Runner<Cli> {
    // TODO: set values that make sense for unit tests, probably --dev is enough
    let cli = Cli::from_iter(["--dev"]);
    let runner = cli.create_runner(&cli.run.normalize()).unwrap();

    runner
}

// Nice hack from polkadot-sdk to run a unit test in a separate process.
// We need to use this because create_runner sets up logging a new panic hook, and that is
// global and fails if it was already setup by a previous test.
// Improved from upstream by using the exact test name, and by never capturing the test output.
fn run_test_in_another_process(
    test_name: &str,
    test_body: impl FnOnce(),
) -> Option<std::process::Output> {
    run_test_in_another_process_expect_error(test_name, 0, test_body)
}

fn run_test_in_another_process_expect_error(
    test_name: &str,
    exit_code: i32,
    test_body: impl FnOnce(),
) -> Option<std::process::Output> {
    if std::env::var("RUN_FORKED_TEST").is_ok() {
        test_body();
        None
    } else {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .arg(test_name)
            .arg("--exact")
            .arg("--nocapture")
            .arg("--include-ignored")
            .env("RUN_FORKED_TEST", "1")
            .output()
            .unwrap();

        assert_eq!(output.status.code(), Some(exit_code));
        Some(output)
    }
}

/// Macro to get the name of the current function at runtime. Used to make calling
/// `run_test_in_another_process` less error-prone. Copied from `stdext`, but modified to remove
/// the binary name from the output.
// https://github.com/popzxc/stdext-rs/blob/dc03b4afa28b3a1d2451ca54ad252244f029099b/src/macros.rs#L63
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        let name = &name[..name.len() - 3];
        // Strip initial tanssi_node::
        let end_of_first_item = name.bytes().position(|x| x == b':').unwrap();
        // `2` is the length of the `::` after `tanssi_node`
        &name[end_of_first_item + 2..]
    }};
}

#[test]
fn function_name_works() {
    assert_eq!(function_name!(), "tests::function_name_works");
}

#[test]
fn run_test_in_another_process_works() {
    let parent_pid = std::process::id();
    let output = run_test_in_another_process(function_name!(), || {
        let child_pid = std::process::id();
        eprintln!("Child process running, PID: {}.", child_pid);
    });

    if output.is_none() {
        // Assert that the output is only None if we are the child process
        assert!(std::env::var("RUN_FORKED_TEST").is_ok());
    }

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    // TODO: instead of inspecting stderr, why not use proper IPC?
    // Parent can create a named pipe and pass it as env variable to child.
    assert!(stderr.contains("Child process running, PID: "));
    // Assert child process id is different from parent process id
    assert!(!stderr.contains(&format!("PID: {}.", parent_pid)));
}
