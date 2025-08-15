// This example demonstrates how to use the menu widget

use iced::border::Radius;
use iced::widget::{
    button, checkbox, container, horizontal_space, row, scrollable, slider, text, text_input,
    toggler, vertical_slider,
};
use iced::widget::{column as col, vertical_space};
use iced::{alignment, theme, Border, Color, Element, Length, Size, Theme};

use iced_aw::menu::{self, Menu};
use iced_aw::style::{menu_bar::primary, Status};
use iced_aw::{iced_aw_font, menu_bar, menu_items, ICED_AW_FONT_BYTES};
use iced_aw::{quad, widgets::InnerBounds};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .theme(App::theme)
        .font(ICED_AW_FONT_BYTES)
        .window_size(Size::new(1000.0, 600.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ToggleChange(bool),
    ColorChange(Color),
    ThemeChange(bool),
    TextChange(String),
    None,
}

struct App {
    title: String,
    value: u8,
    check: bool,
    toggle: bool,
    theme: iced::Theme,
    dark_mode: bool,
    text: String,
}

impl Default for App {
    fn default() -> Self {
        let theme = iced::Theme::custom(
            "Custom Theme",
            theme::Palette {
                primary: Color::from([0.45, 0.25, 0.57]),
                ..iced::Theme::Light.palette()
            },
        );

        Self {
            title: "Menu Test".to_string(),
            value: 0,
            check: false,
            toggle: false,
            theme,
            dark_mode: false,
            text: "Text Input".into(),
        }
    }
}

impl App {
    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Debug(s) => {
                self.title = s;
            }
            Message::ValueChange(v) => {
                self.value = v;
                self.title = v.to_string();
            }
            Message::CheckChange(c) => {
                self.check = c;
                self.title = c.to_string();
            }
            Message::ToggleChange(t) => {
                self.toggle = t;
                self.title = t.to_string();
            }
            Message::ColorChange(c) => {
                self.theme = iced::Theme::custom(
                    "Color Change",
                    theme::Palette {
                        primary: c,
                        ..self.theme.palette()
                    },
                );
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
            }
            Message::ThemeChange(b) => {
                self.dark_mode = b;
                let primary = self.theme.palette().primary;
                if b {
                    self.theme = iced::Theme::custom(
                        "Dark",
                        theme::Palette {
                            primary,
                            ..iced::Theme::Dark.palette()
                        },
                    )
                } else {
                    self.theme = iced::Theme::custom(
                        "Light",
                        theme::Palette {
                            primary,
                            ..iced::Theme::Light.palette()
                        },
                    )
                }
            }
            Message::TextChange(s) => {
                self.text.clone_from(&s);
                self.title = s;
            }
            Message::None => {}
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        let menu_tpl_2 = |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);

        #[rustfmt::skip]
        let mb = menu_bar!(
            (debug_button_s("Nested Menus"), {
                let sub5 = menu_tpl_2(menu_items!(
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                ));

                let sub4 = menu_tpl_2(menu_items!(
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                )).width(200.0);

                let sub3 = menu_tpl_2(menu_items!(
                    (debug_button("You can"))
                    (debug_button("nest menus"))
                    (submenu_button("SUB"), sub4)
                    (debug_button("how ever"))
                    (debug_button("You like"))
                    (submenu_button("SUB"), sub5)
                )).width(180.0);

                let sub2 = menu_tpl_2(menu_items!(
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (submenu_button("More sub menus"), sub3)
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                )).width(160.0);

                let sub1 = menu_tpl_2(menu_items!(
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (submenu_button("Another sub menu"), sub2)
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                )).width(220.0);

                menu_tpl_1(menu_items!(
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (submenu_button("A sub menu"), sub1)
                    (debug_button("Item"))
                    (debug_button("Item"))
                    (debug_button("Item"))
                )).width(140.0)
            })
            (debug_button_s("Widgets"), menu_tpl_1(menu_items!(
                (debug_button("You can use any widget"))
                (debug_button("as a menu item"))
                (button(
                    text("Button")
                        .width(Length::Fill)
                        .align_y(alignment::Vertical::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::Debug("Button".into()))
                )
                (checkbox("Checkbox", self.check).on_toggle(Message::CheckChange)
                    .width(Length::Fill)
                )
                (
                    row![
                        "Slider",
                        horizontal_space().width(Length::Fixed(8.0)),
                        slider(0..=255, self.value, Message::ValueChange)
                    ]
                )
                (text_input("", &self.text).on_input(Message::TextChange))
                (container(toggler(
                        self.toggle,
                    ).label("Or as a sub menu item".to_string()).on_toggle(Message::ToggleChange))
                    .padding([0, 8])
                    .height(30.0)
                    .align_y(alignment::Vertical::Center),
                    menu_tpl_2(menu_items!(
                        (debug_button("Item"))
                        (debug_button("Item"))
                        (debug_button("Item"))
                        (debug_button("Item"))
                    ))
                )
                (debug_button("Separator"))
                (separator())
                (debug_button("Labeled Separator"))
                (labeled_separator("Separator"))
                (debug_button("Dot Separator"))
                (dot_separator(&self.theme))
                (debug_button("Item"))
                (debug_button("Item"))
            )).width(240.0))
            (debug_button_s("Controls"), menu_tpl_1(menu_items!(
                (row![toggler(
                        self.dark_mode,
                    ).label("Dark Mode".to_string()).on_toggle(Message::ThemeChange)].padding([0, 8])
                )
                (color_button([0.45, 0.25, 0.57]))
                (color_button([0.15, 0.59, 0.64]))
                (color_button([0.76, 0.82, 0.20]))
                (color_button([0.17, 0.27, 0.33]))
                (labeled_button("Primary", Message::None)
                    .width(Length::Fill),
                    {
                        let [r, g, b, _] = self.theme.palette().primary.into_rgba8();

                        menu_tpl_2(menu_items!(
                            (slider(0..=255, r, move |x| {
                                Message::ColorChange(Color::from_rgb8(x, g, b))
                            }))
                            (slider(0..=255, g, move |x| {
                                Message::ColorChange(Color::from_rgb8(r, x, b))
                            }))
                            (slider(0..=255, b, move |x| {
                                Message::ColorChange(Color::from_rgb8(r, g, x))
                            }))
                        ))
                    }
                )
            )))
            (debug_button_s("Scroll"), menu_tpl_1(menu_items!(
                (debug_button("ajrs"))
                (debug_button("bsdfho"))
                (debug_button("clkjhbf"))
                (debug_button("dekjdaud"))
                (debug_button("ecsh"))
                (debug_button("fweiu"))
                (debug_button("giwe"))
                (debug_button("heruyv"))
                (debug_button("isabe"))
                (submenu_button("jcsu"), menu_tpl_2(menu_items!(
                    (debug_button("ajrs"))
                    (debug_button("bsdfho"))
                    (debug_button("clkjhbf"))
                    (debug_button("dekjdaud"))
                    (debug_button("ecsh"))
                    (debug_button("fweiu"))
                    (debug_button("giwe"))
                    (debug_button("heruyv"))
                    (debug_button("isabe"))
                    (debug_button("jcsu"))
                    (debug_button("kaljkahd"))
                    (submenu_button("luyortp"), menu_tpl_2(menu_items!(
                        (debug_button("ajrs"))
                        (debug_button("bsdfho"))
                        (debug_button("clkjhbf"))
                        (debug_button("dekjdaud"))
                        (debug_button("ecsh"))
                        (debug_button("fweiu"))
                        (debug_button("giwe"))
                        (debug_button("heruyv"))
                        (debug_button("isabe"))
                        (debug_button("jcsu"))
                        (debug_button("kaljkahd"))
                        (debug_button("luyortp"))
                        (debug_button("mmdyrc"))
                        (debug_button("nquc"))
                    )))
                    (debug_button("mmdyrc"))
                    (debug_button("nquc"))
                )))
                (debug_button("kaljkahd"))
                (debug_button("luyortp"))
                (debug_button("mmdyrc"))
                (debug_button("nquc"))
                (debug_button("ajrs"))
                (debug_button("bsdfho"))
                (debug_button("clkjhbf"))
                (debug_button("dekjdaud"))
                (debug_button("ecsh"))
                (debug_button("fweiu"))
                (debug_button("giwe"))
                (debug_button("heruyv"))
                (debug_button("isabe"))
                (debug_button("jcsu"))
                (debug_button("kaljkahd"))
                (debug_button("luyortp"))
                (debug_button("mmdyrc"))
                (debug_button("nquc"))
                (debug_button("ajrs"))
                (debug_button("bsdfho"))
                (debug_button("clkjhbf"))
                (submenu_button("dekjdaud"), menu_tpl_2(menu_items!(
                    (debug_button("ajrs"))
                    (debug_button("bsdfho"))
                    (debug_button("clkjhbf"))
                    (debug_button("dekjdaud"))
                    (debug_button("ecsh"))
                    (debug_button("fweiu"))
                    (debug_button("giwe"))
                    (debug_button("heruyv"))
                    (debug_button("isabe"))
                    (debug_button("jcsu"))
                    (debug_button("kaljkahd"))
                    (debug_button("luyortp"))
                    (debug_button("mmdyrc"))
                    (debug_button("nquc"))
                    (debug_button("ajrs"))
                    (debug_button("bsdfho"))
                    (debug_button("clkjhbf"))
                    (debug_button("dekjdaud"))
                    (debug_button("ecsh"))
                    (debug_button("fweiu"))
                    (debug_button("giwe"))
                    (debug_button("heruyv"))
                    (debug_button("isabe"))
                    (debug_button("jcsu"))
                    (debug_button("kaljkahd"))
                    (debug_button("luyortp"))
                    (debug_button("mmdyrc"))
                    (debug_button("nquc"))
                )))
                (debug_button("ecsh"))
                (debug_button("fweiu"))
                (debug_button("giwe"))
                (debug_button("heruyv"))
                (debug_button("isabe"))
                (debug_button("jcsu"))
                (debug_button("kaljkahd"))
                (debug_button("luyortp"))
                (debug_button("mmdyrc"))
                (debug_button("nquc"))
                (debug_button("ajrs"))
                (debug_button("bsdfho"))
                (debug_button("clkjhbf"))
                (debug_button("dekjdaud"))
                (debug_button("ecsh"))
                (debug_button("fweiu"))
                (debug_button("giwe"))
                (debug_button("heruyv"))
                (debug_button("isabe"))
                (debug_button("jcsu"))
                (debug_button("kaljkahd"))
                (debug_button("luyortp"))
                (debug_button("mmdyrc"))
                (debug_button("nquc"))
            )))
            (debug_button_s("Dynamic height"), {
                let slider_count = 3;
                let slider_width = 30;
                let spacing = 5;
                let pad = 20;
                let [r, g, b, _] = self.theme.palette().primary.into_rgba8();

                menu_tpl_1(menu_items!(
                    (labeled_separator("Primary"))
                    (
                        row![
                            vertical_slider(0..=255, r, move |x| Message::ColorChange(Color::from_rgb8(
                                x, g, b
                            )))
                            .width(slider_width)
                            ,
                            vertical_slider(0..=255, g, move |x| Message::ColorChange(Color::from_rgb8(
                                r, x, b
                            )))
                            .width(slider_width)
                            ,
                            vertical_slider(0..=255, b, move |x| Message::ColorChange(Color::from_rgb8(
                                r, g, x
                            )))
                            .width(slider_width)
                            ,
                        ].spacing(spacing)
                        .height(100.0)
                    )
                    (separator())
                    (debug_button("AABB").height(40))
                    (debug_button("CCDD").height(140))
                    (debug_button("EEFF").height(30))
                    (debug_button("GGHH").height(100))
                    (debug_button("IIJJ").height(60))
                    (debug_button("KKLL").height(120))
                    (debug_button("MMNN").height(50))
                )).width(slider_width * slider_count + (slider_count - 1) * spacing + pad)
            })
        )
        .draw_path(menu::DrawPath::Backdrop)
        .style(|theme:&iced::Theme, status: Status | menu::Style{
            path_border: Border{
                radius: Radius::new(6.0),
                ..Default::default()
            },
            ..primary(theme, status)
        });

        let r = row![
            horizontal_space().width(295),
            mb,
            horizontal_space().width(295),
        ]
        .align_y(alignment::Alignment::Center);

        let c = col![
            vertical_space().height(500),
            r,
            vertical_space().height(500),
        ];

        let sc = scrollable(c); /*.direction(scrollable::Direction::Both {
                                    vertical: scrollable::Properties::new().alignment(scrollable::Alignment::End),
                                    horizontal: scrollable::Properties::new(),
                                });*/

        fn back_style(theme: &iced::Theme) -> container::Style {
            container::Style {
                background: Some(theme.extended_palette().primary.base.color.into()),
                ..Default::default()
            }
        }
        let back = container(sc)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(back_style);

        back.into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> button::Button<'a, Message> {
    button(content)
        .padding([4, 8])
        .style(iced::widget::button::primary)
        .on_press(msg)
}

fn labeled_button(
    label: &str,
    msg: Message,
) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
    base_button(text(label).align_y(alignment::Vertical::Center), msg)
}

