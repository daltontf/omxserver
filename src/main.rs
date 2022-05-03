use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::{fs, io};

use rouille::Response;

use askama::Template;

#[derive(Template)]
#[template(path = "list_files.html")]
struct ListFilesTemplate {
    base: String,
    files: Vec<String>,
}

#[derive(Template)]
#[template(path = "player.html")]
struct PlayerTemplate {
    file: String
}

struct PlayerState {
    player: Child,
    stdin: ChildStdin,
    file: String,
}

impl PlayerState {
    fn new(file: &str) -> PlayerState {
        let mut player = Command::new("omxplayer")
            .arg(file)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start omxplayer");
        let stdin = player.stdin.take().unwrap();
        PlayerState {
            player,
            stdin,
            file: String::from(file),
        }
    }

    fn send_key(&mut self, key: &str) -> bool {
        if self.stdin.write(key.as_bytes()).is_ok() {
            self.stdin.flush().expect("flush failed");          
            true
        } else {
            false
        }
    }

    fn quit(&mut self) -> bool {
        let result = self.send_key("q");
        self.player.wait().expect("could not wait");
        result
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:\n\tomxplayer <path to root> <tcp port>");
        std::process::exit(1);
    }

    let mut root = PathBuf::new();

    root.push(&args[1]);

    let omxplayer: Arc<Mutex<Option<PlayerState>>> = Arc::new(Mutex::new(Option::None));

    let mutex = Arc::clone(&omxplayer);

    rouille::start_server("0.0.0.0:".to_owned() + &args[2], move |request| {
        let url = request.url();

        let base = &url[1..];       

        if request.method().eq("PUT") {
            let mut maybe_player_state = mutex.lock().unwrap();
            if let Some(player_state) = maybe_player_state.as_mut() {
                let (_valid, _success) = match base {
                    "pause_resume" => (true, player_state.send_key(" ")),
                    "seek-30s" => (true, player_state.send_key("\x1b[D")),
                    "seek+30s" => (true, player_state.send_key("\x1b[C")),
                    "seek-10m" => (true, player_state.send_key("\x1b[B")),
                    "seek+10m" => (true, player_state.send_key("\x1b[A")),
                    "stop" => (true, player_state.quit()),
                    _ => (false, false)
                };
                Response::html("")
            } else {
                Response::empty_404()
            }
        } else {
            let file = root.join(base);
            if file.exists() {
                if file.is_dir() {
                    let mut files = fs::read_dir(file)
                        .unwrap()
                        .map(|res| {
                            res.map(|e| {
                                e.file_name().into_string().unwrap()
                            })
                        })
                        .collect::<Result<Vec<_>, io::Error>>()
                        .unwrap();

                    files.sort();

                    let file_list = ListFilesTemplate {
                        base: if base.len() > 0 { String::from("\\") + &base } else { String::from(base) },
                        files: files,
                    };
                    Response::html(file_list.render().unwrap())
                } else {
                    let mut maybe_player_state = mutex.lock().unwrap();
                    if let Some(player_state) = maybe_player_state.as_mut() {
                        player_state.quit();
                    }
                    let player_state = PlayerState::new(file.to_str().unwrap());

                    maybe_player_state.replace(player_state);

                    let player = PlayerTemplate {
                        file: file.file_name().unwrap().to_str().unwrap().to_string()
                    };
                    Response::html(player.render().unwrap())
                }
            } else {
                // TODO? if file not root redirect to root.
                Response::empty_404()
            }
        }
    });
}
