

use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;

use core::engine_support_systems::system_management::systems::filesystems::{VFilesystem, VMetadata, VFile, OpenOptions};
use core::engine_support_systems::error_handling::error::{GameResult, GameError};

use core::engine_support_systems::data_structures::threadpools::filesystem_threadpool::FilesystemThreadPool;

//The Filesystem must:
//- Give access to files

//game name (root)
//logs
//


pub struct Metadata(fs::Metadata);
impl VMetadata for Metadata {
    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
    fn is_file(&self) -> bool {
        self.0.is_file()
    }
    fn len(&self) -> u64 {
        self.0.len()
    }
}



pub struct Filesystem {
    root: PathBuf,
    readonly: bool,
    thread_pool: FilesystemThreadPool,
}

impl fmt::Debug for Filesystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<Filesystem root: {}>", self.root.display())
    }
}

impl Filesystem {

    pub fn new(root: &Path, readonly: bool, thread_pool: FilesystemThreadPool) -> Self {
        Filesystem {
            root: root.to_path_buf(),
            readonly,
            thread_pool,
        }
    }

    fn get_absolute(&self, path: &Path) -> GameResult<PathBuf> {
        if let Some(safe_path) = self.sanitize_path(path) {
            let mut root_path = self.root.clone();
            root_path.push(safe_path);
            Ok(root_path)
        } else {
            Err(GameError::FileSystemError(format!("Path {:?} is not valid: must be an absolute path with no references to parent directories", path)))
        }
    }
}

//TODO: The API must use the threadpool to operate
impl VFilesystem for Filesystem {
    fn get_number_of_thread(&self) -> usize {
        self.thread_pool.get_number_of_thread()
    }

    fn get_thread_pool(&self) -> &FilesystemThreadPool {
        &self.thread_pool
    }

    fn shut_down(&self) -> GameResult<()> {
        Ok(())
    }

    fn open_with_options(&self, path: &Path, open_options: &OpenOptions) -> GameResult<Box<VFile>> {
        if self.readonly && (open_options.is_write() || open_options.is_create() || open_options.is_append() || open_options.is_truncate()) {
            return Err(GameError::FileSystemError(format!("Cannot alter file {:?} in root {:?}, filesystem read-only", path, self)));
        }

        let absolute_path = self.get_absolute(path)?;
        open_options.to_fs_openoptions().open(absolute_path).map(|x| {
            Box::new(x) as Box<VFile>
        }).map_err(GameError::from)
    }

    fn mkdir(&self, path: &Path) -> GameResult<()> {
        if self.readonly {
            return Err(GameError::FileSystemError(format!("Tried to create directory {:?} but the filesystem is real-only", path)));
        }
        let absolute_path = self.get_absolute(path)?;
        fs::DirBuilder::new().recursive(true).create(absolute_path).map_err(GameError::from)
    }

    fn rm(&self, path: &Path) -> GameResult<()> {
        if self.readonly {
            return Err(GameError::FileSystemError(format!("Tried to remove the file/empty directory {:?}, but the filesystem is read-only", path)));
        }

        let absolute_path = self.get_absolute(path)?;
        if absolute_path.is_dir() {
            fs::remove_dir(path).map_err(GameError::from)
        } else {
            fs::remove_file(path).map_err(GameError::from)
        }
    }

    fn rmrf(&self, path: &Path) -> GameResult<()> {
        if self.readonly {
            return Err(GameError::FileSystemError(format!("Tried to remove the file/directory {:?}, but the filesystem is read-only", path)));
        }

        let absolute_path = self.get_absolute(path)?;
        if absolute_path.is_dir() {
            fs::remove_dir_all(path).map_err(GameError::from)
        } else {
            fs::remove_file(path).map_err(GameError::from)
        }
    }

    fn exists(&self, path: &Path) -> bool {
        match self.get_absolute(path) {
            Ok(p) => p.exists(),
            _ => false,
        }
    }

    fn metadata(&self, path: &Path) -> GameResult<Box<VMetadata>> {
        let absolute_path = self.get_absolute(path)?;
        absolute_path.metadata().map(|m| {
            Box::new(Metadata(m)) as Box<VMetadata>
        }).map_err(GameError::from)
    }

    fn read_dir(&self, path: &Path) -> GameResult<Box<Iterator<Item = GameResult<PathBuf>>>> {
        let absolute_path = self.get_absolute(path)?;

        let direntry_to_path = |entry: &fs::DirEntry| -> GameResult<PathBuf> {
            let fname = entry.file_name().into_string().unwrap();
            let mut pathbuf = PathBuf::from(path);
            pathbuf.push(fname);
            Ok(pathbuf)
        };

        let itr = fs::read_dir(path)?
            .map(|entry| direntry_to_path(&entry?))
            .collect::<Vec<_>>()
            .into_iter();

        Ok(Box::new(itr))
    }

    fn to_path_buf(&self) -> Option<PathBuf> {
        Some(self.root.clone())
    }
}


//TODO: test the physical filesystem
#[cfg(test)]
mod filesystem_test {
    use super::*;

    #[test]
    fn test() {

    }
}