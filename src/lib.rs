use fs4::FileExt;
use std::{
    collections::btree_map,
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    ops::Bound,
    path::PathBuf,
};

const KEY_VAL_HEADER_LEN: u32 = 4;
const MERGE_FILE_EXT: &str = "merge";

type KeyDir = std::collections::BTreeMap<Vec<u8>,(u64, u32)>;