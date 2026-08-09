use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::ir::{Node, Edge};

/// A single cache entry corresponding to one parsed source file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheEntry {
    /// The SHA256 cache key for this entry.
    pub cache_key: String,
    /// The relative file path to the source file.
    pub file_path: String,
    /// The version of the CodeViz binary that generated this cache entry.
    pub codeviz_version: String,
    /// The parsed nodes for this file.
    pub nodes: Vec<Node>,
    /// The parsed edges for this file.
    pub edges: Vec<Edge>,
}

/// Manages incremental caching of CodeGraph parse results.
pub struct CacheManager {
    /// The directory where cache files are stored.
    pub cache_dir: PathBuf,
    /// The version of the running CodeViz binary.
    pub version: String,
}

impl CacheManager {
    /// Creates a new `CacheManager` with the specified cache directory and binary version.
    pub fn new<P: AsRef<Path>>(cache_dir: P, version: &str) -> Self {
        Self {
            cache_dir: cache_dir.as_ref().to_path_buf(),
            version: version.to_string(),
        }
    }

    /// Computes the SHA256 cache key for a given file based on its path, modification time, size, and file contents.
    /// Returns `None` if the file cannot be read or its metadata cannot be accessed.
    pub fn compute_cache_key<P: AsRef<Path>>(&self, file_path: P) -> Option<String> {
        let path = file_path.as_ref();
        let metadata = fs::metadata(path).ok()?;

        let mtime = metadata
            .modified()
            .ok()?
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_nanos();

        let size = metadata.len();

        let path_str = path.to_string_lossy();

        let contents = fs::read(path).ok()?;

        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        hasher.update(b"|");
        hasher.update(mtime.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(size.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(&contents);
        let result = hasher.finalize();
        Some(hex::encode(result))
    }

    /// Attempts to read a cached entry for the given file.
    /// Returns the cached `CacheEntry` if a valid one exists and matches the current version,
    /// otherwise returns `None`.
    pub fn get<P: AsRef<Path>>(&self, file_path: P) -> Option<CacheEntry> {
        let cache_key = self.compute_cache_key(file_path)?;
        let entry_path = self.cache_dir.join(format!("{}.json", cache_key));

        if !entry_path.exists() {
            return None;
        }

        let content = fs::read_to_string(&entry_path).ok()?;
        let entry: CacheEntry = serde_json::from_str(&content).ok()?;

        if entry.codeviz_version != self.version || entry.cache_key != cache_key {
            return None;
        }

        Some(entry)
    }

    /// Writes a new cache entry for the given file, nodes, and edges.
    /// Does nothing if the cache key cannot be computed or writing fails.
    pub fn put<P: AsRef<Path>>(&self, file_path: P, nodes: Vec<Node>, edges: Vec<Edge>) -> Result<(), String> {
        let cache_key = self.compute_cache_key(file_path.as_ref())
            .ok_or_else(|| "Failed to compute cache key".to_string())?;

        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }

        let entry = CacheEntry {
            cache_key: cache_key.clone(),
            file_path: file_path.as_ref().to_string_lossy().into_owned(),
            codeviz_version: self.version.clone(),
            nodes,
            edges,
        };

        let entry_path = self.cache_dir.join(format!("{}.json", cache_key));
        let json = serde_json::to_string(&entry)
            .map_err(|e| format!("Failed to serialize cache entry: {}", e))?;

        fs::write(&entry_path, json)
            .map_err(|e| format!("Failed to write cache entry to disk: {}", e))?;

        Ok(())
    }

    /// Clears the entire cache directory.
    pub fn clear(&self) -> Result<(), String> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)
                .map_err(|e| format!("Failed to remove cache directory: {}", e))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use crate::ir::{NodeKind, EdgeKind};

    #[test]
    fn test_cache_roundtrip() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().join(".codeviz_cache");
        let manager = CacheManager::new(&cache_dir, "0.1.0");

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "hello world").unwrap();

        let nodes = vec![Node {
            id: "test::id".to_string(),
            label: "test".to_string(),
            kind: NodeKind::File,
            file_path: "test".to_string(),
            line: None,
            is_public: true,
        }];

        let edges = vec![Edge {
            from_id: "test::id".to_string(),
            to_id: "test::id2".to_string(),
            kind: EdgeKind::Imports,
        }];

        // Put to cache
        assert!(manager.put(file.path(), nodes.clone(), edges.clone()).is_ok());

        // Get from cache
        let entry = manager.get(file.path()).expect("Should get cache entry");
        assert_eq!(entry.nodes, nodes);
        assert_eq!(entry.edges, edges);
    }

    #[test]
    fn test_cache_miss_on_mtime_change() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().join(".codeviz_cache");
        let manager = CacheManager::new(&cache_dir, "0.1.0");

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "hello world").unwrap();

        let nodes = vec![];
        let edges = vec![];

        assert!(manager.put(file.path(), nodes.clone(), edges.clone()).is_ok());

        assert!(manager.get(file.path()).is_some());

        // Modify file
        writeln!(file, "modified").unwrap();
        file.flush().unwrap();

        // Ensure mtime actually changes (sometimes tests run too fast for filesystem precision)
        let _ = file.as_file().set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(1));

        // Should be a cache miss
        assert!(manager.get(file.path()).is_none());
    }

    #[test]
    fn test_cache_clear() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().join(".codeviz_cache");
        let manager = CacheManager::new(&cache_dir, "0.1.0");

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "hello world").unwrap();

        assert!(manager.put(file.path(), vec![], vec![]).is_ok());

        assert!(cache_dir.exists());
        let count = std::fs::read_dir(&cache_dir).unwrap().count();
        assert!(count > 0);

        assert!(manager.clear().is_ok());
        assert!(!cache_dir.exists());
    }
}

    #[test]
    fn test_no_cache_bypass() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().join(".codeviz_cache");
        let manager = CacheManager::new(&cache_dir, "0.1.0");

        let mut file = tempfile::NamedTempFile::new().unwrap();
        use std::io::Write;
        writeln!(file, "hello world").unwrap();

        assert!(manager.put(file.path(), vec![], vec![]).is_ok());
        assert!(manager.get(file.path()).is_some());
    }
