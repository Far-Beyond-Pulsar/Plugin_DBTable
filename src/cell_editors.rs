use crate::reflection::SqlType;
use gpui::{prelude::*, *};
use serde_json::Value;

#[derive(Clone, Debug)]
pub enum CellEditor {
    Text {
        value: String,
    },
    Integer {
        value: String,
    },
    Real {
        value: String,
    },
    Boolean {
        value: bool,
    },
    ForeignKey {
        selected_id: Option<i64>,
        options: Vec<(i64, String)>,
    },
    DateTime {
        value: String,
    },
}

impl CellEditor {
    pub fn new_from_sql_type(sql_type: &SqlType, current_value: Option<Value>) -> Self {
        match sql_type {
            SqlType::Integer => {
                let value = current_value
                    .and_then(|v| v.as_i64())
                    .map(|i| i.to_string())
                    .unwrap_or_default();
                CellEditor::Integer { value }
            }
            SqlType::Real => {
                let value = current_value
                    .and_then(|v| v.as_f64())
                    .map(|f| f.to_string())
                    .unwrap_or_default();
                CellEditor::Real { value }
            }
            SqlType::Boolean => {
                let value = current_value.and_then(|v| v.as_bool()).unwrap_or(false);
                CellEditor::Boolean { value }
            }
            SqlType::ForeignKey { .. } => CellEditor::ForeignKey {
                selected_id: current_value.and_then(|v| v.as_i64()),
                options: Vec::new(),
            },
            SqlType::DateTime => {
                let value = current_value
                    .as_ref()
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                CellEditor::DateTime { value }
            }
            _ => {
                let value = current_value
                    .map(|v| match v {
                        Value::String(s) => s,
                        _ => v.to_string(),
                    })
                    .unwrap_or_default();
                CellEditor::Text { value }
            }
        }
    }

    pub fn to_value(&self) -> Result<Value, String> {
        match self {
            CellEditor::Text { value } => Ok(Value::String(value.clone())),
            CellEditor::Integer { value } => value
                .parse::<i64>()
                .map(|i| Value::Number(i.into()))
                .map_err(|e| format!("Invalid integer: {}", e)),
            CellEditor::Real { value } => value
                .parse::<f64>()
                .map_err(|e| format!("Invalid number: {}", e))
                .and_then(|f| {
                    serde_json::Number::from_f64(f)
                        .ok_or_else(|| "Invalid float".to_string())
                        .map(Value::Number)
                }),
            CellEditor::Boolean { value } => Ok(Value::Bool(*value)),
            CellEditor::ForeignKey { selected_id, .. } => selected_id
                .map(|id| Value::Number(id.into()))
                .ok_or_else(|| "No foreign key selected".to_string()),
            CellEditor::DateTime { value } => Ok(Value::String(value.clone())),
        }
    }

    pub fn set_foreign_key_options(&mut self, options: Vec<(i64, String)>) {
        if let CellEditor::ForeignKey { options: opts, .. } = self {
            *opts = options;
        }
    }
}

pub struct CellEditorView {
    editor: CellEditor,
}

impl CellEditorView {
    pub fn new(editor: CellEditor) -> Self {
        Self { editor }
    }
}

impl IntoElement for CellEditorView {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        match &self.editor {
            CellEditor::Text { value } => div().child(value.clone()),
            CellEditor::Integer { value } => div().child(value.clone()),
            CellEditor::Real { value } => div().child(value.clone()),
            CellEditor::Boolean { value } => div().child(if *value { "✓" } else { "✗" }),
            CellEditor::ForeignKey {
                selected_id,
                options,
            } => {
                let display = if let Some(id) = selected_id {
                    options
                        .iter()
                        .find(|(opt_id, _)| *opt_id == *id)
                        .map(|(_, label)| label.clone())
                        .unwrap_or_else(|| format!("ID: {}", id))
                } else {
                    "None".to_string()
                };
                div().child(display)
            }
            CellEditor::DateTime { value } => div().child(value.clone()),
        }
        .px_2()
        .py_1()
        .text_sm()
    }
}
