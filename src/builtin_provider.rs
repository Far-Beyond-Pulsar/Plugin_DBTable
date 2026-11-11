//! The plugin's editor provider: its file types and editors, offered to
//! the editor while the plugin is enabled (`plugin_manager::PluginEditorProvider`,
//! registered by the root's `build`).

use gpui::AppContext;
use gpui::{App, Window};
use plugin_editor_api::*;
use plugin_manager::{BuiltinEditorProvider, EditorContext};
use std::path::PathBuf;
use std::sync::Arc;
use ui::dock::PanelView;

/// Wraps the editor_table_plugin crate as a built-in editor provider.
/// Opens `.db`, `.sqlite`, and `.sqlite3` files in the SQLite table editor.
pub struct TableEditorBuiltinProvider;

impl BuiltinEditorProvider for TableEditorBuiltinProvider {
    fn provider_id(&self) -> &str {
        "com.pulsar.table-editor"
    }

    fn file_types(&self) -> Vec<FileTypeDefinition> {
        crate::TableEditorPlugin::default().file_types()
    }

    fn editors(&self) -> Vec<EditorMetadata> {
        crate::TableEditorPlugin::default().editors()
    }

    fn can_handle(&self, editor_id: &EditorId) -> bool {
        editor_id.as_str() == "table-editor"
    }

    fn create_editor(
        &self,
        file_path: PathBuf,
        _editor_context: &EditorContext,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<Arc<dyn PanelView>, PluginError> {
        use crate::DataTableEditor;

        let panel = cx.new(|cx| {
            DataTableEditor::open_database(file_path.clone(), window, cx).unwrap_or_else(|e| {
                tracing::error!("Failed to open database: {}", e);
                DataTableEditor::new(window, cx)
            })
        });

        Ok(Arc::new(panel))
    }
}

/// Registers the editor provider.
pub(crate) fn build(p: &mut pulsar_plugins::PluginBuilder) {
    p.add(plugin_manager::PluginEditorProvider {
        origin: p.origin(),
        id: "com.pulsar.table-editor",
        create: || Arc::new(TableEditorBuiltinProvider),
    });
}
