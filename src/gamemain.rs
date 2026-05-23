use rand::Rng;
const DEADLINE: f64 = 25.0; // ゲームオーバーの判定ライン（例）

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
    keywords: Vec<Keyword>,                  // 現在の単語リスト
    lastkey: char,                          // 最後に入力されたキー
    deadline: f64,                          // ゲームオーバーの判定ライン
    debug: String,                          // デバッグ用文字列
}

// ゲームの実装
impl GameMain {
    //------------------------------------------------------------------
    // ゲームの初期化
    //------------------------------------------------------------------
    pub fn new(textlist: Vec<String>) -> GameMain {
        GameMain {
            status: 0,                      // 待機状態
            textlist,                       // 単語の候補リストを初期化  
            score: 0,                       // スコアの初期化
            miss: 0,                        // ミス数の初期化
            difficult: 1.00,                // 難易度の初期化
            keywords: Vec::new(),           // 単語リストの初期化
            lastkey: ' ',                   // 最後に入力されたキー
            deadline: DEADLINE,             // ゲームオーバーの判定ライン
            debug: String::new(),           // デバッグ用文字列
        }
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
    // キー入力の処理
    //------------------------------------------------------------------
    pub fn key_input(&mut self, char: char) {
        self.lastkey = char; // 最後に入力されたキーを更新
    }

    //------------------------------------------------------------------
    // 単語に対するキー入力の処理
    //------------------------------------------------------------------
    pub fn key_input_one(keyword : &mut Keyword, char: char) -> bool{
        // 一致しない場合は処理しない
        if char != keyword.text.chars().nth(keyword.progress).unwrap() {
            return false;
        }
        // 一致する場合はタイプ済み文字数を増加
        keyword.progress += 1;
        true
    }

    //------------------------------------------------------------------
    // ゲームの更新（単語の位置を更新、ゲームオーバー判定など）
    //------------------------------------------------------------------
    pub fn update(&mut self) {
        // 落下している単語がある場合は、最後に入力されたキーに対して判定を行う
        if self.keywords.len() > 0 {
            // 入力キーに対して判定
            let _is_match = Self::key_input_one(&mut self.keywords[0], self.lastkey);
            if !_is_match && self.lastkey != ' ' {
                // ミス数を増加
                self.miss += 1;
            }
            // 単語が完成した場合の処理
            if self.keywords[0].progress >= self.keywords[0].text.len(){
                // スコアを増加
                self.score += self.keywords[0].text.len() as i32;
                // 単語をリストから削除
                self.keywords[0].clear();
                self.keywords.remove(0);
            }
        }
        // 最後に入力されたキーをリセット
        self.lastkey = ' ';

        // 一旦単語数がdifficult未満なら新しい単語を生成する
        if self.keywords.len() < self.difficult as usize {
            self.create_keyword();
        }

        // ここで単語の位置を更新し、ゲームオーバー判定
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
            }
        }
    }

    //------------------------------------------------------------------
    // ゲームの描画（単語の位置を表示、判定ラインの表示など）
    //------------------------------------------------------------------
    pub fn draw(&self) {
        // スコアと難易度の表示
        println!("\x1B[1;1HScore: {}  Difficult: {:.2}  Miss: {}",
                self.score, self.difficult, self.miss);

        // 判定ラインを表示
        let line = "-".repeat(120); // 判定ラインの文字列を生成
        println!("\x1B[{};1H{}", self.deadline as i32 + 1,line);

        // 単語群を表示
        for (num,keyword) in self.keywords.iter().enumerate().rev() {
            if num == 0 {
                let front: String = keyword.text.chars().take(keyword.progress).collect();// タイプ済み部分
                let back:  String = keyword.text.chars().skip(keyword.progress).collect();// タイプしていない部分
                println!("\x1B[{};{}H\x1B[0;32m{}\x1B[0;37m{}",
                    keyword.ypos as i32, keyword.xpos as i32, front, back);
            } else{
                println!("\x1B[{};{}H\x1B[0;90m{}",
                    keyword.ypos as i32, keyword.xpos as i32, keyword.text);
            }   
        }

        // デバッグ情報を表示
        println!("\x1B[{};1H{}", self.deadline as i32 + 2,self.debug);
    }

    //------------------------------------------------------------------
    // 難易度を変更する
    //------------------------------------------------------------------
    pub fn set_difficult(&mut self, difficult: f64) {
        self.difficult += difficult;
    }
    
    //------------------------------------------------------------------
    // ゲームの状態を取得
    //------------------------------------------------------------------
    pub fn get_status(&self) -> i32 {
        self.status
    }

    //------------------------------------------------------------------
    // デバッグ表示
    //------------------------------------------------------------------
    pub fn debug_print(&mut self, text: String) {
        self.debug = text;
    }
}   

impl Keyword {
    fn clear(&self) {
        println!("\x1B[{};{}H{}", self.ypos as i32, self.xpos as i32, " ".repeat(self.text.len()));
    }
}
