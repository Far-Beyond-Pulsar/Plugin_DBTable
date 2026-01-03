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
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
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

/// Storage for editor instances owned by the plugin
struct EditorStorage {
    panel: Arc<dyn ui::dock::PanelView>,
    wrapper: Box<TableEditorWrapper>,
}

/// The Table Editor Plugin
pub struct TableEditorPlugin {
    editors: Arc<Mutex<HashMap<usize, EditorStorage>>>,
    next_editor_id: Arc<Mutex<usize>>,
}

impl Default for TableEditorPlugin {
    fn default() -> Self {
        Self {
            editors: Arc::new(Mutex::new(HashMap::new())),
            next_editor_id: Arc::new(Mutex::new(0)),
        }
    }
}

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
        logger: &plugin_editor_api::EditorLogger,
    ) -> Result<(Arc<dyn PanelView>, Box<dyn EditorInstance>), PluginError> {
        logger.info("TABLE EDITOR LOADED!!");
        if editor_id.as_str() == "table-editor" {
            let panel = cx.new(|cx| {
                DataTableEditor::open_database(file_path.clone(), window, cx)
                    .unwrap_or_else(|e| {
                        tracing::error!("Failed to open database: {}", e);
                        DataTableEditor::new(window, cx)
                    })
            });

            let panel_arc: Arc<dyn ui::dock::PanelView> = Arc::new(panel.clone());
            let wrapper = Box::new(TableEditorWrapper {
                panel: panel.into(),
                file_path: file_path.clone(),
            });

            let id = {
                let mut next_id = self.next_editor_id.lock().unwrap();
                let id = *next_id;
                *next_id += 1;
                id
            };

            self.editors.lock().unwrap().insert(id, EditorStorage {
                panel: panel_arc.clone(),
                wrapper: wrapper.clone(),
            });

            log::info!("Created table editor instance {} for {:?}", id, file_path);
            Ok((panel_arc, wrapper))
        } else {
            Err(PluginError::EditorNotFound { editor_id })
        }
    }

    fn on_load(&mut self) {
        log::info!("Table Editor Plugin loaded");
    }

    fn on_unload(&mut self) {
        let mut editors = self.editors.lock().unwrap();
        let count = editors.len();
        editors.clear();
        log::info!("Table Editor Plugin unloaded (cleaned up {} editors)", count);
    }
}

#[derive(Clone)]
pub struct TableEditorWrapper {
    panel: Entity<DataTableEditor>,
    file_path: std::path::PathBuf,
}

impl plugin_editor_api::EditorInstance for TableEditorWrapper {
    fn file_path(&self) -> &std::path::PathBuf {
        &self.file_path
    }

    fn save(&mut self, window: &mut Window, cx: &mut App) -> Result<(), PluginError> {
        self.panel.update(cx, |panel, cx| {
            panel.plugin_save(window, cx)
        })
    }

    fn reload(&mut self, window: &mut Window, cx: &mut App) -> Result<(), PluginError> {
        self.panel.update(cx, |panel, cx| {
            panel.plugin_reload(window, cx)
        })
    }

    fn is_dirty(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

export_plugin!(TableEditorPlugin);
