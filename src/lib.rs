use gpui_kit::{AppContext, Bounds, WindowBounds, WindowOptions, component::Root, px, size};

use crate::main_view::MainView;

mod main_view;
mod version;

pub fn run() {
    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            //let bounds = Bounds::centered(None, size(px(600.0), px(200.0)), cx);

            cx.open_window(
                WindowOptions {
                    //window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(gpui_kit::TitlebarOptions {
                        title: Some("Rust Updater".into()),
                        ..Default::default()
                    }),
                    is_resizable: false,
                    is_minimizable: false,
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| MainView::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Failed to open window");
        })
        .detach();
    });
}
