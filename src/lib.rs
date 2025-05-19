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
}
