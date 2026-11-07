use std::{
    fs,
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

#[test]
fn textifies_nearest_colors_and_alpha_with_custom_mappings() {
    let directory =
        std::env::temp_dir().join(format!("pocket-pixel-textify-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let config = serde_json::json!({
        "palette": {"name": "test", "colors": ["#000000", "#ffffff"], "normals": ["#da80da", "#8080ff"]},
        "map": {"colors": ["a", "b"], "normals": ["→", "F"], "transparent": "/"}
    });
    let config_path = directory.join("pocket-pixel.config.json");
    fs::write(&config_path, config.to_string()).unwrap();
    let image = image::RgbaImage::from_raw(
        3,
        2,
        vec![
            10, 20, 30, 255, 240, 250, 245, 255, 255, 0, 0, 0, 20, 20, 20, 127, 20, 20, 20, 128,
            255, 255, 255, 255,
        ],
    )
    .unwrap();
    image.save(directory.join("sprite.png")).unwrap();
    let original = fs::read(directory.join("sprite.png")).unwrap();
    let result = run(&directory, &["textify", "sprite.png"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("partial alpha"));
    let path = directory.join("sprite.pp.json");
    let artifact: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(
        artifact,
        serde_json::json!({"colors": ["ab/", "/ab"], "normals": ["FF/", "/FF"]})
    );
    assert_eq!(fs::read(directory.join("sprite.png")).unwrap(), original);
    let before = fs::read(&path).unwrap();
    assert!(!run(&directory, &["textify", "sprite.png"]).status.success());
    assert_eq!(fs::read(&path).unwrap(), before);
    let result = run(
        &directory,
        &[
            "textify",
            "sprite.png",
            "-o",
            "cutoff.pp.json",
            "--alpha-threshold",
            "0",
        ],
    );
    assert!(result.status.success());
    let artifact: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(directory.join("cutoff.pp.json")).unwrap())
            .unwrap();
    assert_eq!(artifact["colors"], serde_json::json!(["ab/", "aab"]));
    let result = run(&directory, &["textify", "sprite.png", "-o", "sprite.png"]);
    assert!(!result.status.success());
    assert_eq!(fs::read(directory.join("sprite.png")).unwrap(), original);
    // Normalize the textified data to exact palette pixels through generate.
    fs::copy(&path, directory.join("normalized.pp.json")).unwrap();
    assert!(
        run(&directory, &["generate", "normalized.pp.json"])
            .status
            .success()
    );
    let normalized = image::open(directory.join("normalized.png"))
        .unwrap()
        .to_rgba8();
    assert_eq!(normalized.dimensions(), (3, 2));
    assert_eq!(
        normalized.as_raw(),
        &vec![
            0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255,
            255
        ]
    );
    // Old configs work for opaque inputs and report missing transparency when needed.
    let mut legacy = config.clone();
    legacy["map"].as_object_mut().unwrap().remove("transparent");
    fs::write(&config_path, legacy.to_string()).unwrap();
    let result = run(
        &directory,
        &["textify", "sprite.png", "-o", "missing-alpha.pp.json"],
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("map.transparent"));
    assert!(!directory.join("missing-alpha.pp.json").exists());
    image::RgbImage::from_pixel(1, 1, image::Rgb([255, 255, 255]))
        .save(directory.join("opaque.bmp"))
        .unwrap();
    assert!(run(&directory, &["textify", "opaque.bmp"]).status.success());
    let opaque: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(directory.join("opaque.pp.json")).unwrap())
            .unwrap();
    assert_eq!(
        opaque,
        serde_json::json!({"colors": ["b"], "normals": ["F"]})
    );
    // Equal distances select the earliest configured entry deterministically.
    let mut tie = config.clone();
    tie["palette"]["colors"] = serde_json::json!(["#000000", "#020000"]);
    fs::write(&config_path, tie.to_string()).unwrap();
    image::RgbImage::from_pixel(1, 1, image::Rgb([1, 0, 0]))
        .save(directory.join("tie.png"))
        .unwrap();
    assert!(run(&directory, &["textify", "tie.png"]).status.success());
    let artifact: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(directory.join("tie.pp.json")).unwrap()).unwrap();
    assert_eq!(artifact["colors"], serde_json::json!(["a"]));
    for (value, message) in [
        (serde_json::json!([]), "equal lengths"),
        (serde_json::json!(["#zzzzzz", "#ffffff"]), "#RRGGBB"),
    ] {
        let mut bad = config.clone();
        bad["palette"]["colors"] = value;
        fs::write(&config_path, bad.to_string()).unwrap();
        let result = run(
            &directory,
            &["textify", "sprite.png", "-o", "invalid.pp.json"],
        );
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains(message));
        assert!(!directory.join("invalid.pp.json").exists());
    }
    fs::write(&config_path, config.to_string()).unwrap();
    fs::write(directory.join("broken.png"), "not an image").unwrap();
    assert!(!run(&directory, &["textify", "broken.png"]).status.success());
    assert!(!directory.join("broken.pp.json").exists());
    fs::remove_dir_all(directory).unwrap();
}
