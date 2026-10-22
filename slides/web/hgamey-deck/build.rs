// 把 data/ 下由 refresh-data.sh 生成的 JSON 嵌进程序；游戏画面裁剪（crops.json + crops/*.png）
// 不放公开仓，从私有目录读：环境变量 HGAMEY_DECK_PRIVATE，默认 hgamey/internal/decks/hgamey-deck/private。
// 任何一处不存在时用空数据，照样能编译（结果浏览的「画面对比」页显示为空）。
use std::{env, fs, path::Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data = Path::new(&dir).join("data");
    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("data_gen.rs");
    println!("cargo:rerun-if-env-changed=HGAMEY_DECK_PRIVATE");
    let private = env::var("HGAMEY_DECK_PRIVATE").map(std::path::PathBuf::from).unwrap_or_else(|_| Path::new(&dir).join("../../../../internal/decks/hgamey-deck/private"));
    println!("cargo:rerun-if-changed={}", private.display());
    if !private.join("crops.json").exists() {
        println!("cargo:warning=私有数据目录 {} 没有 crops.json：画面对比为空", private.display());
    }
    println!("cargo:rerun-if-changed=data");
    println!("cargo:rerun-if-changed=content/deck.json");
    let mut s = String::new();
    for (name, konst) in [("leaderboard.json", "LEADERBOARD_JSON"), ("effects.json", "EFFECTS_JSON"), ("crops.json", "CROPS_JSON"), ("manifest.json", "MANIFEST_JSON")] {
        let p = if name == "crops.json" { private.join(name) } else { data.join(name) };
        println!("cargo:rerun-if-changed={}", p.display());
        if p.exists() {
            s += &format!("pub const {konst}: &str = include_str!({:?});\n", p.display().to_string());
        } else {
            s += &format!("pub const {konst}: &str = \"{{}}\";\n");
        }
    }
    s += "pub static CROPS: &[(&str, &[u8])] = &[\n";
    let cdir = private.join("crops");
    if let Ok(rd) = fs::read_dir(&cdir) {
        let mut names: Vec<_> = rd.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).filter(|n| n.ends_with(".png")).collect();
        names.sort();
        for n in names {
            let p = cdir.join(&n);
            s += &format!("    ({:?}, include_bytes!({:?})),\n", n, p.display().to_string());
        }
    }
    s += "];\n";
    fs::write(out, s).unwrap();
}
