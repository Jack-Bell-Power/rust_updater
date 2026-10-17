use windows_reactor::*;

use crate::version::{get_current_version, get_latest_version};

mod version;

enum Message {
    Update,
    VersionChecked { current: String, latest: String },
    Failed,
}

struct RustUpdater {
    dist_server: String,
    update_root: String,
    current_version: String,
    latest_version: String,
}

impl Component for RustUpdater {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), context: &ComponentContext<Self>) -> Self {
        let _task = context.spawn_background(move |_| {
            let current = get_current_version();
            match get_latest_version() {
                Ok(latest) => Message::VersionChecked { current, latest },
                Err(_) => Message::Failed,
            }
        });

        Self {
            dist_server: String::from("https://mirrors.ustc.edu.cn/rust-static"),
            update_root: String::from("https://mirrors.ustc.edu.cn/rust-static/rustup"),
            current_version: String::from("Checking..."),
            latest_version: String::from("Checking..."),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Update => {}
            Message::VersionChecked { current, latest } => {
                self.current_version = current;
                self.latest_version = latest;
            }
            Message::Failed => {
                self.current_version = String::from("Failed");
                self.latest_version = String::from("Failed");
            }
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let content = StackPanel::new()
            .spacing(8.0)
            .margin(Thickness::uniform(16.0))
            .children((
                format!("Current version: {}", self.current_version),
                format!("Latest version: {}", self.latest_version),
                TextBox::new(&self.dist_server).placeholder_text("Enter dist server url"),
                TextBox::new(&self.update_root).placeholder_text("Enter update root url"),
                Button::new()
                    .content("Update")
                    .horizontal_alignment(HorizontalAlignment::Stretch),
                Border::new()
                    .height(292.0)
                    .background(Color::rgb(238, 242, 246))
                    .border_brush(Color::rgb(204, 213, 223))
                    .border_thickness(Thickness::uniform(1.0))
                    .padding(Thickness::uniform(8.0))
                    .corner_radius(CornerRadius::uniform(8.0))
                    .content(RichTextBlock::new().font_size(14.0).paragraphs(
                        RichText::single_paragraph([RichTextInline::Run(RichTextRun::plain(
                            "Ready.",
                        ))]),
                    )),
            ));

        context.window_visuals(
            WindowVisuals::new()
                .backdrop(WindowBackdrop::Mica)
                .client_size(350.0, 500.0),
        );
        context.window_frame("Rust Updater", content)
    }
}

fn main() {
    App::run_component::<RustUpdater>(()).unwrap();
}
