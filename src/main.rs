use editor::Editor;

mod editor;
mod terminal;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
