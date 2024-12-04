use crate::common::{uv_snapshot, TestContext};
use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_fs::prelude::*;
use fs_err::File;
use indoc::indoc;
use insta::assert_snapshot;
use predicates::prelude::predicate;
use std::env::current_dir;
use zip::ZipArchive;

#[test]
fn build() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    // Build the specified path.
    uv_snapshot!(&filters, context.build().arg("project"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built project/dist/project-0.1.0.tar.gz
    Successfully built project/dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    fs_err::remove_dir_all(project.child("dist"))?;

    // Build the current working directory.
    uv_snapshot!(&filters, context.build().current_dir(project.path()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/project-0.1.0.tar.gz
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    fs_err::remove_dir_all(project.child("dist"))?;

    // Error if there's nothing to build.
    uv_snapshot!(&filters, context.build(), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
      × Failed to build `[TEMP_DIR]/`
      ╰─▶ [TEMP_DIR]/ does not appear to be a Python project, as neither `pyproject.toml` nor `setup.py` are present in the directory
    "###);

    // Build to a specified path.
    uv_snapshot!(&filters, context.build().arg("--out-dir").arg("out").current_dir(project.path()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/out/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built out/project-0.1.0.tar.gz
    Successfully built out/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("out")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("out")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

#[test]
fn sdist() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    // Build the specified path.
    uv_snapshot!(&filters, context.build().arg("--sdist").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Successfully built dist/project-0.1.0.tar.gz
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    Ok(())
}

#[test]
fn wheel() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    // Build the specified path.
    uv_snapshot!(&filters, context.build().arg("--wheel").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building wheel...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

#[test]
fn sdist_wheel() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    // Build the specified path.
    uv_snapshot!(&filters, context.build().arg("--sdist").arg("--wheel").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Building wheel...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/project-0.1.0.tar.gz
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

#[test]
fn wheel_from_sdist() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    // Build the sdist.
    uv_snapshot!(&filters, context.build().arg("--sdist").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Successfully built dist/project-0.1.0.tar.gz
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    // Error if `--wheel` is not specified.
    uv_snapshot!(&filters, context.build().arg("./dist/project-0.1.0.tar.gz").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
      × Failed to build `[TEMP_DIR]/project/dist/project-0.1.0.tar.gz`
      ╰─▶ Pass `--wheel` explicitly to build a wheel from a source distribution
    "###);

    // Error if `--sdist` is specified.
    uv_snapshot!(&filters, context.build().arg("./dist/project-0.1.0.tar.gz").arg("--sdist").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
      × Failed to build `[TEMP_DIR]/project/dist/project-0.1.0.tar.gz`
      ╰─▶ Building an `--sdist` from a source distribution is not supported
    "###);

    // Build the wheel from the sdist.
    uv_snapshot!(&filters, context.build().arg("./dist/project-0.1.0.tar.gz").arg("--wheel").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building wheel from source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    // Passing a wheel is an error.
    uv_snapshot!(&filters, context.build().arg("./dist/project-0.1.0-py3-none-any.whl").arg("--wheel").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
      × Failed to build `[TEMP_DIR]/project/dist/project-0.1.0-py3-none-any.whl`
      ╰─▶ `dist/project-0.1.0-py3-none-any.whl` is not a valid build source. Expected to receive a source directory, or a source distribution ending in one of: `.tar.gz`, `.zip`, `.tar.bz2`, `.tar.lz`, `.tar.lzma`, `.tar.xz`, `.tar.zst`, `.tar`, `.tbz`, `.tgz`, `.tlz`, or `.txz`.
    "###);

    Ok(())
}

#[test]
fn fail() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    project.child("setup.py").write_str(
        r#"
        from setuptools import setup

        setup(
            name="project",
            version="0.1.0",
            packages=["project"],
            install_requires=["foo==3.7.0"],
        )
        "#,
    )?;

    // Build the specified path.
    uv_snapshot!(&filters, context.build().arg("project"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    Traceback (most recent call last):
      File "<string>", line 14, in <module>
      File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 328, in get_requires_for_build_sdist
        return self._get_build_requires(config_settings, requirements=[])
               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
      File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 295, in _get_build_requires
        self.run_setup()
      File "[CACHE_DIR]/builds-v0/[TMP]/build_meta.py", line 311, in run_setup
        exec(code, locals())
      File "<string>", line 2
        from setuptools import setup
    IndentationError: unexpected indent
      × Failed to build `[TEMP_DIR]/project`
      ╰─▶ Build backend failed to determine requirements with `build_sdist()` (exit status: 1)
    "###);

    Ok(())
}

#[test]
fn workspace() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
            (r"\[project\]", "[PKG]"),
            (r"\[member\]", "[PKG]"),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [tool.uv.workspace]
        members = ["packages/*"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    let member = project.child("packages").child("member");
    fs_err::create_dir_all(member.path())?;

    member.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "member"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    member.child("src").child("__init__.py").touch()?;
    member.child("README").touch()?;

    let r#virtual = project.child("packages").child("virtual");
    fs_err::create_dir_all(r#virtual.path())?;

    r#virtual.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "virtual"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    r#virtual.child("src").child("__init__.py").touch()?;
    r#virtual.child("README").touch()?;

    // Build the member.
    uv_snapshot!(&filters, context.build().arg("--package").arg("member").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/member.egg-info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    running check
    creating member-0.1.0
    creating member-0.1.0/src
    creating member-0.1.0/src/member.egg-info
    copying files to member-0.1.0...
    copying README -> member-0.1.0
    copying pyproject.toml -> member-0.1.0
    copying src/__init__.py -> member-0.1.0/src
    copying src/member.egg-info/PKG-INFO -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/SOURCES.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/dependency_links.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/requires.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/top_level.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/SOURCES.txt -> member-0.1.0/src/member.egg-info
    Writing member-0.1.0/setup.cfg
    Creating tar archive
    removing 'member-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/member.egg-info to build/bdist.linux-x86_64/wheel/member-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/member-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'member-0.1.0.dist-info/METADATA'
    adding 'member-0.1.0.dist-info/WHEEL'
    adding 'member-0.1.0.dist-info/top_level.txt'
    adding 'member-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/member-0.1.0.tar.gz
    Successfully built dist/member-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("member-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("member-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    // Build all packages.
    uv_snapshot!(&filters, context.build().arg("--all").arg("--no-build-logs").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    [PKG] Building source distribution...
    [PKG] Building source distribution...
    [PKG] Building wheel from source distribution...
    [PKG] Building wheel from source distribution...
    Successfully built dist/member-0.1.0.tar.gz
    Successfully built dist/member-0.1.0-py3-none-any.whl
    Successfully built dist/project-0.1.0.tar.gz
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("member-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("member-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    // If a source is provided, discover the workspace from the source.
    uv_snapshot!(&filters, context.build().arg("./project").arg("--package").arg("member"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    running check
    creating member-0.1.0
    creating member-0.1.0/src
    creating member-0.1.0/src/member.egg-info
    copying files to member-0.1.0...
    copying README -> member-0.1.0
    copying pyproject.toml -> member-0.1.0
    copying src/__init__.py -> member-0.1.0/src
    copying src/member.egg-info/PKG-INFO -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/SOURCES.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/dependency_links.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/requires.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/top_level.txt -> member-0.1.0/src/member.egg-info
    copying src/member.egg-info/SOURCES.txt -> member-0.1.0/src/member.egg-info
    Writing member-0.1.0/setup.cfg
    Creating tar archive
    removing 'member-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/member.egg-info/PKG-INFO
    writing dependency_links to src/member.egg-info/dependency_links.txt
    writing requirements to src/member.egg-info/requires.txt
    writing top-level names to src/member.egg-info/top_level.txt
    reading manifest file 'src/member.egg-info/SOURCES.txt'
    writing manifest file 'src/member.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/member.egg-info to build/bdist.linux-x86_64/wheel/member-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/member-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'member-0.1.0.dist-info/METADATA'
    adding 'member-0.1.0.dist-info/WHEEL'
    adding 'member-0.1.0.dist-info/top_level.txt'
    adding 'member-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built project/dist/member-0.1.0.tar.gz
    Successfully built project/dist/member-0.1.0-py3-none-any.whl
    "###);

    // If a source is provided, discover the workspace from the source.
    uv_snapshot!(&filters, context.build().arg("./project").arg("--all").arg("--no-build-logs"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    [PKG] Building source distribution...
    [PKG] Building source distribution...
    [PKG] Building wheel from source distribution...
    [PKG] Building wheel from source distribution...
    Successfully built project/dist/member-0.1.0.tar.gz
    Successfully built project/dist/member-0.1.0-py3-none-any.whl
    Successfully built project/dist/project-0.1.0.tar.gz
    Successfully built project/dist/project-0.1.0-py3-none-any.whl
    "###);

    // Fail when `--package` is provided without a workspace.
    uv_snapshot!(&filters, context.build().arg("--package").arg("member"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: `--package` was provided, but no workspace was found
      Caused by: No `pyproject.toml` found in current directory or any parent directory
    "###);

    // Fail when `--all` is provided without a workspace.
    uv_snapshot!(&filters, context.build().arg("--all"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: `--all-packages` was provided, but no workspace was found
      Caused by: No `pyproject.toml` found in current directory or any parent directory
    "###);

    // Fail when `--package` is a non-existent member without a workspace.
    uv_snapshot!(&filters, context.build().arg("--package").arg("fail").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Package `fail` not found in workspace
    "###);

    Ok(())
}

#[test]
fn build_all_with_failure() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
            (r"\[project\]", "[PKG]"),
            (r"\[member-\w+\]", "[PKG]"),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [tool.uv.workspace]
        members = ["packages/*"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    let member_a = project.child("packages").child("member_a");
    fs_err::create_dir_all(member_a.path())?;

    let member_b = project.child("packages").child("member_b");
    fs_err::create_dir_all(member_b.path())?;

    member_a.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "member_a"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    member_a.child("src").child("__init__.py").touch()?;
    member_a.child("README").touch()?;

    member_b.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "member_b"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    member_b.child("src").child("__init__.py").touch()?;
    member_b.child("README").touch()?;

    // member_b build should fail
    member_b.child("setup.py").write_str(
        r#"
        from setuptools import setup

        setup(
            name="project",
            version="0.1.0",
            packages=["project"],
            install_requires=["foo==3.7.0"],
        )
        "#,
    )?;

    // Build all the packages
    uv_snapshot!(&filters, context.build().arg("--all").arg("--no-build-logs").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    [PKG] Building source distribution...
    [PKG] Building source distribution...
    [PKG] Building source distribution...
    [PKG] Building wheel from source distribution...
    [PKG] Building wheel from source distribution...
    Successfully built dist/member_a-0.1.0.tar.gz
    Successfully built dist/member_a-0.1.0-py3-none-any.whl
      × Failed to build `member-b @ [TEMP_DIR]/project/packages/member_b`
      ╰─▶ Build backend failed to determine requirements with `build_sdist()` (exit status: 1)
    Successfully built dist/project-0.1.0.tar.gz
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    // project and member_a should be built, regardless of member_b build failure
    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    project
        .child("dist")
        .child("member_a-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("member_a-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

#[test]
fn build_constraints() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let constraints = project.child("constraints.txt");
    constraints.write_str("setuptools==0.1.0")?;

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    uv_snapshot!(&filters, context.build().arg("--build-constraint").arg("constraints.txt").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
      × Failed to build `[TEMP_DIR]/project`
      ├─▶ Failed to resolve requirements from `build-system.requires`
      ├─▶ No solution found when resolving: `setuptools>=42`
      ╰─▶ Because you require setuptools>=42 and setuptools==0.1.0, we can conclude that your requirements are unsatisfiable.
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    Ok(())
}

#[test]
fn sha() -> Result<()> {
    let context = TestContext::new("3.8");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.8"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    // Reject an incorrect hash.
    let constraints = project.child("constraints.txt");
    constraints.write_str("setuptools==68.2.2 --hash=sha256:a248cb506794bececcddeddb1678bc722f9cfcacf02f98f7c0af6b9ed893caf2")?;

    uv_snapshot!(&filters, context.build().arg("--build-constraint").arg("constraints.txt").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
      × Failed to build `[TEMP_DIR]/project`
      ├─▶ Failed to install requirements from `build-system.requires`
      ├─▶ Failed to download `setuptools==68.2.2`
      ╰─▶ Hash mismatch for `setuptools==68.2.2`

          Expected:
            sha256:a248cb506794bececcddeddb1678bc722f9cfcacf02f98f7c0af6b9ed893caf2

          Computed:
            sha256:b454a35605876da60632df1a60f736524eb73cc47bbc9f3f1ef1b644de74fd2a
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    fs_err::remove_dir_all(project.child("dist"))?;

    // Reject an incorrect hash with --requires-hashes.
    uv_snapshot!(&filters, context.build().arg("--build-constraint").arg("constraints.txt").arg("--require-hashes").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
      × Failed to build `[TEMP_DIR]/project`
      ├─▶ Failed to install requirements from `build-system.requires`
      ├─▶ Failed to download `setuptools==68.2.2`
      ╰─▶ Hash mismatch for `setuptools==68.2.2`

          Expected:
            sha256:a248cb506794bececcddeddb1678bc722f9cfcacf02f98f7c0af6b9ed893caf2

          Computed:
            sha256:b454a35605876da60632df1a60f736524eb73cc47bbc9f3f1ef1b644de74fd2a
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    fs_err::remove_dir_all(project.child("dist"))?;

    // Reject a missing hash.
    let constraints = project.child("constraints.txt");
    constraints.write_str("setuptools==68.2.2")?;

    uv_snapshot!(&filters, context.build().arg("--build-constraint").arg("constraints.txt").arg("--require-hashes").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
      × Failed to build `[TEMP_DIR]/project`
      ├─▶ Failed to resolve requirements from `build-system.requires`
      ├─▶ No solution found when resolving: `setuptools>=42`
      ╰─▶ In `--require-hashes` mode, all requirements must be pinned upfront with `==`, but found: `setuptools`
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    fs_err::remove_dir_all(project.child("dist"))?;

    // Accept a correct hash.
    let constraints = project.child("constraints.txt");
    constraints.write_str("setuptools==68.2.2 --hash=sha256:b454a35605876da60632df1a60f736524eb73cc47bbc9f3f1ef1b644de74fd2a")?;

    uv_snapshot!(&filters, context.build().arg("--build-constraint").arg("constraints.txt").current_dir(&project), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.8.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/project-0.1.0.tar.gz
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

#[test]
fn build_quiet() -> Result<()> {
    let context = TestContext::new("3.12");

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    uv_snapshot!(&context.filters(), context.build().arg("project").arg("-q"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    "###);

    Ok(())
}

#[test]
fn build_no_build_logs() -> Result<()> {
    let context = TestContext::new("3.12");

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    uv_snapshot!(&context.filters(), context.build().arg("project").arg("--no-build-logs"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    Building wheel from source distribution...
    Successfully built project/dist/project-0.1.0.tar.gz
    Successfully built project/dist/project-0.1.0-py3-none-any.whl
    "###);

    Ok(())
}

#[test]
fn tool_uv_sources() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
        ])
        .collect::<Vec<_>>();

    let build = context.temp_dir.child("backend");
    build.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "backend"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["typing-extensions>=3.10"]

        [build-system]
        requires = ["setuptools>=42"]
        build-backend = "setuptools.build_meta"
        "#,
    )?;

    build
        .child("src")
        .child("backend")
        .child("__init__.py")
        .write_str(indoc! { r#"
            def hello() -> str:
                return "Hello, world!"
        "#})?;
    build.child("README.md").touch()?;

    let project = context.temp_dir.child("project");

    project.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig>1"]

        [build-system]
        requires = ["setuptools>=42", "backend==0.1.0"]
        build-backend = "setuptools.build_meta"

        [tool.uv.sources]
        backend = { path = "../backend" }
        "#,
    )?;

    project.child("setup.py").write_str(indoc! {r"
        from setuptools import setup

        from backend import hello

        hello()

        setup()
        ",
    })?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    uv_snapshot!(filters, context.build().current_dir(project.path()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    running egg_info
    creating src/project.egg-info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running sdist
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running check
    creating project-0.1.0
    creating project-0.1.0/src
    creating project-0.1.0/src/project.egg-info
    copying files to project-0.1.0...
    copying README -> project-0.1.0
    copying pyproject.toml -> project-0.1.0
    copying setup.py -> project-0.1.0
    copying src/__init__.py -> project-0.1.0/src
    copying src/project.egg-info/PKG-INFO -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/dependency_links.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/requires.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/top_level.txt -> project-0.1.0/src/project.egg-info
    copying src/project.egg-info/SOURCES.txt -> project-0.1.0/src/project.egg-info
    Writing project-0.1.0/setup.cfg
    Creating tar archive
    removing 'project-0.1.0' (and everything under it)
    Building wheel from source distribution...
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    running bdist_wheel
    running build
    running build_py
    creating build
    creating build/lib
    copying src/__init__.py -> build/lib
    running egg_info
    writing src/project.egg-info/PKG-INFO
    writing dependency_links to src/project.egg-info/dependency_links.txt
    writing requirements to src/project.egg-info/requires.txt
    writing top-level names to src/project.egg-info/top_level.txt
    reading manifest file 'src/project.egg-info/SOURCES.txt'
    writing manifest file 'src/project.egg-info/SOURCES.txt'
    installing to build/bdist.linux-x86_64/wheel
    running install
    running install_lib
    creating build/bdist.linux-x86_64
    creating build/bdist.linux-x86_64/wheel
    copying build/lib/__init__.py -> build/bdist.linux-x86_64/wheel
    running install_egg_info
    Copying src/project.egg-info to build/bdist.linux-x86_64/wheel/project-0.1.0-py3.12.egg-info
    running install_scripts
    creating build/bdist.linux-x86_64/wheel/project-0.1.0.dist-info/WHEEL
    creating '[TEMP_DIR]/project/dist/[TMP]/wheel' to it
    adding '__init__.py'
    adding 'project-0.1.0.dist-info/METADATA'
    adding 'project-0.1.0.dist-info/WHEEL'
    adding 'project-0.1.0.dist-info/top_level.txt'
    adding 'project-0.1.0.dist-info/RECORD'
    removing build/bdist.linux-x86_64/wheel
    Successfully built dist/project-0.1.0.tar.gz
    Successfully built dist/project-0.1.0-py3-none-any.whl
    "###);

    project
        .child("dist")
        .child("project-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    project
        .child("dist")
        .child("project-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

/// Check that we have a working git boundary for builds from source dist to wheel in `dist/`.
#[test]
fn git_boundary_in_dist_build() -> Result<()> {
    let context = TestContext::new("3.12");

    let project = context.temp_dir.child("demo");
    project.child("pyproject.toml").write_str(
        r#"
        [build-system]
        requires = ["hatchling"]
        build-backend = "hatchling.build"

        [project]
        name = "demo"
        version = "0.1.0"
        requires-python = ">=3.11"
        "#,
    )?;
    project.child("src/demo/__init__.py").write_str(
        r#"
        def run():
            print("Running like the wind!")
        "#,
    )?;

    uv_snapshot!(&context.filters(), context.build().current_dir(project.path()), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution...
    Building wheel from source distribution...
    Successfully built dist/demo-0.1.0.tar.gz
    Successfully built dist/demo-0.1.0-py3-none-any.whl
    "###);

    // Check that the source file is included
    let reader = File::open(project.join("dist/demo-0.1.0-py3-none-any.whl"))?;
    let mut files: Vec<_> = ZipArchive::new(reader)?
        .file_names()
        .map(ToString::to_string)
        .collect();
    files.sort();
    assert_snapshot!(files.join("\n"), @r###"
    demo-0.1.0.dist-info/METADATA
    demo-0.1.0.dist-info/RECORD
    demo-0.1.0.dist-info/WHEEL
    demo/__init__.py
    "###);

    Ok(())
}

#[test]
fn build_non_package() -> Result<()> {
    let context = TestContext::new("3.12");
    let filters = context
        .filters()
        .into_iter()
        .chain([
            (r"exit code: 1", "exit status: 1"),
            (r"bdist\.[^/\\\s]+-[^/\\\s]+", "bdist.linux-x86_64"),
            (r"\\\.", ""),
            (r"\[project\]", "[PKG]"),
            (r"\[member\]", "[PKG]"),
        ])
        .collect::<Vec<_>>();

    let project = context.temp_dir.child("project");

    let pyproject_toml = project.child("pyproject.toml");
    pyproject_toml.write_str(
        r#"
        [project]
        name = "project"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["anyio==3.7.0"]

        [tool.uv.workspace]
        members = ["packages/*"]
        "#,
    )?;

    project.child("src").child("__init__.py").touch()?;
    project.child("README").touch()?;

    let member = project.child("packages").child("member");
    fs_err::create_dir_all(member.path())?;

    member.child("pyproject.toml").write_str(
        r#"
        [project]
        name = "member"
        version = "0.1.0"
        requires-python = ">=3.12"
        dependencies = ["iniconfig"]
        "#,
    )?;

    member.child("src").child("__init__.py").touch()?;
    member.child("README").touch()?;

    // Build the member.
    uv_snapshot!(&filters, context.build().arg("--package").arg("member").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Package `member` is missing a `build-system`. For example, to build with `setuptools`, add the following to `packages/member/pyproject.toml`:
    ```toml
    [build-system]
    requires = ["setuptools"]
    build-backend = "setuptools.build_meta"
    ```
    "###);

    project
        .child("dist")
        .child("member-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("member-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    // Build all packages.
    uv_snapshot!(&filters, context.build().arg("--all").arg("--no-build-logs").current_dir(&project), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: Workspace does contain any buildable packages. For example, to build `member` with `setuptools`, add a `build-system` to `packages/member/pyproject.toml`:
    ```toml
    [build-system]
    requires = ["setuptools"]
    build-backend = "setuptools.build_meta"
    ```
    "###);

    project
        .child("dist")
        .child("member-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    project
        .child("dist")
        .child("member-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    Ok(())
}

/// Test the uv fast path. Tests all four possible build plans:
/// * Defaults
/// * `--sdist`
/// * `--wheel`
/// * `--sdist --wheel`
#[test]
fn build_fast_path() -> Result<()> {
    let context = TestContext::new("3.12");

    let built_by_uv = current_dir()?.join("../../scripts/packages/built-by-uv");

    uv_snapshot!(context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output1")), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution (uv build backend)...
    Building wheel from source distribution (uv build backend)...
    Successfully built output1/built_by_uv-0.1.0.tar.gz
    Successfully built output1/built_by_uv-0.1.0-py3-none-any.whl
    "###);
    context
        .temp_dir
        .child("output1")
        .child("built_by_uv-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    context
        .temp_dir
        .child("output1")
        .child("built_by_uv-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    uv_snapshot!(context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output2"))
        .arg("--sdist"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution (uv build backend)...
    Successfully built output2/built_by_uv-0.1.0.tar.gz
    "###);
    context
        .temp_dir
        .child("output2")
        .child("built_by_uv-0.1.0.tar.gz")
        .assert(predicate::path::is_file());

    uv_snapshot!(context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output3"))
        .arg("--wheel"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building wheel (uv build backend)...
    Successfully built output3/built_by_uv-0.1.0-py3-none-any.whl
    "###);
    context
        .temp_dir
        .child("output3")
        .child("built_by_uv-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    uv_snapshot!(context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output4"))
        .arg("--sdist")
        .arg("--wheel"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    Building source distribution (uv build backend)...
    Building wheel (uv build backend)...
    Successfully built output4/built_by_uv-0.1.0.tar.gz
    Successfully built output4/built_by_uv-0.1.0-py3-none-any.whl
    "###);
    context
        .temp_dir
        .child("output4")
        .child("built_by_uv-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    context
        .temp_dir
        .child("output4")
        .child("built_by_uv-0.1.0-py3-none-any.whl")
        .assert(predicate::path::is_file());

    Ok(())
}

/// Test the `--list` option.
#[test]
fn list_files() -> Result<()> {
    let context = TestContext::new("3.12");

    let built_by_uv = current_dir()?.join("../../scripts/packages/built-by-uv");

    // By default, we build the wheel from the source dist, which we need to do even for the list
    // task.
    uv_snapshot!(context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output1"))
        .arg("--list"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Building built_by_uv-0.1.0.tar.gz will include the following files:
    built_by_uv-0.1.0/LICENSE-APACHE (LICENSE-APACHE)
    built_by_uv-0.1.0/LICENSE-MIT (LICENSE-MIT)
    built_by_uv-0.1.0/PKG-INFO (generated)
    built_by_uv-0.1.0/README.md (README.md)
    built_by_uv-0.1.0/assets/data.csv (assets/data.csv)
    built_by_uv-0.1.0/header/built_by_uv.h (header/built_by_uv.h)
    built_by_uv-0.1.0/pyproject.toml (pyproject.toml)
    built_by_uv-0.1.0/scripts/whoami.sh (scripts/whoami.sh)
    built_by_uv-0.1.0/src/built_by_uv/__init__.py (src/built_by_uv/__init__.py)
    built_by_uv-0.1.0/src/built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
    built_by_uv-0.1.0/src/built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
    built_by_uv-0.1.0/src/built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
    built_by_uv-0.1.0/src/built_by_uv/build-only.h (src/built_by_uv/build-only.h)
    built_by_uv-0.1.0/src/built_by_uv/cli.py (src/built_by_uv/cli.py)
    built_by_uv-0.1.0/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
    Building built_by_uv-0.1.0-py3-none-any.whl will include the following files:
    built_by_uv-0.1.0.data/data/data.csv (assets/data.csv)
    built_by_uv-0.1.0.data/headers/built_by_uv.h (header/built_by_uv.h)
    built_by_uv-0.1.0.data/scripts/whoami.sh (scripts/whoami.sh)
    built_by_uv-0.1.0.dist-info/METADATA (generated)
    built_by_uv-0.1.0.dist-info/WHEEL (generated)
    built_by_uv-0.1.0.dist-info/entry_points.txt (generated)
    built_by_uv-0.1.0.dist-info/licenses/LICENSE-APACHE (LICENSE-APACHE)
    built_by_uv-0.1.0.dist-info/licenses/LICENSE-MIT (LICENSE-MIT)
    built_by_uv-0.1.0.dist-info/licenses/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
    built_by_uv/__init__.py (src/built_by_uv/__init__.py)
    built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
    built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
    built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
    built_by_uv/cli.py (src/built_by_uv/cli.py)

    ----- stderr -----
    Building source distribution (uv build backend)...
    Successfully built output1/built_by_uv-0.1.0.tar.gz
    "###);
    context
        .temp_dir
        .child("output1")
        .child("built_by_uv-0.1.0.tar.gz")
        .assert(predicate::path::is_file());
    context
        .temp_dir
        .child("output1")
        .child("built_by_uv-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    uv_snapshot!(context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output2"))
        .arg("--list")
        .arg("--sdist")
        .arg("--wheel"), @r###"
    success: true
    exit_code: 0
    ----- stdout -----
    Building built_by_uv-0.1.0.tar.gz will include the following files:
    built_by_uv-0.1.0/LICENSE-APACHE (LICENSE-APACHE)
    built_by_uv-0.1.0/LICENSE-MIT (LICENSE-MIT)
    built_by_uv-0.1.0/PKG-INFO (generated)
    built_by_uv-0.1.0/README.md (README.md)
    built_by_uv-0.1.0/assets/data.csv (assets/data.csv)
    built_by_uv-0.1.0/header/built_by_uv.h (header/built_by_uv.h)
    built_by_uv-0.1.0/pyproject.toml (pyproject.toml)
    built_by_uv-0.1.0/scripts/whoami.sh (scripts/whoami.sh)
    built_by_uv-0.1.0/src/built_by_uv/__init__.py (src/built_by_uv/__init__.py)
    built_by_uv-0.1.0/src/built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
    built_by_uv-0.1.0/src/built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
    built_by_uv-0.1.0/src/built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
    built_by_uv-0.1.0/src/built_by_uv/build-only.h (src/built_by_uv/build-only.h)
    built_by_uv-0.1.0/src/built_by_uv/cli.py (src/built_by_uv/cli.py)
    built_by_uv-0.1.0/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
    Building built_by_uv-0.1.0-py3-none-any.whl will include the following files:
    built_by_uv-0.1.0.data/data/data.csv (assets/data.csv)
    built_by_uv-0.1.0.data/headers/built_by_uv.h (header/built_by_uv.h)
    built_by_uv-0.1.0.data/scripts/whoami.sh (scripts/whoami.sh)
    built_by_uv-0.1.0.dist-info/METADATA (generated)
    built_by_uv-0.1.0.dist-info/WHEEL (generated)
    built_by_uv-0.1.0.dist-info/entry_points.txt (generated)
    built_by_uv-0.1.0.dist-info/licenses/LICENSE-APACHE (LICENSE-APACHE)
    built_by_uv-0.1.0.dist-info/licenses/LICENSE-MIT (LICENSE-MIT)
    built_by_uv-0.1.0.dist-info/licenses/third-party-licenses/PEP-401.txt (third-party-licenses/PEP-401.txt)
    built_by_uv/__init__.py (src/built_by_uv/__init__.py)
    built_by_uv/arithmetic/__init__.py (src/built_by_uv/arithmetic/__init__.py)
    built_by_uv/arithmetic/circle.py (src/built_by_uv/arithmetic/circle.py)
    built_by_uv/arithmetic/pi.txt (src/built_by_uv/arithmetic/pi.txt)
    built_by_uv/cli.py (src/built_by_uv/cli.py)

    ----- stderr -----
    "###);
    context
        .temp_dir
        .child("output2")
        .child("built_by_uv-0.1.0.tar.gz")
        .assert(predicate::path::missing());
    context
        .temp_dir
        .child("output2")
        .child("built_by_uv-0.1.0-py3-none-any.whl")
        .assert(predicate::path::missing());

    Ok(())
}

/// Test `--list` option errors.
#[test]
fn list_files_errors() -> Result<()> {
    let context = TestContext::new("3.12");

    let built_by_uv = current_dir()?.join("../../scripts/packages/built-by-uv");

    let mut filters = context.filters();
    // In CI, we run with link mode settings.
    filters.push(("--link-mode <LINK_MODE> ", ""));
    uv_snapshot!(filters, context.build()
        .arg("--preview")
        .arg(&built_by_uv)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output1"))
        .arg("--list")
        .arg("--force-pep517"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    error: the argument '--list' cannot be used with '--force-pep517'

    Usage: uv build --cache-dir [CACHE_DIR] --out-dir <OUT_DIR> --exclude-newer <EXCLUDE_NEWER> <SRC>

    For more information, try '--help'.
    "###);

    // Not a uv build backend package, we can't list it.
    let anyio_local = current_dir()?.join("../../scripts/packages/anyio_local");
    let mut filters = context.filters();
    // Windows normalization
    filters.push(("/crates/uv/../../", "/"));
    uv_snapshot!(filters, context.build()
        .arg("--preview")
        .arg(&anyio_local)
        .arg("--out-dir")
        .arg(context.temp_dir.join("output2"))
        .arg("--list"), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
      × Failed to build `[WORKSPACE]/scripts/packages/anyio_local`
      ╰─▶ Can only use `--list` with the uv backend
    "###);
    Ok(())
}

#[test]
fn version_mismatch() -> Result<()> {
    let context = TestContext::new("3.12");
    let anyio_local = current_dir()?.join("../../scripts/packages/anyio_local");
    context
        .build()
        .arg("--sdist")
        .arg("--out-dir")
        .arg(context.temp_dir.path())
        .arg(anyio_local)
        .assert()
        .success();
    let wrong_source_dist = context.temp_dir.child("anyio-1.2.3.tar.gz");
    fs_err::rename(
        context.temp_dir.child("anyio-4.3.0+foo.tar.gz"),
        &wrong_source_dist,
    )?;
    uv_snapshot!(context.filters(), context.build()
        .arg(wrong_source_dist.path())
        .arg("--wheel")
        .arg("--out-dir")
        .arg(context.temp_dir.path()), @r###"
    success: false
    exit_code: 2
    ----- stdout -----

    ----- stderr -----
    Building wheel from source distribution...
      × Failed to build `[TEMP_DIR]/anyio-1.2.3.tar.gz`
      ╰─▶ The source distribution declares version 1.2.3, but the wheel declares version 4.3.0+foo
    "###);
    Ok(())
}
