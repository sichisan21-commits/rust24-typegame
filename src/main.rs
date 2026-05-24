mod gamemain;
mod keymanager;
use gamemain::GameMain;
use std::collections::HashMap;

//------------------------------------------------------------------
// メイン関数
//------------------------------------------------------------------
fn main() {
    // 表示キーワード初期化
    let textlist = vec![
        "apple", "apricot", "avocado","answer","asset","atom","attach","attack","attempt","attend","attract","auction","audio",
        "ball", "bear", "bell", "bird", "book", "bottle","banana", "berry", "black", "blue","bread","break","brother","build",
        "chair", "challenge", "clock", "cloud", "coffee", "contact","copy","correct","cotton","couch","cover","create","credit",
        "dark", "drink", "defeat", "dog", "door", "download", "draw", "dream","dress","drop","drum","dry","duck","dust",
        "excellent","exchange","excite","exclude","execute","expand","expect","expert","explain","explore","express","extend",
        "face", "fast", "feedback", "fire", "fish", "foot", "fork","frame","free","fresh","friend","front","fruit","fuel","fun","future",
        "gym","gypsy","gyrate","gymnast","gizmo","glove","goal","gold","good","goose","gorilla",
        "hyper","happy","hat","heart","hello","help","hero","hide","high","history","hobby","holiday","home",
        "icebox","icon","ideal","identify","idle","ignore","illuminate","illustrate","imagine",
        "juice", "jump","jungle","justice","jacket","jog","join","joke","journey","judge",
        "key", "king", "kite", "kitten", "kitchen", "knight","jazz","jeans","knife","knock","know",
        "lab", "lake", "lamp", "language", "laugh", "leaf", "lemon", "level", "light",
        "machine", "melon", "menu", "metal", "milk", "mind", "mountain", "music",
        "name", "nature", "network", "news", "night", "noise", "north", "note", "number","nutrition",
        "option","orange","order","organize","origin","other","outcome","output","outside",
        "paper", "pause", "peach", "phone", "plane","practice","predict","prefer","prepare","present",
        "question", "quick", "quiet", "quit", "quiz","quality","quantity","quarter","queen","query","quest","queue",
        "resume", "river", "road", "room", "run", "rust","rustacean","rustic","rusty","ruthless",
        "ship", "sleep", "snow", "sound", "space","spell", "spoon", "star", "start", "stone", "storm", "sugar","support",
        "table", "tea","text", "time", "train", "tree","truth","turtle","twist","type","typical","typography","typist","typhoon",
        "update","use","user","utility","utilize","utopia","utter","universe","unicorn","unique",
        "victory", "voice","vowel","voyage","vacuum","valley","valuable","vanish","variety","vast","vector",
        "walk", "water", "white", "win","wind", "window", "wood", "word", "world","worry","worship","worth","wrap","wreck",
        "xenon","xerox","x-ray","xylem","xbox",
        "yellow","yacht","yard","year","yell","yes","yoga","yolk","young","youth","yummy","yawn","yesterday",
        "zebra","zero","zone","zoom","zoo","zeal","zen","zip","zinc","zodiac","zombie","zucchini","zither",]
       .into_iter()
        .map(|word| word.to_string())
        .collect::<Vec<String>>();

    // キーワード重複があった場合はエラーを表示して終了
    let mut counts = HashMap::new();
    for word in &textlist {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("--- 重複している単語 ---");
    let mut flg = 0;
    for (word, count) in counts {
        if count > 1 {
            println!("{} ({}回)", word, count);
            flg = 1;
        }
    }
    if flg == 1 {
        return
    }

    // ゲームの生成
    let mut game_main = GameMain::new(textlist); // ゲームの初期化

    // ゲームループ
    print!("\x1B[?25l");
    game_main.run(); // ゲームの更新と描画を行う
    print!("\x1B[?25h");
}
