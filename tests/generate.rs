use std::{
    fs,
    io::Cursor,
    path::Path,
    process::{Command, Output},
};

fn run(directory: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pocket-pixel"))
        .args(args)
        .current_dir(directory)
        .output()
        .unwrap()
}

fn decode(path: &Path) -> (u32, u32, Vec<u8>) {
    let decoder = png::Decoder::new(Cursor::new(fs::read(path).unwrap()));
    let mut reader = decoder.read_info().unwrap();
    let mut bytes = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut bytes).unwrap();
    assert_eq!(info.color_type, png::ColorType::Rgb);
    bytes.truncate(info.buffer_size());
    (info.width, info.height, bytes)
}

#[test]
fn generates_exact_pixels_and_preserves_outputs_on_invalid_artifacts() {
    let directory =
        std::env::temp_dir().join(format!("pocket-pixel-generate-{}", std::process::id()));
    fs::create_dir_all(directory.join("art")).unwrap();
    assert!(run(&directory, &["init"]).status.success());
    let path = directory.join("art/hero.pp.json");
    let artifact = serde_json::json!({"colors": [".@", "@."], "normals": ["•→", "↑↖"]});
    fs::write(&path, artifact.to_string()).unwrap();
    let result = run(&directory, &["generate", "art/hero.pp.json"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let color_path = directory.join("art/hero.png");
    let normal_path = directory.join("art/hero_normal.png");
    assert_eq!(
        decode(&color_path),
        (
            2,
            2,
            vec![24, 20, 37, 255, 255, 255, 255, 255, 255, 24, 20, 37]
        )
    );
    assert_eq!(
        decode(&normal_path),
        (
            2,
            2,
            vec![128, 128, 255, 218, 128, 218, 128, 218, 218, 64, 191, 218]
        )
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), artifact.to_string());
    let color_before = fs::read(&color_path).unwrap();
    let normal_before = fs::read(&normal_path).unwrap();
    for (artifact, expected) in [
        (
            serde_json::json!({"colors": [".@", "."], "normals": ["••", "••"]}),
            "row 2",
        ),
        (
            serde_json::json!({"colors": ["?"], "normals": ["•"]}),
            "unmapped character",
        ),
        (
            serde_json::json!({"colors": ["."], "normals": ["••"]}),
            "matching dimensions",
        ),
        (
            serde_json::json!({"colors": [], "normals": []}),
            "non-empty",
        ),
        (serde_json::json!({"colors": ["."]}), "normals"),
    ] {
        fs::write(&path, artifact.to_string()).unwrap();
        let result = run(&directory, &["generate", "art/hero.pp.json"]);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains(expected));
        assert_eq!(fs::read(&color_path).unwrap(), color_before);
        assert_eq!(fs::read(&normal_path).unwrap(), normal_before);
    }
    let mut config: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(directory.join("pocket-pixel.config.json")).unwrap(),
    )
    .unwrap();
    config["map"]["colors"][0] = serde_json::json!(".");
    fs::write(
        directory.join("pocket-pixel.config.json"),
        config.to_string(),
    )
    .unwrap();
    fs::write(
        &path,
        serde_json::json!({"colors": ["."], "normals": ["•"]}).to_string(),
    )
    .unwrap();
    let result = run(&directory, &["generate", "art/hero.pp.json"]);
    assert!(String::from_utf8_lossy(&result.stderr).contains("duplicate character"));
    assert_eq!(fs::read(&color_path).unwrap(), color_before);
    fs::remove_dir_all(directory).unwrap();
}
