use rand::Rng;
use crate::keymanager::KeyManager;
const DEADLINE: f64 = 27.0; // ゲームオーバーの判定ライン（例）

// １オブジェクト（単語）
pub struct Keyword {
    text: String,                   // 単語テキスト
    xpos: f64,                      // 単語の横位置
    ypos: f64,                      // 単語の縦位置
    speed: f64,                     // 単語の落下速度
    progress: usize,                // タイプ済み文字数
}

// ゲームの管理データ
pub struct GameMain  {
    status: i32,                            // ゲームの状態（0:待機,1:ゲーム中,2:ゲームオーバー）
    textlist: Vec<String>,                  // 単語の候補リスト
    score: i32,                             // プレイヤーのスコア
    miss: i32,                              // ミス数
    difficult: f64,                         // ゲームの難易度（時間経過で増加）
    keywords: Vec<Keyword>,                 // 現在の単語リスト
    lastkey: char,                          // 最後に入力されたキー
    deadline: f64,                          // ゲームオーバーの判定ライン
    game_msg: String,                       // ゲームメッセージ
    key_manager: KeyManager,                // キー管理オブジェクト
    combo: i32,                              // コンボ数
    maxcombo: i32,                           // 最大コンボ数
}

// ゲームの実装
impl GameMain {
    //------------------------------------------------------------------
    // ゲームの初期化
    //------------------------------------------------------------------
    pub fn new(textlist: Vec<String>) -> GameMain {
        let mut game_main = GameMain {
            status: 0,                      // 待機状態
            textlist,                       // 単語の候補リストを初期化  
            score: 0,                       // スコアの初期化
            miss: 0,                        // ミス数の初期化
            difficult: 1.00,                // 難易度の初期化
            keywords: Vec::new(),           // 単語リストの初期化
            lastkey: ' ',                   // 最後に入力されたキー
            deadline: DEADLINE,             // ゲームオーバーの判定ライン
                                            // ゲームメッセージ初期化
            game_msg: String::from("PRESS 'S' KEY TO START('Q' KEY TO QUIT)"),
            key_manager: KeyManager::new(), // キー管理オブジェクトの初期化
            combo: 0,                        // コンボ数の初期化
            maxcombo: 0,                     // 最大コンボ数の初期化
        };
        game_main.reset();
        game_main
    }

    //------------------------------------------------------------------
    // ゲームの初期化
    //------------------------------------------------------------------
    pub fn reset(&mut self) {
        self.score = 0;                       // スコアの初期化
        self.miss = 0;                        // ミス数の初期化
        self.difficult = 1.00;                // 難易度の初期化
        self.keywords.clear();                // 単語リストのクリア
        self.lastkey = ' ';                   // 最後に入力されたキー
        self.combo = 0;                       // コンボ数の初期化
        self.maxcombo = 0;                    // 最大コンボ数の初期化
        println!("\x1B[2J"); // 画面をクリア
    }

    //------------------------------------------------------------------
    // 単語の生成
    //------------------------------------------------------------------
    pub fn create_keyword(&mut self) {
        let mut rng = rand::thread_rng();

        self.keywords.push(Keyword{
            // 候補リストからランダムに単語を選択
            text: self.textlist[rng.gen_range(0..self.textlist.len())].clone(),
            // 単語の横位置をランダムに設定
            xpos: rng.gen_range(0..=100) as f64,
            ypos: 2.0,
            speed: 0.05,
            progress: 0,
        });
    }

    //------------------------------------------------------------------
    // ゲームの更新（単語の位置を更新、ゲームオーバー判定など）
    //------------------------------------------------------------------
    pub fn update(&mut self) {
        //------------------------------
        // キー入力判定
        //------------------------------
        // 'Q'キーが入力されたらゲーム終了
        if self.lastkey == 'Q' {
            self.status = 3;
            return
        }
        // ゲーム待機状態でキーが入力されたらゲーム開始
        if self.status == 0 || self.status == 2 {
            // ゲーム開始前に'S'キーが押されていなかったらゲーム開始しない
            if self.lastkey != 'S' {
                return
            }
            self.status = 1; // ゲーム開始状態に変更
            self.game_msg = String::from(""); // ゲームメッセージをクリア
            self.reset(); // ゲームの初期化
        }

        //------------------------------
        // 一旦単語数がdifficult未満なら新しい単語を生成する
        //------------------------------
        if self.keywords.len() < self.difficult as usize {
            self.create_keyword();
        }

        //------------------------------
        // 最初のキーワードに対してキー入力の処理を行う
        //------------------------------
        // 入力キーが単語の次の文字と一致するか判定
        let _is_match = self.keywords[0].key_input_one(self.lastkey);
        // 一致しない場合
        if !_is_match && self.lastkey != ' ' {
            self.miss += 1;
            self.combo = 0; // コンボ数をリセット
        }
        // 単語が完成した場合の処理
        if self.keywords[0].progress >= self.keywords[0].text.len(){
            // コンボ数を増加
            self.combo += 1;
            if self.combo > self.maxcombo {
                self.maxcombo = self.combo; // 最大コンボ数を更新
            }
            // スコアを増加
            self.score += self.keywords[0].text.len() as i32 * (self.combo.min(10) * 10); // コンボ数に応じてスコアを増加
            // 自分の位置を消去して単語をリストから削除
            self.keywords[0].clear();
            self.keywords.remove(0);
        }
        self.lastkey = ' ';

        //------------------------------
        // キーワードの位置を更新
        //------------------------------
        for keyword in &mut self.keywords {

            // 単語の新しい位置を計算
            let new_ypos = keyword.ypos + keyword.speed;

            // 単語の位置が変わった場合の処理
            if new_ypos as i32 != keyword.ypos as i32 {
                keyword.clear();
            }

            // 単語の位置を更新
            keyword.ypos = new_ypos;
            
            // ゲームオーバー判定
            if keyword.ypos as i32 > self.deadline as i32 {
                self.status = 2; // ゲームオーバー状態に変更
                self.game_message(format!("GAME OVER!! PRESS 'S' KEY TO RESTART('Q' KEY TO QUIT)"));
                return;
            }
        }
        self.game_message(format!("WORD COUNT: {}", self.keywords.len()));
        self.set_difficult(0.003); // 難易度を徐々に上げる
        // ゲームループの実装
    }

