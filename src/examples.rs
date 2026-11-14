//! The plugin's example files: SQLite databases (`.db`, `.sqlite`,
//! `.sqlite3`), provided read-only into every project under the plugin's
//! folder at `examples/`.

use std::sync::Arc;

use engine_fs::{EmbeddedFile, FolderSource, MemoryFolder, PluginFolder};

/// Every embedded example, at its path inside the plugin's folder.
pub const EXAMPLES: &[EmbeddedFile] = engine_fs::embedded_files!("examples_assets":
    "examples/README.md",
    "examples/items.db",
    "examples/dialogue.sqlite",
    "examples/scores.sqlite3",
);

fn open(folder: PluginFolder) -> Arc<dyn FolderSource> {
    Arc::new(MemoryFolder::with_files(folder, EXAMPLES))
}

/// Register the plugin's file provider (the plugin's `build` calls this).
pub(crate) fn build(p: &mut pulsar_plugins::PluginBuilder) {
    p.add(engine_fs::FileProvider {
        origin: p.origin(),
        open,
        // Embedded in the plugin; a game serves the packager's snapshot.
        in_game: engine_fs::InGame::Snapshot,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseManager;

    #[test]
    fn every_database_example_opens_with_the_plugins_database_manager() {
        let dir = std::env::temp_dir().join(format!("table_examples_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut count = 0;
        for file in EXAMPLES.iter().filter(|f| !f.path.ends_with(".md")) {
            let name = file.path.rsplit('/').next().unwrap();
            let path = dir.join(name);
            std::fs::write(&path, file.bytes).unwrap();
            let db = DatabaseManager::new(&path)
                .unwrap_or_else(|e| panic!("{}: does not open: {e}", file.path));
            db.introspect_and_register_schemas().unwrap();
            let tables = db.list_tables().unwrap();
            assert!(tables.len() >= 2, "{}: {tables:?}", file.path);
            for table in &tables {
                db.get_row_count(table)
                    .unwrap_or_else(|e| panic!("{}: {table}: {e}", file.path));
            }
            assert!(
                db.get_row_count("_about").unwrap() == 1,
                "{}: an _about note",
                file.path
            );
            count += 1;
        }
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(count, 3);
    }

    #[test]
    fn the_readme_is_embedded() {
        assert!(EXAMPLES
            .iter()
            .any(|f| f.path == "examples/README.md" && !f.bytes.is_empty()));
    }
}
