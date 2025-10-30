use base64::Engine as _;
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;
use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

const DEFAULT_NAME: &str = "level1";
const DATA_URL_PREFIX: &str = "data:image/png;base64,";

// flexible error type without anyhow
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Parser, Debug)]
#[command(name = "xtask", about = "Map downloader & processor")]
struct Cli {
    #[arg(short, long, default_value=DEFAULT_NAME)]
    name: String,
}



pub fn download_map(map_name: &str, dest_path: &Path) -> Result<()> {
    let url = format!("https://shmul.dev/maps/{}", map_name);

    let client = Client::new();
    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Failed to GET {url}: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("❌ Error: {} not found (HTTP {})", map_name, response.status()).into());
    }

    let bytes = response.bytes().map_err(|e| format!("Failed to read body: {e}"))?;

    fs::write(dest_path, &bytes)
        .map_err(|e| format!("Failed to write {}: {e}", dest_path.display()))?;

    println!("✅ Downloaded {map_name} → {}", dest_path.display());
    Ok(())
}

/// Load JSON file and extract base64 strings from `layerImages.tileFg` and `layerImages.entities`.
pub fn extract_layer_images(json_path: &Path) -> Result<(Option<String>, Option<String>)> {
    let text = fs::read_to_string(json_path).map_err(|_| format!("Failed to read {}", json_path.display()))?;
    let root: Value = serde_json::from_str(&text).map_err(|_| format!("Invalid JSON, {}", json_path.display()))?;

    let tile_fg = root
        .get("layerImages")
        .and_then(|v| v.get("tileFg"))
        .and_then(|v| v.as_str())
        .map(|s| s.strip_prefix(DATA_URL_PREFIX).unwrap_or(s).to_string());

    let entities = root
        .get("layerImages")
        .and_then(|v| v.get("entity"))
        .and_then(|v| v.as_str())
        .map(|s| s.strip_prefix(DATA_URL_PREFIX).unwrap_or(s).to_string());

    Ok((tile_fg, entities))
}

pub fn save_base64_png(b64_img: &str, dest_path: &Path) -> Result<()> {
    // Ensure prefix is present
    let trimmed = b64_img;

    // Decode base64 into bytes
    let bytes = match base64::engine::general_purpose::STANDARD.decode(trimmed.as_bytes()) {
        Ok(b) => b,
        Err(e) => return Err(format!("Failed to decode base64: {e}").into()),
    };

    // Write to file
    fs::write(dest_path, &bytes).map_err(|_| format!("Failed to write {}", dest_path.display()))?;

    Ok(())
}

pub fn rewrite_json_fields(json_path: &Path, tile_path: Option<&Path>, entity_path: Option<&Path>) -> Result<()> {
    let text = fs::read_to_string(json_path).map_err(|_| format!("Failed to read {}", json_path.display()))?;

    let mut root: Value = serde_json::from_str(&text).map_err(|_| format!("Invalid JSON in {}", json_path.display()))?;

    if let Some(tile) = tile_path {
        if let Some(obj) = root.get_mut("layerImages").and_then(|v| v.as_object_mut()) {
            obj.insert("tileFg".to_string(), Value::String(tile.to_string_lossy().into()));
        }
    }

    if let Some(entity) = entity_path {
        if let Some(obj) = root.get_mut("layerImages").and_then(|v| v.as_object_mut()) {
            obj.insert("entity".to_string(), Value::String(entity.to_string_lossy().into()));
        }
    }

    let pretty = serde_json::to_string_pretty(&root)?;
    fs::write(json_path, pretty)?;

    Ok(())
}

pub fn find_assets_dir() -> Option<PathBuf> {
    // start from current working directory
    let mut dir = env::current_dir().ok()?;
    loop {
        let candidate = dir.join("assets");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !dir.pop() {
            break; // we hit the filesystem root
        }
    }
    None
}

pub fn ensure_dir<P: AsRef<Path>>(dir: P) -> Result<PathBuf> {
    let dir_ref = dir.as_ref();
    if !dir_ref.exists() {
        fs::create_dir_all(dir_ref).map_err(|_| format!("Failed to create directory {}", dir_ref.display()))?;
    }
    Ok(dir_ref.to_path_buf())
}

pub fn run(map_name: &str) -> Result<()> {

    let assets_dir = find_assets_dir().unwrap_or_else(|| {
        let p = std::env::current_dir().unwrap().join("assets");
        std::fs::create_dir_all(&p).expect("Failed to create ./assets");
        p
    });

    let map_dir = ensure_dir(assets_dir.join(&map_name))?;
    let dest = map_dir.join(format!("{map_name}.json"));
    let ouput_json_name = format!("{}.json", map_name);

    download_map(&ouput_json_name, &dest)?;


    let (tile_fg_b64, entities_b64) = extract_layer_images(&dest)?;

    // find assets dir or create one
    // println!("📂 Using assets dir: {}", assets_dir.display());

    let tile_asset_path = PathBuf::from(format!("{map_name}/tile_fg.png"));
    let entity_asset_path = PathBuf::from(format!("{map_name}/entity.png"));

    // now when saving:
    if let Some(b64) = tile_fg_b64 {
        let out = map_dir.join("tile_fg.png");
        save_base64_png(&b64, &out)?;
        println!("✅ Wrote {}", out.display());
    };

    if let Some(b64) = entities_b64 {
        let out = map_dir.join("entity.png");
        save_base64_png(&b64, &out)?;
        println!("✅ Wrote {}", out.display());
    };

    rewrite_json_fields(
        &dest,
        Some(&tile_asset_path),
        Some(&entity_asset_path),
    )?;

    Ok(())
}
