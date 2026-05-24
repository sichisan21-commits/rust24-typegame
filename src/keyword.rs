// １オブジェクト（単語）
pub struct Keyword {
    pub text: String,                   // 単語テキスト
    pub xpos: f64,                      // 単語の横位置
    pub ypos: f64,                      // 単語の縦位置
    pub speed: f64,                     // 単語の落下速度
    pub progress: usize,                // タイプ済み文字数
}

impl Keyword {
    //------------------------------
    // キーワードの更新処理
    //------------------------------
    pub fn update(&mut self) {
        // 単語の新しい位置を計算
        let new_ypos = self.ypos + self.speed;
        // 単語の位置が変わった場合の処理
        if new_ypos as i32 != self.ypos as i32 {
            self.clear();
        }
        // 単語の位置を更新
        self.ypos = new_ypos;
    }

    //------------------------------
    // 自分の位置を消去する
    //------------------------------
    pub fn clear(&self) {
        println!("\x1B[{};{}H{}", self.ypos as i32, self.xpos as i32, " ".repeat(self.text.len()));
    }

    //------------------------------
    // 描画する
    // P1:TRUE=入力対象／FALSE=非入力対象
    //------------------------------
    pub fn draw(&self, mode:bool) {
        if !mode {
            println!("\x1B[{};{}H\x1B[0;90m{}",
                self.ypos as i32, self.xpos as i32, self.text.to_uppercase());
        } else {
            let front: String = self.text.chars().take(self.progress).collect();// タイプ済み部分
            let back:  String = self.text.chars().skip(self.progress).collect();// タイプしていない部分
            let front = front.to_uppercase();
            let back = back.to_uppercase();
            println!("\x1B[{};{}H\x1B[0;32m{}\x1B[0;33m{}",
                self.ypos as i32, self.xpos as i32, front, back);
        }
    }

    //------------------------------
    // 入力キーに対する判定
    //------------------------------
    pub fn key_input_one(&mut self, char: char) -> bool{
        // 一致しない場合は処理しない
        if char.to_ascii_lowercase() != self.text.chars().nth(self.progress).unwrap() {
            return false;
        }
        // 一致する場合はタイプ済み文字数を増加
        self.progress += 1;
        true
    }

    //------------------------------
    // 入力が完了しているか
    //------------------------------
    pub fn is_complete(&self) -> bool {
        if self.progress >= self.text.len(){
            return true
        }
        false
    }

    //------------------------------
    // 入力が完了しているか
    //------------------------------
    pub fn length(&self) -> usize {
        self.text.len()
    }

}
