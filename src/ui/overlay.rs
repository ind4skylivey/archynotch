use crate::app::{Message, PlaybackStatus, TrackMetadata};
use crate::visualizers::VisualizerType;
use iced::{
    widget::{button, column, container, horizontal_space, image, row, text},
    Alignment, Background, Border, Color, Element, Length, Shadow,
};

const CYBER_BG: Color = Color::from_rgb(0.02, 0.02, 0.05);
const NEON_CYAN: Color = Color::from_rgb(0.0, 0.95, 1.0);
const NEON_PINK: Color = Color::from_rgb(1.0, 0.0, 0.5);
const DIM_TEXT: Color = Color::from_rgb(0.6, 0.6, 0.7);

pub struct OverlayView<'a> {
    track: &'a Option<TrackMetadata>,
    cover_image: &'a Option<image::Handle>,
    #[allow(dead_code)]
    audio_samples: &'a [f32],
    #[allow(dead_code)]
    visualizer_type: VisualizerType,
    is_expanded: bool,
    playback_status: &'a PlaybackStatus,
}

impl<'a> OverlayView<'a> {
    pub fn new(
        track: &'a Option<TrackMetadata>,
        cover_image: &'a Option<image::Handle>,
        audio_samples: &'a [f32],
        visualizer_type: VisualizerType,
        is_expanded: bool,
        playback_status: &'a PlaybackStatus,
    ) -> Self {
        Self {
            track,
            cover_image,
            audio_samples,
            visualizer_type,
            is_expanded,
            playback_status,
        }
    }
}

impl<'a> Into<Element<'a, Message>> for OverlayView<'a> {
    fn into(self) -> Element<'a, Message> {
        let content = if let Some(track) = self.track {
            let cover: Element<Message> = if let Some(handle) = self.cover_image {
                container(
                    image(handle.clone())
                        .width(40)
                        .height(40)
                        .content_fit(iced::ContentFit::Cover),
                )
                .style(|_| container::Style {
                    border: Border {
                        color: NEON_CYAN,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .into()
            } else {
                container(text("DISK").size(10).color(NEON_CYAN))
                    .width(40)
                    .height(40)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgb(0.05, 0.05, 0.1))),
                        border: Border {
                            color: NEON_CYAN,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    })
                    .into()
            };

            let info = column![
                text(&track.title).size(13).color(NEON_CYAN),
                text(track.artist.to_uppercase()).size(10).color(DIM_TEXT)
            ]
            .width(Length::Shrink);

            if self.is_expanded {
                let play_pause_icon = match self.playback_status {
                    PlaybackStatus::Playing => "||",
                    _ => "|>",
                };
                let controls = row![
                    button(text("<<").size(14).color(NEON_PINK))
                        .on_press(Message::PreviousTrack)
                        .style(button::text),
                    button(text(play_pause_icon).size(14).color(Color::WHITE))
                        .on_press(Message::TogglePlayPause)
                        .style(button::text),
                    button(text(">>").size(14).color(NEON_PINK))
                        .on_press(Message::NextTrack)
                        .style(button::text),
                ]
                .spacing(5)
                .align_y(Alignment::Center);

                row![
                    controls,
                    horizontal_space().width(15),
                    cover,
                    horizontal_space().width(10),
                    info
                ]
                .align_y(Alignment::Center)
            } else {
                row![
                    text("::").color(NEON_PINK),
                    horizontal_space().width(8),
                    cover,
                    horizontal_space().width(12),
                    info
                ]
                .align_y(Alignment::Center)
            }
        } else {
            row![text("[ SYSTEM READY ]").size(12).color(DIM_TEXT)].into()
        };

        container(
            button(container(content).padding([8, 20]).center_y(Length::Shrink))
                .on_press(Message::ExpandToggle)
                .style(|_, status| {
                    let border_color = if status == button::Status::Hovered {
                        NEON_PINK
                    } else {
                        NEON_CYAN
                    };
                    button::Style {
                        background: Some(Background::Color(CYBER_BG)),
                        text_color: NEON_CYAN,
                        border: Border {
                            radius: 4.0.into(),
                            width: 2.0,
                            color: border_color,
                        },
                        shadow: Shadow {
                            color: if status == button::Status::Hovered {
                                Color::from_rgba(1.0, 0.0, 0.5, 0.4)
                            } else {
                                Color::from_rgba(0.0, 0.95, 1.0, 0.25)
                            },
                            blur_radius: 15.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .width(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
    }
}
