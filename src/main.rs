mod gamemain;
mod keymanager;
use gamemain::GameMain;

//------------------------------------------------------------------
// メイン関数
//------------------------------------------------------------------
fn main() {
    // 表示キーワード初期化
    let textlist = vec![
        "apple", "banana", "cat", "dog", "bird", "fish", "tree", "water", "fire", "earth",
        "wind", "sky", "star", "moon", "sun", "stone", "river", "mountain", "cloud", "rain",
        "snow", "storm", "light", "dark", "green", "blue", "red", "yellow", "black", "white",
        "happy", "sad", "fast", "slow", "run", "walk", "jump", "sleep", "dream", "time",
        "space", "world", "heart", "mind", "hand", "foot", "head", "face", "voice", "sound",
        "music", "game", "play", "word", "text", "code", "rust", "metal", "paper", "glass",
        "wood", "sand", "salt", "sugar", "bread", "milk", "coffee", "tea", "juice", "fruit",
        "grape", "lemon", "peach", "melon", "berry", "knife", "spoon", "fork", "chair",
        "table", "door", "window", "house", "room", "road", "car", "train", "ship", "plane",
        "boat", "clock", "phone", "light", "power", "energy", "magic", "spell", "level",
        "score", "bonus","challenge", "victory", "defeat", "win", "lose", "draw", "start",
        "end", "pause", "resume", "quit", "restart", "menu", "option", "setting", "help",
        "about", "contact", "support", "feedback", "update", "download",]
        .into_iter()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();

    // ゲームの生成
    let mut game_main = GameMain::new(textlist); // ゲームの初期化

    // ゲームループ
    print!("\x1B[?25l");
    game_main.run(); // ゲームの更新と描画を行う
    print!("\x1B[?25h");

}
