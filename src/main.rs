mod cache;
mod helpers;
mod types;

use discord_rich_presence::activity::{Activity, Assets, Button};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
use types::{iTunesAppName, iTunesInfos, iTunesProps};
use url::Url;

use cache::Cache;
use helpers::{
    get_macos_version, get_music_player_position, get_music_props, get_music_state, is_music_open,
};

fn search_album_artwork(cache: &mut Cache, props: &iTunesProps) -> Option<iTunesInfos> {
    let query = format!("{} {}", props.artist, props.name);

    let infos = cache.get(query.clone());

    if let None = infos {
        let url = Url::parse_with_params(
            "https://itunes.apple.com/search?",
            &[
                ("media", "music"),
                ("entity", "album"),
                ("limit", "1"),
                ("term", &query),
            ],
        );

        if let Err(err) = url {
            println!("Error: {}", err);
            return None;
        }

        #[derive(Deserialize)]
        struct ResponseInner {
            #[serde(rename = "artworkUrl100")]
            artwork_url_100: String,
            #[serde(rename = "collectionViewUrl")]
            collection_view_url: String,
        }

        #[derive(Deserialize)]
        struct ResponseOuter {
            results: Vec<ResponseInner>,
        }

        let resp = minreq::get(url.unwrap()).send();

        if let Err(err) = resp {
            println!("Error: {}", err);
            return None;
        }

        let resp: ResponseOuter = serde_json::from_str(resp.unwrap().as_str().unwrap()).unwrap();

        if resp.results.len() == 0 {
            return None;
        }

        let infos = iTunesInfos {
            artwork: Some(resp.results[0].artwork_url_100.clone()),
            url: Some(resp.results[0].collection_view_url.clone()),
        };

        println!("{}: {:?}", query, infos);

        cache.set(query, infos.clone());

        return Some(infos);
    }

    return Some(infos.unwrap().to_owned());
}
fn main() {
    const MAC_OS_CATALINA: f32 = 10.15;

    let macos_ver: f32 = get_macos_version();
    let is_apple_music: bool = macos_ver >= MAC_OS_CATALINA;
    let app_name: iTunesAppName = if is_apple_music {
        iTunesAppName::Music
    } else {
        iTunesAppName::iTunes
    };

    let client_id: String = if is_apple_music {
        "773825528921849856".to_owned()
    } else {
        "979297966739300416".to_owned()
    };

    let mut client = DiscordIpcClient::new(&client_id).unwrap();

    let mut cache = Cache::new();

    cache.load_cache();

    client.connect().unwrap();

    loop {
        let is_open = is_music_open(&app_name);

        if is_open {
            let state = get_music_state(&app_name);

            println!("state: {:?}", state);

            if state == "playing" {
                let props = get_music_props(&app_name);

                let mut end = 0.0;
                if let Some(duration) = props.duration {
                    let player_position = get_music_player_position(&app_name);

                    let delta = (duration - player_position) * 1000.0;

                    let since_the_epoch = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");

                    end = (since_the_epoch.as_millis() as f64 + delta).ceil();
                }

                let mut activity = Activity::new()
                    .details(&props.name)
                    .assets(Assets::new().large_image("appicon"));

                if end > 0.0 {
                    activity = activity.timestamps(activity::Timestamps::new().end(end as i64));
                }

                if props.artist.len() > 0 {
                    activity = activity.state(&props.artist);
                }

                let p_artwork: String;
                let p_url: String;

                if props.album.len() > 0 {
                    let infos = search_album_artwork(&mut cache, &props);

                    if let Some(infos) = infos {
                        p_artwork = infos.artwork.unwrap_or_else(|| "appicon".to_string());
                        activity = activity.assets(
                            Assets::new()
                                .large_image(&p_artwork)
                                .large_text(&props.artist),
                        );

                        if let Some(url) = infos.url {
                            p_url = url;
                            activity =
                                activity.buttons(vec![Button::new("Poslouchej na Apple Music ", &p_url)]);
                        }
                    }
                }

                let set_activity = client.set_activity(activity);
                if let Err(err) = set_activity {
                    println!("Error: {}", err);
                }
            } else if state == "paused" || state == "stopped" {
                let clear_activity = client.clear_activity();
                if let Err(err) = clear_activity {
                    println!("Error: {}", err);
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
