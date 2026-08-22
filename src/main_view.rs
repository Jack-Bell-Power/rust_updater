use std::process::Command;

use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, px};
use gpui_component::{
    Disableable,
    button::Button,
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
    label::Label,
    v_flex,
};

use crate::version::{get_current_version, get_latest_version};

pub struct MainView {
    current_version: String,
    latest_version: String,
    dist_server: Entity<InputState>,
    update_root: Entity<InputState>,
    can_update: bool,
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_form()
            .child(
                field().label("Version").child(
                    h_flex()
                        .px(px(8.0))
                        .gap(px(20.0))
                        .child(Label::new("Current version:").secondary(&self.current_version))
                        .child(Label::new("latest version:").secondary(&self.latest_version)),
                ),
            )
            .child(
                field().label("Mirror URL").child(
                    v_flex()
                        .px(px(8.0))
                        .gap(px(5.0))
                        .child(Input::new(&self.dist_server))
                        .child(Input::new(&self.update_root))
                        .child(
                            Button::new("b_update")
                                .label("Update")
                                .disabled(!self.can_update)
                                .on_click(cx.listener(|view, _, _, cx| {
                                    let dist_server = view.dist_server.read(cx).value().to_string();

                                    let update_root = view.update_root.read(cx).value().to_string();

                                    if dist_server.trim().is_empty()
                                        || update_root.trim().is_empty()
                                    {
                                        println!("Dist server or Update root can't be empty!");
                                        return;
                                    }

                                    Command::new("powershell")
                                        .args([
                                            "-NoExit",
                                            "-Command",
                                            &format!(
                                                "$env:RUSTUP_DIST_SERVER='{}'; \
                                                $env:RUSTUP_UPDATE_ROOT='{}'; \
                                                rustup update",
                                                dist_server, update_root
                                            ),
                                        ])
                                        .spawn()
                                        .expect("Failed to start PowerShell");
                                })),
                        ),
                ),
            )
    }
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let entity = cx.entity();

        cx.spawn(async move |_, cx| {
            let version = smol::unblock(|| get_latest_version())
                .await
                .unwrap_or_else(|_| "unknown".to_owned());

            entity
                .update(cx, |view, cx| {
                    view.latest_version = version;

                    view.can_update = view.version_compare();

                    cx.notify();
                })
                .ok();
        })
        .detach();

        let dist_server = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter dist server url")
                .default_value("https://mirrors.ustc.edu.cn/rust-static")
        });

        let update_root = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter update root url")
                .default_value("https://mirrors.ustc.edu.cn/rust-static/rustup")
        });

        Self {
            current_version: "1.97.0".to_owned(), //get_current_version(),
            latest_version: "unknown".to_owned(),
            dist_server,
            update_root,
            can_update: false,
        }
    }

    fn version_compare(&self) -> bool {
        let current: Vec<u32> = self
            .current_version
            .split('.')
            .filter_map(|x| x.parse().ok())
            .collect();

        let latest: Vec<u32> = self
            .latest_version
            .split('.')
            .filter_map(|x| x.parse().ok())
            .collect();

        current < latest
    }
}
