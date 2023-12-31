use druid::{AppLauncher, WindowDesc, Widget, PlatformError, lens};
use druid::widget::{Flex, TextBox, Button, Scroll, List, Label};
use druid::Data;
use druid::WidgetExt;
use druid::Lens;
use std::sync::Arc;

use im;

#[derive(Clone, Data, Lens)]
struct ChatState {
    message_input: String,
    chat_log: Arc<Vec<String>>,
}


impl Default for ChatState {
    fn default() -> Self {
        ChatState {
            message_input: String::new(),
            chat_log: Arc::new(Vec::new()),
        }
    }
}

pub fn run_gui() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Rust Chat Client");
    AppLauncher::with_window(main_window).launch(ChatState::default())
}

fn ui_builder() -> impl Widget<ChatState> {
    // Main vertical layout
    Flex::column()
        // Chat log
        .with_child(
            Scroll::new(List::new(|| {
                Label::new(|item: &String, _env: &_| item.clone())
            }))
            .vertical()
            .lens(ChatState::chat_log)
        )
        // Input field and send button
        .with_child(
            Flex::row()
                .with_flex_child(
                    TextBox::new()
                        .with_placeholder("Enter your message...")
                        .lens(ChatState::message_input),
                    1.0,
                )
                .with_child(
                    Button::new("Send").on_click(|_ctx, data: &mut ChatState, _env| {
                        if !data.message_input.trim().is_empty() {
                            let mut new_log = (*data.chat_log).clone();
                            new_log.push(data.message_input.clone());
                            data.chat_log = Arc::new(new_log);
                            data.message_input.clear();
                        }
                    }),
                ),
        )
}
