use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::vec;

pub struct Icon {
    pub _manifest: Value,
    pub root: RootElement,
}

impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon").field("root", &self.root).finish()
    }
}

#[derive(Debug)]
pub struct RootElement {
    pub children: Vec<GroupElement>,
}

#[derive(Debug)]
pub struct GroupElement {
    pub children: Vec<LayerElement>,
}

pub struct LayerElement {
    pub position: (f64, f64),
    pub scale: f64,
    pub image: Vec<u8>,
    pub image_type: String,
}

impl std::fmt::Debug for LayerElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LayerElement")
            .field("position", &self.position)
            .field("scale", &self.scale)
            .field("image", &format!("{} bytes", &self.image.len()))
            .finish()
    }
}

pub fn parse(input: &str, background_option: Option<String>) -> Icon {
    let input_path = Path::new(input);
    let mut manifestfile: Vec<u8> = Vec::new();
    let mut asset_cache: HashMap<String, Vec<u8>> = HashMap::new();

    if input_path.is_dir() {
        let folder = fs::read_dir(input_path).expect("Failed to open folder");
        for file in folder {
            let file = file.expect("Failed to read manifest file");
            if file.path() == format!("{input}/icon.json") {
                manifestfile = fs::read(file.path()).expect("Failed to read manifest");
            }
            if file.path().is_dir() && file.file_name() == "Assets" {
                let assets = fs::read_dir(file.path()).expect("Failed to read assets folder");
                for asset in assets {
                    let asset = asset.expect("Failed to read asset file");
                    let buffer = fs::read(asset.path()).expect("Failed to read asset file");
                    asset_cache.insert(asset.file_name().into_string().unwrap(), buffer);
                }
            }
        }
    }

    let manifest: serde_json::Value =
        serde_json::from_slice(&manifestfile).expect("Failed to parse json");

    let mut groups: Vec<GroupElement> = Vec::new();
    for group in manifest["groups"].as_array().unwrap_or(&vec![]) {
        let group_pos = parse_position(group);

        let mut layers: Vec<LayerElement> = Vec::new();
        for layer in group["layers"].as_array().unwrap_or(&vec![]) {
            let layer_pos = parse_position(layer);
            let (position, scale) = add_position(group_pos, layer_pos);
            let (image, image_type) = read_image(layer, &asset_cache);
            layers.push(LayerElement {
                position,
                scale,
                image,
                image_type,
            });
        }
        groups.push(GroupElement { children: layers });
    }

    if let Some(background) = background_option {
        let input_path = Path::new(&background);
        let file = fs::read(input_path).expect("Failed to read background file");
        let image_type = input_path
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        groups.push(GroupElement {
            children: vec![LayerElement {
                position: (0.0, 0.0),
                scale: 1.0,
                image: file,
                image_type,
            }],
        });
    }

    Icon {
        _manifest: manifest,
        root: RootElement { children: groups },
    }
}

pub fn parse_position(root: &Value) -> ((f64, f64), f64) {
    let position = root.get("position").unwrap_or(&Value::Null);
    let translation_x = position["translation-in-points"]
        .get(0)
        .unwrap_or(&json!(0.0))
        .as_f64()
        .unwrap_or(0.0);
    let translation_y = position["translation-in-points"]
        .get(1)
        .unwrap_or(&json!(0.0))
        .as_f64()
        .unwrap_or(0.0);
    let scale = position["scale"].as_f64().unwrap_or(1.0);
    return ((translation_x, translation_y), scale);
}

pub fn add_position(a: ((f64, f64), f64), b: ((f64, f64), f64)) -> ((f64, f64), f64) {
    return ((a.0.0 + b.0.0, a.0.1 + b.0.1), a.1 * b.1);
}

pub fn read_image(layer: &Value, asset_cache: &HashMap<String, Vec<u8>>) -> (Vec<u8>, String) {
    let image_name = layer["image-name"].as_str().unwrap_or("");
    let image_type = image_name
        .split(".")
        .last()
        .unwrap_or("svg")
        .to_string()
        .to_lowercase();
    let image_bytes = asset_cache.get(image_name).unwrap_or(&vec![]).clone();
    return (image_bytes, image_type);
}