fn debug_button(label: &str) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into())).width(Length::Fill)
}

fn debug_button_s(label: &str) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into())).width(Length::Shrink)
}

fn submenu_button(label: &str) -> button::Button<'_, Message, iced::Theme, iced::Renderer> {
    base_button(
        row![
            text(label)
                .width(Length::Fill)
                .align_y(alignment::Vertical::Center),
            iced_aw_font::right_open()
                .width(Length::Shrink)
                .align_y(alignment::Vertical::Center),
        ]
        .align_y(iced::Alignment::Center),
        Message::Debug(label.into()),
    )
    .width(Length::Fill)
}

fn color_button<'a>(
    color: impl Into<Color>,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    let color = color.into();
    base_button(circle(color), Message::ColorChange(color))
}

fn separator() -> quad::Quad {
    quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}

fn dot_separator<'a>(theme: &iced::Theme) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    row((0..20).map(|_| {
        quad::Quad {
            quad_color: theme.extended_palette().background.base.text.into(),
            inner_bounds: InnerBounds::Square(4.0),
            ..separator()
        }
        .into()
    }))
    .height(20.0)
    .into()
}

fn labeled_separator(label: &'_ str) -> Element<'_, Message, iced::Theme, iced::Renderer> {
    let q_1 = quad::Quad {
        height: Length::Fill,
        ..separator()
    };
    let q_2 = quad::Quad {
        height: Length::Fill,
        ..separator()
    };

    row![
        q_1,
        text(label)
            .height(Length::Fill)
            .align_y(alignment::Vertical::Center),
        q_2,
    ]
    .height(20.0)
    .into()
}

fn circle(color: Color) -> quad::Quad {
    let radius = 10.0;

    quad::Quad {
        quad_color: color.into(),
        inner_bounds: InnerBounds::Square(radius * 2.0),
        quad_border: Border {
            radius: Radius::new(radius),
            ..Default::default()
        },
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}
