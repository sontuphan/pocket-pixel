use std::{fs, process::Command};

#[test]
fn previews_exact_colors_and_odd_height_without_writing_images() {
    let directory =
        std::env::temp_dir().join(format!("pocket-pixel-preview-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_pocket-pixel"))
            .args(args)
            .current_dir(&directory)
            .output()
            .unwrap()
    };
    assert!(run(&["init"]).status.success());
    let path = directory.join("hero.pp.json");
    let source =
        serde_json::json!({"colors": [".", "@", "."], "normals": ["•", "→", "↑"]}).to_string();
    fs::write(&path, &source).unwrap();
    let result = run(&["preview", "hero.pp.json"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let output = String::from_utf8(result.stdout).unwrap();
    assert_eq!(
        output,
        concat!(
            "Color image + normal map (1x3)\n",
            "\x1b[38;2;24;20;37m\x1b[48;2;255;255;255m▀\x1b[0m    ",
            "\x1b[38;2;128;128;255m\x1b[48;2;218;128;218m▀\x1b[0m\n",
            "\x1b[38;2;24;20;37m\x1b[49m▀\x1b[0m    ",
            "\x1b[38;2;128;218;218m\x1b[49m▀\x1b[0m\n"
        )
    );
    assert!(!directory.join("hero.png").exists());
    assert!(!directory.join("hero_normal.png").exists());
    assert_eq!(fs::read_to_string(&path).unwrap(), source);
    fs::write(&path, r#"{"colors":["."],"normals":["?"]}"#).unwrap();
    let invalid = run(&["preview", "hero.pp.json"]);
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("unmapped character"));
    fs::remove_dir_all(directory).unwrap();
}
