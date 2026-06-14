//! # Table Editor Plugin
//!
//! This plugin provides a professional database table editor for SQLite databases.
//! It supports .db, .sqlite, and .sqlite3 files with a multi-panel interface.
//!
//! ## File Types
//!
//! - **SQLite Database** (.db, .sqlite, .sqlite3)
//!   - Contains SQLite database files
//!   - Supports viewing and editing tables
//!
//! ## Editors
//!
//! - **Table Editor**: Multi-panel editor with table browser, query editor, and data view

use plugin_editor_api::*;
use std::path::PathBuf;
use std::sync::Arc;
use gpui::*;
use ui::dock::PanelView;

// Table Editor modules
pub mod database;
pub mod editor;
pub mod reflection;
pub mod query_editor;
pub mod table_view;
pub mod cell_editors;
mod workspace_panels;

// Re-export main types
pub use editor::DataTableEditor;
pub use database::DatabaseManager;
pub use reflection::TypeSchema;
pub use workspace_panels::*;

/// The Table Editor Plugin
#[derive(Default)]
pub struct TableEditorPlugin;

impl EditorPlugin for TableEditorPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: PluginId::new("com.pulsar.table-editor"),
            name: "Table Editor".into(),
            version: "0.1.0".into(),
            author: "Pulsar Team".into(),
            description: "Professional database table editor for SQLite databases".into(),
        }
    }

    fn file_types(&self) -> Vec<FileTypeDefinition> {
        vec![
            FileTypeDefinition {
                id: FileTypeId::new("database"),
                extension: "db".to_string(),
                display_name: "SQLite Database (.db)".to_string(),
                icon: ui::IconName::Database,
                color: gpui::rgb(0x4CAF50).into(),
                structure: FileStructure::Standalone,
                default_content: serde_json::Value::Null,
                categories: vec!["Data".to_string(), "SQLite".to_string()],
            },
            FileTypeDefinition {
                id: FileTypeId::new("sqlite"),
                extension: "sqlite".to_string(),
                display_name: "SQLite Database (.sqlite)".to_string(),
                icon: ui::IconName::Database,
                color: gpui::rgb(0x4CAF50).into(),
                structure: FileStructure::Standalone,
                default_content: serde_json::Value::Null,
                categories: vec!["Data".to_string(), "SQLite".to_string()],
            },
            FileTypeDefinition {
                id: FileTypeId::new("sqlite3"),
                extension: "sqlite3".to_string(),
                display_name: "SQLite Database (.sqlite3)".to_string(),
                icon: ui::IconName::Database,
                color: gpui::rgb(0x4CAF50).into(),
                structure: FileStructure::Standalone,
                default_content: serde_json::Value::Null,
                categories: vec!["Data".to_string(), "SQLite".to_string()],
            },
        ]
    }

    fn editors(&self) -> Vec<EditorMetadata> {
        vec![EditorMetadata {
            id: EditorId::new("table-editor"),
            display_name: "Table Editor".into(),
            supported_file_types: vec![
                FileTypeId::new("database"),
                FileTypeId::new("sqlite"),
                FileTypeId::new("sqlite3"),
            ],
        }]
    }

    fn create_editor(
        &self,
        editor_id: EditorId,
        file_path: PathBuf,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Arc<dyn PanelView>, PluginError> {
        if editor_id.as_str() == "table-editor" {
            let panel = cx.new(|cx| {
                DataTableEditor::open_database(file_path.clone(), window, cx)
                    .unwrap_or_else(|e| {
                        tracing::error!("Failed to open database: {}", e);
                        DataTableEditor::new(window, cx)
                    })
            });

            let panel_arc: Arc<dyn ui::dock::PanelView> = Arc::new(panel);

            log::info!("Created table editor for {:?}", file_path);
            Ok(panel_arc)
        } else {
            Err(PluginError::EditorNotFound { editor_id })
        }
    }

    fn on_load(&mut self) {
        log::info!("Table Editor Plugin loaded");
    }
}
