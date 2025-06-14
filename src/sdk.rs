#![allow(unused_macros, unused_imports)]

macro_rules! rows {
    [$($child:expr),*] => {
        {
            let mut tree = plugin::UiTree {
                nodes: vec![],
                children: vec![vec![]],
            };

            tree.nodes.push(plugin::UiNode {
                name: "rows".to_string(),
                props: vec![],
            });

            let mut child_idx: u32 = 0;
            $(
                child_idx += 1;
                tree.nodes.push($child);
                tree.children[0].push(child_idx);
                tree.children.push(vec![]);
            )*

            tree
        }
    };
}

macro_rules! text_input {
    [placeholder: $placeholder:expr] => {
        {
            let placeholder: String = $placeholder;
            plugin::UiNode {
                name: "text-input".to_string(),
                props: vec![("placeholder".to_string(), placeholder)],
            }
        }
    };
}

macro_rules! button {
    [label: $label:expr] => {
        {
            let label: String = $label;
            plugin::UiNode {
                name: "button".to_string(),
                props: vec![("label".to_string(), label)],
            }
        }
    };
}

pub(crate) use button;
pub(crate) use rows;
pub(crate) use text_input;
