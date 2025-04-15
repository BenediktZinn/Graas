use gtk::prelude::*;
use sourceview5::Language;
use sourceview5::prelude::BufferExt;
use sourceview5::{Buffer as SourceBuffer, LanguageManager, View as SourceView};

pub struct TextCanvas {
    pub source_view: SourceView,
}

impl TextCanvas {
    // Methods for TextCanvas
    pub fn new() -> Self {
        // Load Rust language definition
        let lang_manager = LanguageManager::new();
        let rust_language: Language = lang_manager
            .language("rust")
            .expect("Rust language definition not found");

        // Create source buffer and set Rust language
        let buffer = SourceBuffer::new(None);
        buffer.set_language(Some(&rust_language));
        buffer.set_highlight_syntax(true);

        let source_view = SourceView::new();
        source_view.set_buffer(Some(&buffer));

        TextCanvas { source_view }
    }

    pub fn draw(&self) {}
}
