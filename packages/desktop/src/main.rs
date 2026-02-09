use dioxus::prelude::*;
use image::ImageReader;

use views::Home;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_window(dioxus::desktop::WindowBuilder::new().with_title("Frust"))
                .with_icon(load_icon()),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        Router::<Route> {}
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}

fn load_icon() -> dioxus::desktop::tao::window::Icon {
    const ICON_BYTES: &[u8] = include_bytes!("../../web/assets/favicon.ico");

    let decoder = ImageReader::new(std::io::Cursor::new(ICON_BYTES))
        .with_guessed_format()
        .expect("failed to detect favicon format");
    let image = decoder.decode().expect("failed to decode favicon");
    let rgba = image.into_rgba8();
    let (width, height) = rgba.dimensions();

    dioxus::desktop::tao::window::Icon::from_rgba(rgba.into_raw(), width, height)
        .expect("failed to build desktop icon")
}
