use osascript::JavaScript;
use serde::Deserialize;

use crate::types::{iTunesAppName, iTunesProps};

pub fn get_macos_version() -> f32 {
    let output = std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    output.parse().unwrap()
}

#[derive(serde::Serialize)]
struct Params {
    name: String,
}

fn run_apple_javascript<T: for<'d> Deserialize<'d>>(
    app_name: &iTunesAppName,
    command: JavaScript,
) -> T {
    let res: T = command
        .execute_with_params(Params {
            name: app_name.to_string(),
        })
        .unwrap();
    res
}

pub fn is_music_open(app_name: &iTunesAppName) -> bool {
    let command =
        JavaScript::new("return Application(\"System Events\").processes[$params.name].exists();");

    run_apple_javascript(app_name, command)
}

pub fn get_music_props(app_name: &iTunesAppName) -> iTunesProps {
    let command = JavaScript::new(
        "
    var App = Application($params.name);
    return App.currentTrack().properties();
",
    );

    run_apple_javascript(app_name, command)
}

pub fn get_music_player_position(app_name: &iTunesAppName) -> f64 {
    let command = JavaScript::new(
        "
    var App = Application($params.name);
    return App.playerPosition();
",
    );

    run_apple_javascript(app_name, command)
}

pub fn get_music_state(app_name: &iTunesAppName) -> String {
    let command = JavaScript::new(
        "
    var App = Application($params.name);
    return App.playerState();
    ",
    );

    run_apple_javascript(app_name, command)
}
