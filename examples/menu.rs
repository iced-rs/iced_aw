// This example demonstrates how to use the menu widget

use iced::border::Radius;
use iced::widget::{
    button, checkbox, column as col, container, pick_list, row, scrollable, slider, space, text,
    text_input, toggler, tooltip, vertical_slider, Space,
};
use iced::{alignment, theme, Border, Color, Element, Length, Padding, Size, Theme};

use iced_aw::menu::{self, Item, Menu};
use iced_aw::style::{menu_bar::primary, Status};
use iced_aw::{iced_aw_font, menu_bar, menu_items, ICED_AW_FONT_BYTES};
use iced_aw::{quad, widgets::InnerBounds};

#[cfg(feature = "debug_log")]
use log::debug;

pub fn main() -> iced::Result {
    #[cfg(feature = "debug_log")]
    env_logger::init();

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
    ToggleCloseOnClick(bool),
    FruitSelected(Fruit),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Orange,
    Strawberry,
    Tomato,
    Banana,
    Cherry,
    Pear,
    Pineapple,
    Mango,
    Kiwi,
    Grape,
    Watermelon,
    Melon,
}
impl Fruit {
    const ALL: [Fruit; 13] = [
        Fruit::Apple,
        Fruit::Orange,
        Fruit::Strawberry,
        Fruit::Tomato,
        Fruit::Banana,
        Fruit::Cherry,
        Fruit::Pear,
        Fruit::Pineapple,
        Fruit::Mango,
        Fruit::Kiwi,
        Fruit::Grape,
        Fruit::Watermelon,
        Fruit::Melon,
    ];
}
impl ToString for Fruit {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

struct App {
    title: String,
    value: u8,
    check: bool,
    toggle: bool,
    theme: iced::Theme,
    dark_mode: bool,
    close_on_click: bool,
    text: String,
    fruit: Fruit,
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
            close_on_click: true,
            text: "Text Input".into(),
            fruit: Fruit::Apple,
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
            Message::ToggleCloseOnClick(b) => {
                self.close_on_click = b;
                self.title = b.to_string();
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
            Message::FruitSelected(f) => {
                self.fruit = f;
                self.title = format!("{:?}", f);
            }
            Message::None => {}
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        #[cfg(feature = "debug_log")]
        debug!(target:"App::view", "App | view");

        let menu_tpl_1 = |items| Menu::new(items).width(180.0).offset(15.0).spacing(5.0);
        let menu_tpl_2 = |items| Menu::new(items).width(180.0).offset(0.0).spacing(5.0);
        let hold_item = |widget| Item::new(widget).close_on_click(false);
        let hold_item_wm = |widget, menu| Item::with_menu(widget, menu).close_on_click(false);
        // let coc_item = |widget| Item::new(widget).close_on_click(true);
        // let coc_item_wm = |widget, menu| Item::with_menu(widget, menu).close_on_click(true);

        #[rustfmt::skip]
        let mb = menu_bar!(
            (debug_button_s("Nested Menus"), {
                let sub5 = menu_tpl_2(menu_items!(
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                ));

                let sub4 = menu_tpl_2(menu_items!(
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                )).width(200.0);

                let sub3 = menu_tpl_2(menu_items!(
                    (debug_button_f("You can")),
                    (debug_button_f("nest menus")),
                    (submenu_button("SUB"), sub4),
                    (debug_button_f("how ever")),
                    (debug_button_f("You like")),
                    (submenu_button("SUB"), sub5),
                )).width(180.0);

                let sub2 = menu_tpl_2(menu_items!(
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (submenu_button("More sub menus"), sub3),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                )).width(160.0);

                let sub1 = menu_tpl_2(menu_items!(
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (submenu_button("Another sub menu"), sub2),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                )).width(220.0);

                menu_tpl_1(menu_items!(
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (submenu_button("A sub menu"), sub1),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                )).width(140.0)
            }),
            (debug_button_s("Widgets"), menu_tpl_1(menu_items!(
                (debug_button_f("You can use any widget")),
                (debug_button_f("as a menu item")),
                (button(
                    text("Button")
                        .width(Length::Fill)
                        .align_y(alignment::Vertical::Center),
                    )
                    .width(Length::Fill)
                    .on_press(Message::Debug("Button".into()))
                ),
                (
                    row![
                        "Checkbox",
                        checkbox(self.check).on_toggle(Message::CheckChange).width(Length::Fill)
                    ]
                ),
                (
                    row![
                        "Slider",
                        space::horizontal().width(Length::Fixed(8.0)),
                        slider(0..=255, self.value, Message::ValueChange)
                    ]
                ),
                (text_input("", &self.text).on_input(Message::TextChange)),
                (
                    container(toggler(self.toggle)
                        .label("Or as a sub menu item".to_string())
                        .on_toggle(Message::ToggleChange)
                    )
                    .padding([0, 8])
                    .height(30.0)
                    .width(Length::Fill)
                    .align_y(alignment::Vertical::Center),
                    menu_tpl_2(menu_items!(
                        (debug_button_f("Item")),
                        (debug_button_f("Item")),
                        (debug_button_f("Item")),
                        (debug_button_f("Item")),
                    ))
                ),
                (debug_button_f("Separator")),
                (separator()),
                (debug_button_f("Labeled Separator")),
                (labeled_separator("Separator")),
                (debug_button_f("Dot Separator")),
                (dot_separator(&self.theme)),
                (debug_button_f("Item")),
                (debug_button_f("Item")),
            )).width(240.0)),
            (debug_button_s("Controls"), menu_tpl_1(menu_items!(
                (row![toggle(
                        "Close On Click Global",
                        self.close_on_click,
                        Message::ToggleCloseOnClick
                    )
                    ].padding([0, 8])
                ),
                (row![toggle(
                        "Dark Mode",
                        self.dark_mode,
                        Message::ThemeChange
                    )
                    ].padding([0, 8])
                ),
                (color_button([0.45, 0.25, 0.57])),
                (color_button([0.15, 0.59, 0.64])),
                (color_button([0.76, 0.82, 0.20])),
                (color_button([0.17, 0.27, 0.33])),
                (debug_button_f("Primary"), {
                    let [r, g, b, _] = self.theme.palette().primary.into_rgba8();

                    menu_tpl_2(menu_items!(
                        (slider(0..=255, r, move |x| {
                            Message::ColorChange(Color::from_rgb8(x, g, b))
                        })),
                        (slider(0..=255, g, move |x| {
                            Message::ColorChange(Color::from_rgb8(r, x, b))
                        })),
                        (slider(0..=255, b, move |x| {
                            Message::ColorChange(Color::from_rgb8(r, g, x))
                        })),
                    ))
                }),
            )).width(220.0)),
            (debug_button_s("Scroll"), menu_tpl_1(menu_items!(
                (debug_button_f("ajrs")),
                (debug_button_f("bsdfho")),
                (debug_button_f("clkjhbf")),
                (debug_button_f("dekjdaud")),
                (debug_button_f("ecsh")),
                (debug_button_f("fweiu")),
                (debug_button_f("giwe")),
                (debug_button_f("heruyv")),
                (debug_button_f("isabe")),
                (submenu_button("jcsu"), menu_tpl_2(menu_items!(
                    (debug_button_f("ajrs")),
                    (debug_button_f("bsdfho")),
                    (debug_button_f("clkjhbf")),
                    (debug_button_f("dekjdaud")),
                    (debug_button_f("ecsh")),
                    (debug_button_f("fweiu")),
                    (debug_button_f("giwe")),
                    (debug_button_f("heruyv")),
                    (debug_button_f("isabe")),
                    (debug_button_f("jcsu")),
                    (debug_button_f("kaljkahd")),
                    (submenu_button("luyortp"), menu_tpl_2(menu_items!(
                        (debug_button_f("ajrs")),
                        (debug_button_f("bsdfho")),
                        (debug_button_f("clkjhbf")),
                        (debug_button_f("dekjdaud")),
                        (debug_button_f("ecsh")),
                        (debug_button_f("fweiu")),
                        (debug_button_f("giwe")),
                        (debug_button_f("heruyv")),
                        (debug_button_f("isabe")),
                        (debug_button_f("jcsu")),
                        (debug_button_f("kaljkahd")),
                        (debug_button_f("luyortp")),
                        (debug_button_f("mmdyrc")),
                        (debug_button_f("nquc")),
                    ))),
                    (debug_button_f("mmdyrc")),
                    (debug_button_f("nquc")),
                ))),
                (debug_button_f("kaljkahd")),
                (debug_button_f("luyortp")),
                (debug_button_f("mmdyrc")),
                (debug_button_f("nquc")),
                (debug_button_f("ajrs")),
                (debug_button_f("bsdfho")),
                (debug_button_f("clkjhbf")),
                (debug_button_f("dekjdaud")),
                (debug_button_f("ecsh")),
                (debug_button_f("fweiu")),
                (debug_button_f("giwe")),
                (debug_button_f("heruyv")),
                (debug_button_f("isabe")),
                (debug_button_f("jcsu")),
                (debug_button_f("kaljkahd")),
                (debug_button_f("luyortp")),
                (debug_button_f("mmdyrc")),
                (debug_button_f("nquc")),
                (debug_button_f("ajrs")),
                (debug_button_f("bsdfho")),
                (debug_button_f("clkjhbf")),
                (submenu_button("dekjdaud"), menu_tpl_2(menu_items!(
                    (debug_button_f("ajrs")),
                    (debug_button_f("bsdfho")),
                    (debug_button_f("clkjhbf")),
                    (debug_button_f("dekjdaud")),
                    (debug_button_f("ecsh")),
                    (debug_button_f("fweiu")),
                    (debug_button_f("giwe")),
                    (debug_button_f("heruyv")),
                    (debug_button_f("isabe")),
                    (debug_button_f("jcsu")),
                    (debug_button_f("kaljkahd")),
                    (debug_button_f("luyortp")),
                    (debug_button_f("mmdyrc")),
                    (debug_button_f("nquc")),
                    (debug_button_f("ajrs")),
                    (debug_button_f("bsdfho")),
                    (debug_button_f("clkjhbf")),
                    (debug_button_f("dekjdaud")),
                    (debug_button_f("ecsh")),
                    (debug_button_f("fweiu")),
                    (debug_button_f("giwe")),
                    (debug_button_f("heruyv")),
                    (debug_button_f("isabe")),
                    (debug_button_f("jcsu")),
                    (debug_button_f("kaljkahd")),
                    (debug_button_f("luyortp")),
                    (debug_button_f("mmdyrc")),
                    (debug_button_f("nquc")),
                ))),
                (debug_button_f("ecsh")),
                (debug_button_f("fweiu")),
                (debug_button_f("giwe")),
                (debug_button_f("heruyv")),
                (debug_button_f("isabe")),
                (debug_button_f("jcsu")),
                (debug_button_f("kaljkahd")),
                (debug_button_f("luyortp")),
                (debug_button_f("mmdyrc")),
                (debug_button_f("nquc")),
                (debug_button_f("ajrs")),
                (debug_button_f("bsdfho")),
                (debug_button_f("clkjhbf")),
                (debug_button_f("dekjdaud")),
                (debug_button_f("ecsh")),
                (debug_button_f("fweiu")),
                (debug_button_f("giwe")),
                (debug_button_f("heruyv")),
                (debug_button_f("isabe")),
                (debug_button_f("jcsu")),
                (debug_button_f("kaljkahd")),
                (debug_button_f("luyortp")),
                (debug_button_f("mmdyrc")),
                (debug_button_f("nquc")),
            ))),
            (debug_button_s("Dynamic height"), {
                let slider_count = 3;
                let slider_width = 30;
                let spacing = 5;
                let pad = 20;
                let [r, g, b, _] = self.theme.palette().primary.into_rgba8();

                menu_tpl_1(menu_items!(
                    (labeled_separator("Primary")),
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
                    ),
                    (separator()),
                    (debug_button("AABB", Some(Length::Fill), Some(Length::Fixed(40.0)))),
                    (debug_button("CCDD", Some(Length::Fill), Some(Length::Fixed(140.0)))),
                    (debug_button("EEFF", Some(Length::Fill), Some(Length::Fixed(30.0)))),
                    (debug_button("GGHH", Some(Length::Fill), Some(Length::Fixed(100.0)))),
                    (debug_button("IIJJ", Some(Length::Fill), Some(Length::Fixed(60.0)))),
                    (debug_button("KKLL", Some(Length::Fill), Some(Length::Fixed(120.0)))),
                    (debug_button("MMNN", Some(Length::Fill), Some(Length::Fixed(50.0)))),
                )).width(slider_width * slider_count + (slider_count - 1) * spacing + pad)
            }),
            (debug_button_s("Overlay"), {
                let sub2 = menu_tpl_2(menu_items!(
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                ));

                let fruit_button = |fruit: Fruit | {
                    let r = if self.fruit == fruit {
                        row![
                            text(format!("{fruit:?}")).width(Length::Fill),
                            quad::Quad{
                                width: Length::Fixed(16.0),
                                height: Length::Fixed(8.0),
                                inner_bounds: InnerBounds::Square(8.0),
                                quad_color: Color::from([0.7; 3]).into(),
                                quad_border: Border{
                                    radius: Radius::new(8),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        ]
                    } else {
                        row![
                            text(format!("{fruit:?}")).width(Length::Fill),
                        ]
                    }.align_y(iced::Alignment::Center);

                    tooltip_button(
                        format!("{fruit:?}"),
                        r,
                        Some(Length::Fill),
                        None,
                        Message::FruitSelected(fruit)
                    )
                };

                let fruit_menu = menu_tpl_2(Fruit::ALL.iter()
                    .map(|fruit|
                        Item::new(fruit_button(*fruit))
                    ).collect()
                )
                .close_on_item_click(false)
                .close_on_background_click(false);

                let sub1 = menu_tpl_2(menu_items!(
                    // (
                    //     container(
                    //         text("This menu is set to always close on click\n \
                    //         Except for the pick lists\
                    //         ")
                    //         .size(12)
                    //         .color(Color::from([0.5; 3]))
                    //     )
                    //     .padding([4, 4])
                    //     .align_y(alignment::Alignment::Center)
                    //     .align_x(alignment::Alignment::Center)
                    // ),

                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    hold_item(row![
                        text("Pick Fruit")
                            .width(Length::Fill)
                            .align_y(alignment::Vertical::Center),
                        pick_list(
                            Fruit::ALL,
                            Some(self.fruit),
                            Message::FruitSelected
                        ),
                    ].padding([0, 8])
                    .align_y(iced::Alignment::Center)),
                    (debug_button_f("Item")),
                    (debug_button_f("Item")),
                    (
                        tooltip_button(
                            format!("{:?}", self.fruit),
                            row![
                                text("Pick Fruit")
                                    .width(Length::Fill)
                                    .align_y(alignment::Vertical::Center),
                                text(format!("{:?}", self.fruit))
                                    .align_y(alignment::Vertical::Center)
                                    .color(Color::from([0.5; 3])),
                            ].align_y(iced::Alignment::Center),
                            Some(Length::Fill),
                            None,
                            Message::Debug(format!("{:?}", self.fruit))
                        ),
                        fruit_menu
                    ),
                    (debug_button_f("Item")),
                    hold_item_wm(
                        tooltip_button(
                            "Pick Fruit".to_string(),
                            row![
                                text("Pick Fruit")
                                    .width(Length::Fill)
                                    .align_y(alignment::Vertical::Center),
                                pick_list(
                                    Fruit::ALL,
                                    Some(self.fruit),
                                    Message::FruitSelected
                                ),
                            ].align_y(iced::Alignment::Center),
                            Some(Length::Fill),
                            Some(Length::Shrink),
                            Message::Debug(format!("{:?}", self.fruit))
                        ),
                        sub2
                    ),
                    (debug_button_f("Item"))
                )).width(240.0).close_on_item_click(true);

                menu_tpl_1(menu_items!(
                    (submenu_button("A sub menu"), sub1)
                )).width(140.0)
            }),
            // (horizontal_space().width(Length::Fill)),
        )
        .draw_path(menu::DrawPath::Backdrop)
        .close_on_item_click_global(self.close_on_click)
        .close_on_background_click_global(self.close_on_click)
        // .safe_bounds_margin(f32::MAX)
        .padding(Padding::new(5.0))
        // .width(Length::Fixed(400.0))
        // .width(Length::Fill)
        // .width(Length::Shrink)
        .style(|theme:&iced::Theme, status: Status | menu::Style{
            path_border: Border{
                radius: Radius::new(6.0),
                ..Default::default()
            },
            path: Color::from_rgb(
                theme.extended_palette().primary.weak.color.r * 1.2,
                theme.extended_palette().primary.weak.color.g * 1.2,
                theme.extended_palette().primary.weak.color.b * 1.2,
            ).into(),
            ..primary(theme, status)
        });

        let r = row![
            space::horizontal().width(350),
            mb,
            space::horizontal().width(350),
        ]
        .align_y(alignment::Alignment::Center);

        let c = col![
            space::vertical().height(500),
            r,
            space::vertical().height(700),
        ];

        let sc = scrollable(c)
            // .direction(scrollable::Direction::Vertical(
            //     scrollable::Scrollbar::default()
            // ));
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::default(),
                horizontal: scrollable::Scrollbar::default(),
            });
        /*.direction(scrollable::Direction::Both {
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
        .style(|theme, status| {
            use iced_widget::button::{Status, Style};

            let palette = theme.extended_palette();
            let base = Style {
                text_color: palette.background.base.text,
                border: Border::default().rounded(6.0),
                ..Style::default()
            };
            match status {
                Status::Active => base.with_background(Color::TRANSPARENT),
                Status::Hovered => base.with_background(Color::from_rgb(
                    palette.primary.weak.color.r * 1.2,
                    palette.primary.weak.color.g * 1.2,
                    palette.primary.weak.color.b * 1.2,
                )),
                Status::Disabled => base.with_background(Color::from_rgb(0.5, 0.5, 0.5)),
                Status::Pressed => base.with_background(palette.primary.weak.color),
                // Status::Disabled => base.with_background(Color::from_rgb(1.0, 0.0, 0.0)),
                // Status::Pressed => base.with_background(Color::from_rgb(0.0, 1.0, 0.0)),
                // _ => iced::widget::button::primary(theme, status)
            }
        })
        .on_press(msg)
}

fn build_tooltip<'a>(
    label: String,
    content: impl Into<Element<'a, Message, iced::Theme, iced::Renderer>>,
) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    tooltip(
        content,
        container(text(label).color(Color::WHITE))
            .padding(10)
            .style(|theme| {
                container::rounded_box(theme)
                    .border(Border::default().rounded(8.0))
                    .background(Color::from_rgb(0.2, 0.2, 0.2))
            }),
        tooltip::Position::Bottom,
    )
    .into()
}

fn tooltip_button<'a>(
    label: String,
    content: impl Into<Element<'a, Message, iced::Theme, iced::Renderer>>,
    width: Option<Length>,
    height: Option<Length>,
    msg: Message,
) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    build_tooltip(
        label,
        base_button(content, msg)
            .width(width.unwrap_or(Length::Shrink))
            .height(height.unwrap_or(Length::Shrink)),
    )
}

fn debug_button(
    label: &str,
    width: Option<Length>,
    height: Option<Length>,
) -> Element<'_, Message, iced::Theme, iced::Renderer> {
    tooltip_button(
        label.to_string(),
        text(label)
            .height(height.unwrap_or(Length::Shrink))
            .align_y(alignment::Vertical::Center),
        width,
        height,
        Message::Debug(label.into()),
    )
}

fn debug_button_s(label: &str) -> Element<'_, Message, iced::Theme, iced::Renderer> {
    debug_button(label, Some(Length::Shrink), Some(Length::Shrink)).into()
}

fn debug_button_f(label: &str) -> Element<'_, Message, iced::Theme, iced::Renderer> {
    debug_button(label, Some(Length::Fill), Some(Length::Shrink)).into()
}

fn submenu_button(label: &str) -> Element<'_, Message, iced::Theme, iced::Renderer> {
    tooltip_button(
        label.to_string(),
        row![
            text(label)
                .width(Length::Fill)
                .align_y(alignment::Vertical::Center),
            iced_aw_font::right_open()
                .width(Length::Shrink)
                .align_y(alignment::Vertical::Center),
        ]
        .align_y(iced::Alignment::Center),
        Some(Length::Fill),
        None,
        Message::Debug(label.into()),
    )
}

fn color_button<'a>(color: impl Into<Color>) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    let color = color.into();
    // base_button(circle(color), Message::ColorChange(color))
    tooltip_button(
        format!("{color:?}"),
        circle(color),
        Some(Length::Fill),
        None,
        Message::ColorChange(color),
    )
    .into()
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

fn toggle<'a>(
    label: &'a str,
    is_checked: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    row![
        text(label).align_y(alignment::Vertical::Center),
        Space::new().width(Length::Fill).height(Length::Shrink),
        toggler(is_checked).size(20.0).on_toggle(on_toggle),
    ]
    .align_y(alignment::Alignment::Center)
    .into()
}
