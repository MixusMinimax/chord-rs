/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::path::{Path, PathBuf};

use crate::proto::compile_proto;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    compile_proto(&out_dir);
}

mod proto {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::path::{Path, PathBuf};

    use cargo_emit::{rerun_if_changed, warning};
    use prost_build::Config;
    use walkdir::WalkDir;

    use crate::{build_rs, read_hash, write_hash};

    pub fn compile_proto(out_dir: &Path) {
        let (proto_root, proto_files) = proto_files();
        if !should_compile(out_dir, &proto_files) {
            eprintln!("Proto files have not changed, skipping compilation");
            return;
        }
        rerun_if_changed!(proto_root.as_os_str().to_str().unwrap());
        let mut config = Config::new();
        config
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
            .out_dir(out_dir)
            .default_package_filename("_")
            .include_file("_.include.rs")
            .compile_protos(&proto_files, &[proto_root])
            .unwrap();
    }

    fn proto_files() -> (PathBuf, Vec<PathBuf>) {
        let proto_root = Path::new("../chord-api/src/proto");
        (
            proto_root.into(),
            WalkDir::new(proto_root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "proto"))
                .map(|e| e.path().to_path_buf())
                .collect(),
        )
    }

    fn should_compile(out_dir: &Path, proto_files: &[impl AsRef<Path>]) -> bool {
        let mut hasher = DefaultHasher::new();
        proto_files.iter().for_each(|e| match std::fs::read(e) {
            Ok(content) => content.hash(&mut hasher),
            Err(e) => warning!("Failed to read proto file: {}", e),
        });
        let new_hash = hasher.finish();
        let path = hash_path(out_dir);
        let old_hash = read_hash(&path);
        let has_changed = old_hash.map_or(true, |old_hash| old_hash != new_hash);
        if has_changed {
            write_hash(&path, new_hash);
        }
        if build_rs::has_changed(out_dir) {
            warning!("build.rs has changed, recompiling proto files");
            true
        } else if has_changed {
            warning!("Proto files have changed, recompiling proto files");
            true
        } else {
            false
        }
    }

    fn hash_path(out_dir: &Path) -> PathBuf {
        out_dir.join("proto.hash")
    }
}

mod build_rs {
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::path::{Path, PathBuf};

    use crate::{read_hash, write_hash};

    static mut HAS_CHANGED: Option<bool> = None;

    /// checks if the build.rs file has changed, and caches the result
    pub fn has_changed(out_dir: &Path) -> bool {
        if let Some(has_changed) = unsafe { HAS_CHANGED } {
            return has_changed;
        }
        let path = hash_path(out_dir);
        let mut hasher = DefaultHasher::new();
        let build_rs_content =
            std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("build.rs")).unwrap();
        build_rs_content.hash(&mut hasher);
        let new_hash = hasher.finish();
        let has_changed = read_hash(&path).map_or(true, |old_hash| old_hash != new_hash);
        if has_changed {
            write_hash(&path, new_hash);
        }
        unsafe { HAS_CHANGED = Some(has_changed) };
        has_changed
    }

    fn hash_path(out_dir: &Path) -> PathBuf {
        out_dir.join("build.rs.hash")
    }
}

pub fn read_hash(path: &Path) -> Option<u64> {
    std::fs::read(path)
        .ok()
        .and_then(|s| Some(u64::from_le_bytes(s.try_into().ok()?)))
}

pub fn write_hash(path: &Path, hash: u64) {
    std::fs::write(path, hash.to_le_bytes()).unwrap();
}