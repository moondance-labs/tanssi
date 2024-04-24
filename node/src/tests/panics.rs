// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.

//! Tests related to panics: which ones stop the node, which ones do not, which tasks are essential,
//! etc.

use {super::*, crate::function_name};

// This test is from sc_cli:
// https://github.com/paritytech/polkadot-sdk/blob/39b1f50f1c251def87c1625d68567ed252dc6272/substrate/client/cli/src/runner.rs#L363
/// This test ensures that `run_node_until_exit` aborts waiting for "stuck" tasks after 60
/// seconds, aka doesn't wait until they are finished (which may never happen).
#[test]
#[ignore = "takes 60 seconds to run"]
fn ensure_run_until_exit_is_not_blocking_indefinitely() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();
                let (sender, receiver) = futures::channel::oneshot::channel();

                // We need to use `spawn_blocking` here so that we get a dedicated thread
                // for our future. This future is more blocking code that will never end.
                task_manager
                    .spawn_handle()
                    .spawn_blocking("test", None, async move {
                        let _ = sender.send(());
                        loop {
                            std::thread::sleep(Duration::from_secs(30));
                        }
                    });

                task_manager
                    .spawn_essential_handle()
                    .spawn_blocking("test2", None, async {
                        // Let's stop this essential task directly when our other task
                        // started. It will signal that the task manager should end.
                        let _ = receiver.await;
                    });

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    assert!(stderr.contains("Task \"test\" was still running after waiting 60 seconds to finish."));
    assert!(
        !stderr.contains("Task \"test2\" was still running after waiting 60 seconds to finish.")
    );
}

#[test]
fn node_stops_if_blocking_task_panics() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                task_manager
                    .spawn_handle()
                    .spawn_blocking("test", None, async move {
                        panic!("spawn_blocking panicked");
                    });

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    assert!(stderr.contains("Thread 'tokio-runtime-worker' panicked at 'spawn_blocking panicked',"));
}

#[test]
fn node_stops_if_non_essential_task_panics() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                task_manager.spawn_handle().spawn("test", None, async move {
                    panic!("non-essential task panicked");
                });

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    assert!(
        stderr.contains("Thread 'tokio-runtime-worker' panicked at 'non-essential task panicked',")
    );
}

#[test]
fn node_stops_if_essential_task_panics() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                task_manager
                    .spawn_essential_handle()
                    .spawn("test", None, async move {
                        panic!("essential task panicked");
                    });

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    assert!(stderr.contains("Thread 'tokio-runtime-worker' panicked at 'essential task panicked',"));
}

#[test]
fn node_stops_if_essential_task_finishes() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                task_manager
                    .spawn_essential_handle()
                    .spawn("test", None, async move {
                        // Sleep for 2 seconds and return.
                        // An essential task that returns should cause the task manager to stop.
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    });

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    assert!(stderr.contains("Essential task `test` failed. Shutting down service."));
}

#[test]
fn node_stops_if_rust_thread_panics() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                std::thread::spawn(|| panic!("rust thread panicked"));

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap_err();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());
    assert!(stderr.contains("Thread '<unnamed>' panicked at 'rust thread panicked',"));
}

#[test]
#[ignore = "takes 10 seconds to run"]
fn node_does_not_stop_if_non_essential_task_finishes() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                task_manager
                    .spawn_handle()
                    .spawn("test1", None, async move {
                        // Sleep for 2 seconds and return.
                        // A non-essential task that returns should not cause the task manager to stop.
                        tokio::time::sleep(Duration::from_secs(2)).await;
                    });

                task_manager
                    .spawn_essential_handle()
                    .spawn("test2", None, async move {
                        // Sleep for 10 seconds and return.
                        // An essential task that returns should cause the task manager to stop.
                        // Therefore this node should stop after 10 seconds.
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    });

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());

    assert!(stderr.contains("Essential task `test2` failed. Shutting down service."));
    assert!(!stderr.contains("test1"));
}

#[test]
fn catch_unwind_example() {
    let output = run_test_in_another_process_expect_error(function_name!(), 1, || {
        let runner = create_runner();

        runner
            .run_node_until_exit(move |cfg| async move {
                let task_manager = TaskManager::new(cfg.tokio_handle.clone(), None).unwrap();

                // Because of the custom panic hook set by create_runner, using catch_unwind is
                // only possible in a single-threaded context after calling force_unwind.
                {
                    let _guard = sp_panic_handler::AbortGuard::force_unwind();

                    std::panic::catch_unwind(|| {
                        panic!("First panic did not stop the node");
                    })
                    .unwrap_err();
                }

                // We dropped the guard, the default behavior is to abort
                std::panic::catch_unwind(|| {
                    panic!("Second panic should not stop the node, but it does");
                })
                .unwrap_err();

                Ok::<_, sc_service::Error>(task_manager)
            })
            .unwrap();

        panic!("Third panic, unreachable");
    });

    let Some(output) = output else { return };

    let stderr = dbg!(String::from_utf8(output.stderr).unwrap());
    assert!(stderr.contains(" panicked at 'First panic did not stop the node',"));
    assert!(stderr.contains(" panicked at 'Second panic should not stop the node, but it does',"));
    assert!(!stderr.contains("Third panic, unreachable"));
    assert_eq!(stderr.matches(" panicked at ").count(), 2)
}