    //------------------------------------------------------------------
    // ゲームの描画（単語の位置を表示、判定ラインの表示など）
    //------------------------------------------------------------------
    pub fn draw(&self) {
        // スコアと難易度の表示
        println!("\x1B[1;1HDifficult: {:.2}  Combo: {}(*10), Miss: {}(*-10)",
            self.difficult, self.combo, self.miss);

        // 判定ラインを表示
        let line = "-".repeat(120); // 判定ラインの文字列を生成
        println!("\x1B[{};1H{}", self.deadline as i32 + 1,line);

        // 単語群を表示
        for (num,keyword) in self.keywords.iter().enumerate().rev() {
            if num == 0 {
                let front: String = keyword.text.chars().take(keyword.progress).collect();// タイプ済み部分
                let back:  String = keyword.text.chars().skip(keyword.progress).collect();// タイプしていない部分
                let front = front.to_uppercase();
                let back = back.to_uppercase();
                println!("\x1B[{};{}H\x1B[0;32m{}\x1B[0;33m{}",
                    keyword.ypos as i32, keyword.xpos as i32, front, back);
            } else{
                println!("\x1B[{};{}H\x1B[0;90m{}",
                    keyword.ypos as i32, keyword.xpos as i32, keyword.text.to_uppercase());
            }   
        }

        // ゲームメッセージの表示
        print!("\x1B[{};1H\x1B[0;31m{}\x1b[0;37m{}",
            self.deadline as i32 + 2,self.game_msg,
            " ".repeat(120 - self.game_msg.len()));
        print!("TOTALSCORE: {}({} MAX COMBO){}",
            (self.score - self.miss * 10).max(0), self.maxcombo," ".repeat(30));

        // コンボ数が5以上の場合は特別な表示    
        let combo_msg = if self.combo >= 10 {
            format!("\x1B[0;36m【{} COMBO!!(MAX) 】", self.combo.min(10))
        }else if self.combo >= 5 {
            format!("\x1B[0;35m【{} COMBO 】", self.combo.min(10))
        } else {
            format!("{}", " ".repeat(30))
        };
        print!("\x1B[{};90H{}\x1b[0;37m", (self.deadline + 2.0) as i32,combo_msg);

    }

    //------------------------------------------------------------------
    // 難易度を変更する
    //------------------------------------------------------------------
    pub fn set_difficult(&mut self, difficult: f64) {
        self.difficult += difficult;
    }

    //------------------------------------------------------------------
    // ゲームメッセージ
    //------------------------------------------------------------------
    pub fn game_message(&mut self, text: String) {
        self.game_msg = text;
    }

    //------------------------------------------------- -----------------
    // ゲームループ
    //------------------------------------------------------------------
    pub fn run(&mut self) {
        while self.status != 3 { // ゲーム終了になるまでループ
            // キー入力チェック
            self.lastkey =  self.key_manager.get_key(); // キー入力の取得
 
            // ゲームの更新
            self.update();

            // ここでゲームの描画処理を実装（例: 単語の位置を表示）
            self.draw();

            // 処理を遅延させる（例: 100msごとに更新）
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    }   
}

impl Keyword {
    fn clear(&self) {
        println!("\x1B[{};{}H{}", self.ypos as i32, self.xpos as i32, " ".repeat(self.text.len()));
    }

    fn key_input_one(&mut self, char: char) -> bool{
        // 一致しない場合は処理しない
        if char.to_ascii_lowercase() != self.text.chars().nth(self.progress).unwrap() {
            return false;
        }
        // 一致する場合はタイプ済み文字数を増加
        self.progress += 1;
        true
    }

}
