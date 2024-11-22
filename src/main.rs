use std::collections::HashMap;
use druid::{AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Label, TextBox, Flex};
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::fs::File;


#[derive(Clone, Data, Lens)]
struct AppState {
    input_word: String,
    result: String,
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Word Unscrambler")
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        input_word: String::new(),
        result: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    
    
    
}

fn build_ui() -> impl Widget<AppState> {
    let input_box = TextBox::new()
        .with_placeholder("Enter scrambled word")
        .lens(AppState::input_word);

    let result_label = Label::new(|data: &AppState, _env: &_| data.result.clone())
        .background(Color::BLACK)
        .padding(10.0);

    let unscramble_button = Button::new("Unscramble")
        .on_click(|_ctx, data: &mut AppState, _env| {

            data.result = unscramble(&data.input_word);
        });

    let layout = Flex::column()
        .with_child(input_box)
        .with_spacer(10.0)
        .with_child(unscramble_button)
        .with_spacer(10.0)
        .with_child(result_label);

    layout.center()
}

fn unscramble(input: &str) -> String {

    input.chars().rev().collect::<String>();

    let mut matches = String::new();
    let search_str = input.trim().to_lowercase();

    

    let path = "/mnt/5CD266C7D266A54C/Miscellaneous Files/Personal Programming/Rust Programs/project1/src/words.txt";
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
        
    let search_char_count = build_char_count(&search_str);
        
    for line in reader.lines() {
        let line = line.expect("Unable to open file").trim().to_lowercase();
        
        if line.len() != search_str.len() {
            continue;
        }
        
        if build_char_count(&line) == search_char_count {
            matches.push_str(&line);
            matches.push('\n');
        }
    }
        
    return matches;

}

fn build_char_count(s: &str) -> HashMap<char, usize> {
    let mut char_count = HashMap::new();
    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    char_count
}

