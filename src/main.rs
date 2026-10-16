use std::str::FromStr;

use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Update,
}

struct Counter {
    dist_server: String,
    update_root: String,
}

impl Component for Counter {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            dist_server: String::from("https://mirrors.ustc.edu.cn/rust-static"),
            update_root: String::from("https://mirrors.ustc.edu.cn/rust-static/rustup"),
        }
    }

    // fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
    //     match message {

    //     }
    // }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let content = StackPanel::new()
            .spacing(8.0)
            .margin(Thickness::uniform(16.0))
            .children((
                TextBox::new(&self.dist_server).placeholder_text("Enter dist server url"),
                TextBox::new(&self.update_root).placeholder_text("Enter update root url"),
                Button::new()
                    .content("Update")
                    .horizontal_alignment(HorizontalAlignment::Stretch),
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
    App::run_component::<Counter>(()).unwrap();
}
