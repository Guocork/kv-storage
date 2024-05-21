use fs4::FileExt;
use std::{
    collections::btree_map, error, io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write}, ops::Bound, path::{self, PathBuf}
};

const KEY_VAL_HEADER_LEN: u32 = 4;
const MERGE_FILE_EXT: &str = "merge";

type KeyDir = std::collections::BTreeMap<Vec<u8>,(u64, u32)>;

pub type Result<T> = std::result::Result<T, std::io::Error>;


struct Log {
    path: PathBuf,  // 路径缓冲区 let path1 = PathBuf::from("some/directory/file.txt"); PathBuf 本身是用于表示单个文件系统路径
    file: std::fs::File  // File 结构体允许你打开、创建、读取和写入文件。
}

pub struct MiniBitcask {
    log: Log,
    keydir: KeyDir
}

impl Log {
    fn new(path: PathBuf) -> Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }

        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;

        file.try_lock_exclusive()?;

        Ok(Self { path, file })
    }
}



// impl Drop for MiniBitcask {
//     fn drop(&mut self) {
//         if let Err(error) = self.flush() {
//             log::error!("failed to flush file: {:?}", error)
//         }
//     }
// }

// impl MiniBitcask {
//     fn new(path: PathBuf) -> Result<Self> {
//         let mut log = Log::new(path)?;
//     }
// }