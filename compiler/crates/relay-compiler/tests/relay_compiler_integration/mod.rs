/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use common::ConsoleLogger;
use common::Diagnostic;
use fixture_tests::Fixture;
use futures_util::FutureExt;
use graphql_cli::DiagnosticPrinter;
use graphql_test_helpers::ProjectFixture;
use graphql_test_helpers::TestDir;
use relay_compiler::build_project::generate_extra_artifacts::default_generate_extra_artifacts_fn;
use relay_compiler::compiler::Compiler;
use relay_compiler::config::Config;
use relay_compiler::errors::BuildProjectError;
use relay_compiler::errors::Error;
use relay_compiler::source_for_location;
use relay_compiler::FileSourceKind;
use relay_compiler::FsSourceReader;
use relay_compiler::LocalPersister;
use relay_compiler::OperationPersister;
use relay_compiler::RemotePersister;
use relay_compiler::SourceReader;
use relay_config::PersistConfig;

pub async fn transform_fixture(fixture: &Fixture<'_>) -> Result<String, String> {
    let project_fixture = ProjectFixture::deserialize(fixture.content);

    let test_dir = TestDir::new();

    project_fixture.write_to_dir(test_dir.path());

    let original_cwd = std::env::current_dir().expect("Could not get cwd");

    std::env::set_current_dir(test_dir.path()).expect("Could not set cwd");

    let run_future = async {
        let mut config =
            Config::search(&PathBuf::from(test_dir.path())).expect("Could not load config");

        config.file_source_config = FileSourceKind::WalkDir;
        config.create_operation_persister = Some(Box::new(|project_config| {
            project_config.persist.as_ref().map(
                |persist_config| -> Box<dyn OperationPersister + Send + Sync> {
                    match persist_config {
                        PersistConfig::Remote(remote_config) => {
                            Box::new(RemotePersister::new(remote_config.clone()))
                        }
                        PersistConfig::Local(local_config) => {
                            Box::new(LocalPersister::new(local_config.clone(), false))
                        }
                    }
                },
            )
        }));
        config.generate_extra_artifacts = Some(Box::new(default_generate_extra_artifacts_fn));

        let compiler = Compiler::new(Arc::new(config), Arc::new(ConsoleLogger));
        let compiler_result = compiler.compile().await;

        match compiler_result {
            Ok(_) => {
                let mut output = ProjectFixture::read_from_dir(test_dir.path());
                // Omit the input files from the output
                output.remove_files(project_fixture);
                output
                    .serialize()
                    // Jump through a few hoops to avoid having at-generated in either this
                    // file or our generated `.expected` files, since that would confuse other
                    // tools.
                    .replace(&format!("{}generated", '@'), "<auto-generated>")
            }
            Err(compiler_error) => print_compiler_error(test_dir.path(), compiler_error),
        }
    };

    let result = match std::panic::AssertUnwindSafe(run_future)
        .catch_unwind()
        .await
    {
        Err(panic_err) => {
            std::env::set_current_dir(original_cwd)
                .expect("Could set cwd (while handling panic from test)");
            std::panic::resume_unwind(panic_err)
        }
        Ok(ok) => Ok(ok),
    };

    std::env::set_current_dir(original_cwd).expect("Could set cwd");

    result
}

fn print_compiler_error(root_dir: &Path, error: Error) -> String {
    let mut error_printer = CompilerErrorPrinter::for_root_dir(root_dir);
    error_printer.print_error(error);
    error_printer.chunks.join("\n")
}

struct CompilerErrorPrinter<'a> {
    chunks: Vec<String>,
    root_dir: &'a Path,
    source_reader: Box<dyn SourceReader + Send + Sync>,
}

impl<'a> CompilerErrorPrinter<'a> {
    fn for_root_dir(root_dir: &'a Path) -> Self {
        Self {
            chunks: vec![],
            root_dir,
            source_reader: Box::new(FsSourceReader {}),
        }
    }

    fn print_error(&mut self, compiler_error: Error) {
        match compiler_error {
            Error::DiagnosticsError { errors } => {
                for diagnostic in errors {
                    self.append_diagnostic(diagnostic)
                }
            }
            Error::BuildProjectsErrors { errors } => {
                for err in errors {
                    self.print_build_error(err);
                }
            }
            err => self.chunks.push(format!("{}", err)),
        }
    }

    fn print_build_error(&mut self, build_error: BuildProjectError) {
        match build_error {
            BuildProjectError::ValidationErrors {
                errors,
                project_name: _,
            } => {
                for diagnostic in errors {
                    self.append_diagnostic(diagnostic)
                }
            }
            e => self.chunks.push(format!("{}", e)),
        }
    }

    fn append_diagnostic(&mut self, diagnostic: Diagnostic) {
        let printer = DiagnosticPrinter::new(|source_location| {
            source_for_location(self.root_dir, source_location, self.source_reader.as_ref())
                .map(|source| source.to_text_source())
        });
        self.chunks.push(printer.diagnostic_to_string(&diagnostic))
    }
}
