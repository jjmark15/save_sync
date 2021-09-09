use assert_fs::fixture::{ChildPath, PathChild, PathCreateDir};
use assert_fs::TempDir;

pub(crate) struct TestDirectoryManager {
    test_directory: TempDir,
}

impl TestDirectoryManager {
    pub(crate) fn new() -> Self {
        let test_directory = assert_fs::TempDir::new().unwrap();
        TestDirectoryManager { test_directory }
    }

    pub(crate) fn persistence_directory(&self) -> ChildPath {
        let path = self.test_directory.child("persistence");
        if !path.exists() {
            path.create_dir_all().unwrap()
        }
        path
    }
}
