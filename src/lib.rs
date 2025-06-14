wai_bindgen_rust::export!("./wai/plugin.wai");

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
        plugin::UiTree {
            nodes: vec![
                plugin::UiNode {
                    name: "rows".to_string(),
                    props: vec![],
                },
                plugin::UiNode {
                    name: "text-input".to_string(),
                    props: vec![("placeholder".to_string(), "Email".to_string())],
                },
                plugin::UiNode {
                    name: "button".to_string(),
                    props: vec![("label".to_string(), "Submit".to_string())],
                },
            ],
            children: vec![vec![1, 2], vec![], vec![]],
        }
    }
}
