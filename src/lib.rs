wai_bindgen_rust::export!("./wai/plugin.wai");

mod sdk;
use sdk::{button, rows, text_input};

struct Plugin;

impl plugin::Plugin for Plugin {
    fn get_manifest() -> plugin::Manifest {
        plugin::Manifest {
            name: "text-block-plugin".to_string(),
            description: "A plugin for creating text blocks".to_string(),
            version: plugin::Version {
                major: 0,
                minor: 1,
                patch: 0,
            },
        }
    }

    fn get_ui_tree() -> plugin::UiTree {
        rows![
            text_input![
                id: "email".to_string(),
                placeholder: "Email".to_string()
            ],
            button![
                label: "Submit".to_string(),
                on_click_event: "submit-email".to_string()
            ]
        ]
    }
}
