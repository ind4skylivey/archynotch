use iced::{application, time, widget::image, window, Element, Subscription, Task, Theme};
use std::time::Duration;

use crate::{
    audio::AudioCapture, config::Config, mpris::MprisClient, ui::overlay::OverlayView,
    visualizers::VisualizerType,
};

pub struct ArchyNotch {
    #[allow(dead_code)]
    config: Config,
    mpris_client: MprisClient,
    #[allow(dead_code)]
    audio_capture: Option<AudioCapture>,

    // State
    is_expanded: bool,
    current_track: Option<TrackMetadata>,
    current_cover: Option<image::Handle>,
    playback_status: PlaybackStatus,
    audio_samples: Vec<f32>,
    visualizer: VisualizerType,
}

#[derive(Debug, Clone)]
pub enum Message {
    MetadataChanged(TrackMetadata),
    PlaybackStatusChanged(PlaybackStatus),
    CoverLoaded(Option<image::Handle>),
    UpdatePoll(Option<(TrackMetadata, PlaybackStatus)>),
    TogglePlayPause,
    NextTrack,
    PreviousTrack,
    ExpandToggle,
    Tick,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    #[allow(dead_code)]
    pub album: String,
    pub cover_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

impl ArchyNotch {
    pub fn new(config: Config) -> (Self, Task<Message>) {
        let mpris_client = MprisClient::new().expect("Failed to create MPRIS client");
        let audio_capture = AudioCapture::new().ok();

        let app = Self {
            config,
            mpris_client,
            audio_capture,
            is_expanded: false,
            current_track: None,
            current_cover: None,
            playback_status: PlaybackStatus::Stopped,
            audio_samples: Vec::new(),
            visualizer: VisualizerType::Spectrum,
        };

        (app, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TogglePlayPause => {
                return Task::perform(perform_mpris_action(MprisAction::Toggle), |_| Message::Tick);
            }
            Message::NextTrack => {
                return Task::perform(perform_mpris_action(MprisAction::Next), |_| Message::Tick);
            }
            Message::PreviousTrack => {
                return Task::perform(perform_mpris_action(MprisAction::Previous), |_| {
                    Message::Tick
                });
            }
            Message::ExpandToggle => {
                self.is_expanded = !self.is_expanded;
            }
            Message::MetadataChanged(metadata) => {
                let url_changed = match (&self.current_track, &metadata.cover_url) {
                    (Some(current), Some(new)) => current.cover_url.as_ref() != Some(new),
                    (None, Some(_)) => true,
                    _ => false,
                };
                self.current_track = Some(metadata.clone());
                if url_changed {
                    if let Some(url) = metadata.cover_url {
                        return Task::perform(load_cover(url), Message::CoverLoaded);
                    }
                }
            }
            Message::CoverLoaded(handle) => {
                self.current_cover = handle;
            }
            Message::PlaybackStatusChanged(status) => {
                self.playback_status = status;
            }
            Message::Tick => {
                return Task::perform(poll_media_state(), Message::UpdatePoll);
            }
            Message::UpdatePoll(Some((metadata, status))) => {
                let mut commands = Vec::new();
                let track_changed = match &self.current_track {
                    Some(current) => {
                        current.title != metadata.title || current.artist != metadata.artist
                    }
                    None => true,
                };
                if track_changed {
                    commands.push(Task::perform(
                        async move { metadata },
                        Message::MetadataChanged,
                    ));
                }
                if status != self.playback_status {
                    commands.push(Task::perform(
                        async move { status },
                        Message::PlaybackStatusChanged,
                    ));
                }
                return Task::batch(commands);
            }
            _ => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        OverlayView::new(
            &self.current_track,
            &self.current_cover,
            &self.audio_samples,
            self.visualizer,
            self.is_expanded,
            &self.playback_status,
        )
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(500)).map(|_| Message::Tick)
    }

    pub fn run(config: Config) -> iced::Result {
        let (width, height) = (config.window.width as f32, config.window.height as f32);
        application("archynotch", ArchyNotch::update, ArchyNotch::view)
            .subscription(ArchyNotch::subscription)
            .theme(|_| Theme::Dark)
            .style(|_theme, _status| application::Appearance {
                background_color: iced::Color::TRANSPARENT,
                text_color: iced::Color::WHITE,
            })
            .window(window::Settings {
                size: (width, height).into(),
                position: window::Position::Centered,
                resizable: false,
                decorations: false,
                transparent: true,
                level: window::Level::AlwaysOnTop,
                ..Default::default()
            })
            .run_with(move || ArchyNotch::new(config))
    }
}

#[derive(Debug, Clone, Copy)]
enum MprisAction {
    Toggle,
    Next,
    Previous,
}

async fn perform_mpris_action(action: MprisAction) {
    let _ = tokio::task::spawn_blocking(move || {
        let finder = mpris::PlayerFinder::new().ok()?;
        let player = finder.find_active().ok()?;
        match action {
            MprisAction::Toggle => player.play_pause().ok(),
            MprisAction::Next => player.next().ok(),
            MprisAction::Previous => player.previous().ok(),
        }
    })
    .await;
}

async fn poll_media_state() -> Option<(TrackMetadata, PlaybackStatus)> {
    tokio::task::spawn_blocking(|| {
        let finder = mpris::PlayerFinder::new().ok()?;
        let player = finder.find_active().ok()?;
        let m = player.get_metadata().ok()?;
        let s = match player.get_playback_status().ok()? {
            mpris::PlaybackStatus::Playing => PlaybackStatus::Playing,
            mpris::PlaybackStatus::Paused => PlaybackStatus::Paused,
            _ => PlaybackStatus::Stopped,
        };
        Some((
            TrackMetadata {
                title: m.title().unwrap_or("Unknown").to_string(),
                artist: m
                    .artists()
                    .unwrap_or_default()
                    .first()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "Unknown".to_string()),
                album: m.album_name().unwrap_or("").to_string(),
                cover_url: m.art_url().map(|u| u.to_string()),
            },
            s,
        ))
    })
    .await
    .ok()
    .flatten()
}

async fn load_cover(url: String) -> Option<image::Handle> {
    if url.starts_with("file://") {
        Some(image::Handle::from_path(url.trim_start_matches("file://")))
    } else if url.starts_with("http") {
        let bytes = reqwest::get(&url).await.ok()?.bytes().await.ok()?;
        Some(image::Handle::from_bytes(bytes))
    } else {
        None
    }
}
