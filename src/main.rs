mod gamemain;
use gamemain::GameMain;
mod keymanager;
use keymanager::KeyManager;

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
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // キー管理の生成
    let mut key_manager = KeyManager::new();

    // ゲームの生成
    let mut game_main = GameMain::new(textlist); // ゲームの初期化
    println!("\x1B[2J"); // 画面をクリア

    // ゲームループ
    while game_main.get_status() != 2 { // ゲームオーバーになるまでループ
        // キー入力チェック
        let _key =  key_manager.get_key(); // キー入力の取得
        game_main.key_input(_key);
        game_main.debug_print(format!("Input key: {}", _key));
 
        // ゲームの更新
        game_main.update();

        // ここでゲームの描画処理を実装（例: 単語の位置を表示）
        game_main.draw();

        // 処理を遅延させる（例: 100msごとに更新）
        std::thread::sleep(std::time::Duration::from_millis(30));
        game_main.set_difficult(0.003); // 難易度を徐々に上げる
    }
}
