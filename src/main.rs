use windows_reactor::*;

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Reset,
}

struct Counter {
    count: i32,
}

impl Component for Counter {
    type Input = ();
    type Message = Message;

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Increment => self.count += 1,
            Message::Reset => self.count = 0,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let content = StackPanel::new().spacing(8.0).children((
            format!("Count: {}", self.count),
            Button::new()
                .on_click(context.message(Message::Increment))
                .content("Increment"),
            Button::new()
                .on_click(context.message(Message::Reset))
                .content("Reset"),
        ));

        context.window_frame("Rust Updater", content)
    }
}

fn main() {
    App::run_component::<Counter>(()).unwrap();
}