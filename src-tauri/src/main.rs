// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

mod ipc;
use ipc::command::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentSchema {
    text: Option<String>,
    icon: Option<String>,
    images: Option<Vec<String>>,
    videos: Option<Vec<String>>,
    inline_images: Option<Vec<String>>,
    color: Option<String>,
    text_stroke: Option<String>,
}

impl CommentSchema {
    pub fn to_comment_text(&self) -> String {
        let mut comment = String::new();
        if let Some(icon) = self.icon.as_ref() {
            comment += icon;
            comment += "##ICON##";
        }

        if let Some(color) = self.color.as_ref() {
            comment += color;
            comment += "##COLOR##";
        }

        if let Some(text_stroke) = self.text_stroke.as_ref() {
            comment += text_stroke;
            comment += "##TEXT_STROKE##";
        }

        let mut text = match self.text.as_ref() {
            Some(text) => text.to_string(),
            None => "".to_string(),
        };

        if let Some(inline_images) = self.inline_images.as_ref() {
            for inline_image in inline_images.iter() {
                text = text.replacen(
                    "##INLINE##",
                    &format!("##INLINE_IMG##{}##INLINE_IMG##", inline_image),
                    1,
                );
            }
        }

        comment += &text;

        if let Some(images) = self.images.as_ref() {
            for image in images.iter() {
                comment += "##IMG##";
                comment += image;
            }
        }

        if let Some(videos) = self.videos.as_ref() {
            for video in videos.iter() {
                comment += "##VIDEO##";
                comment += video;
            }
        }

        comment
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Comment {
    id: usize,
    text: String,
    offset_top_ratio: f32,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct RendererInfo {
    window_index: usize,
    num_displays: usize,
    is_single_window: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct OnCommentReceivedPayload {
    comment: Comment,
    renderer_info: RendererInfo,
}

async fn process_request(stream: &mut TcpStream, _addr: SocketAddr) -> Result<String> {
    let mut received = Vec::with_capacity(1024);
    let mut buf = vec![0; 1024];

    loop {
        match stream.read(&mut buf).await {
            Ok(0) => {
                let text = String::from_utf8_lossy(&received);
                println!("{}", text);
                return Ok(text.to_string());
            }
            Ok(n) => received.append(&mut buf[0..n].to_vec()),
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return Err(e.into());
            }
        };
    }
}

fn main() -> Result<()> {
    let bind_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:2525".to_string());

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = Arc::new(app.handle());

            tauri::async_runtime::spawn(async move {
                let listener = TcpListener::bind(bind_addr).await?;
                loop {
                    let (mut socket, addr) = listener.accept().await?;
                    let app_handle = app_handle.clone();

                    tauri::async_runtime::spawn(async move {
                        let mut text = process_request(&mut socket, addr).await?;
                        if let Ok(deserialized) = serde_json::from_str::<CommentSchema>(&text) {
                            text = deserialized.to_comment_text();
                        }
                        app_handle.emit_all(
                            "comment",
                            OnCommentReceivedPayload {
                                comment: Comment {
                                    id: 0,
                                    text,
                                    offset_top_ratio: rand::thread_rng().gen_range(0.0..1.0) * 0.9,
                                },
                                renderer_info: RendererInfo {
                                    window_index: 0,
                                    num_displays: 1,
                                    is_single_window: true,
                                },
                            },
                        )?;
                        anyhow::Ok(())
                    });
                }

                #[allow(unreachable_code)]
                anyhow::Ok(())
            });

            app.listen_global("comment-arrived-to-left-edge", |event| {
                // TODO
                println!(
                    "got comment-arrived-to-left-edge with payload {:?}",
                    event.payload()
                );
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            request_duration,
            request_default_duration,
            request_text_color_style,
            request_text_stroke_style,
            request_newline_enabled,
            request_icon_enabled,
            request_inline_img_enabled,
            request_img_enabled,
            request_video_enabled,
            request_round_icon_enabled,
        ])
        .on_page_load(|window, _payload| {
            let _ = window.set_ignore_cursor_events(true);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
