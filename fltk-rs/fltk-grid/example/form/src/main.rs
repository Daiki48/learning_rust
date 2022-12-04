use fltk::{
    prelude::*,
    *,
};
use fltk_grid::Grid;

// 今回の主役であるGridと、
// 名前、年齢、職業
// さらに送信ボタンを一つの構造体とする。
struct Form {
    grid: Grid,
    name: input::Input,
    age: input::IntInput, // 数字のみ
    occupation: input::Input,
    btn: button::Button,
}

impl Form {
    // Formのインスタンスを生成するメソッド
    pub fn default() -> Self {
        let mut grid = Grid::default_fill();
        // グリッドの最大列数と、最大行数を設定している
        grid.set_layout(10, 5);
        
        let name = input::Input::default();
        let age = input::IntInput::default();
        let occupation = input::Input::default();
        let btn = button::Button::default().with_label("Submit");
        let mut g = Self {
            grid,
            name,
            age,
            occupation,
            btn,
        };
        g.fill();
        g
    }

    fn fill(&mut self) {
        let grid = &mut self.grid;
        grid.debug(false);

        // 見出しタイトルを生成
        let mut title = frame::Frame::default().with_label("社員フォーム - Employee Form");
        title.set_frame(enums::FrameType::FlatBox);
        title.set_color(enums::Color::DarkMagenta);
        title.set_label_color(enums::Color::White);
        grid.insert(&mut title, 0..3, 1);

        // 2行目の1列目に「name」項目を表示
        grid.insert(&mut frame::Frame::default().with_label("Name"), 2, 1);
        // 2行目の3列目にnameの値を入力する。
        grid.insert(&mut self.name, 2, 3);

        // 4行目の1列目に「Age」項目を表示
        grid.insert(&mut frame::Frame::default().with_label("Age"), 4, 1);
        // 4行目の3列目にageの値を入力する。
        // ちなみに、ここのinputは数字しか入力出来ないIntInputだから、
        // 文字を入力しようとしても出来ない。
        grid.insert(&mut self.age, 4, 3);

        // 6行目の1列目に「Occupation(職業)」項目を表示
        grid.insert(&mut frame::Frame::default().with_label("Occupation"), 6, 1);
        // 6行目の3列目にoccupationの値を入力する。
        grid.insert(&mut self.occupation, 6, 3);

        // 8行目の2列目に「Submit」ボタンを表示する。
        grid.insert(&mut self.btn, 8, 2);
    }

    // Submitボタンを押したときの処理
    // 所有権があるから、クローンしている
    // 入力した名前、年齢、職業がそのままコンソール画面へ出力される
    fn register_default_callback(&mut self) {
        self.btn.set_callback({
            let name = self.name.clone();
            let age = self.age.clone();
            let occupation = self.occupation.clone();
            move |_| {
                println!("Name: {}", name.value());
                println!("Age: {}", age.value());
                println!("Occupation: {}", occupation.value());
            }
        })
    }

    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.grid.resize(x, y, w, h);
    }
}

fn main() {
    // いつも通りのテンプレートでウィンドウを生成
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(500, 300).with_label("Grid Form");
    let mut form = Form::default();
    form.register_default_callback();

    wind.end();

    // ウィンドウサイズを可変出来るように
    wind.make_resizable(true);

    wind.show();

    // 背景を透過出来る。
    // 値が小さくなればなるほど透明になる。(0.1とかだとほとんど透明)
    // wind.set_opacity(0.9);

    wind.resize_callback(move |_, _, _, w, h| form.resize(0, 0, w, h));

    app.run().unwrap();
}
