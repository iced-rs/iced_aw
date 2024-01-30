use iced::widget::column as col;
use iced::widget::{
    button, checkbox, container, horizontal_space, pick_list, row, slider, svg, text, text_input,
    toggler, vertical_slider,
};
use iced::{alignment, theme, Application, Border, Color, Element, Event, Length, Pixels, Size};

use iced_aw::menu::menu_bar::MenuBar;
use iced_aw::menu::{
    menu_tree::MenuTree, CloseCondition, ItemHeight, ItemWidth, PathHighlight,
    Menux, OpenCondition, Axis
};
use iced_aw::quad;
use iced_aw::{helpers::menu_tree, menu_bar, menu_tree};

pub fn main() -> iced::Result {
    // std::env::set_var("RUST_BACKTRACE", "full");
    App::run(iced::Settings {
        default_text_size: Pixels(15.0),
        window: iced::window::Settings{
            size: Size::new(1000.0, 500.0),
            ..Default::default()
        },
        // id: todo!(),
        // flags: todo!(),
        // fonts: todo!(),
        // default_font: todo!(),
        // antialiasing: todo!(),
        
        // default_text_size: 15.0,
        // window: iced::window::Settings {
        //     size: (1000, 500),
        //     // position: iced::window::Position::Default,
        //     ..Default::default()
        // },
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeOption {
    Uniform,
    Static,
    DynamicHeight,
}
impl SizeOption {
    const ALL: [SizeOption; 3] = [
        SizeOption::Uniform,
        SizeOption::Static,
        SizeOption::DynamicHeight,
    ];
}
impl std::fmt::Display for SizeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Uniform => "Uniform",
                Self::Static => "Static",
                Self::DynamicHeight => "DynamicHeight",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    Debug(String),
    ValueChange(u8),
    CheckChange(bool),
    ToggleChange(bool),
    ColorChange(Color),
    FlipHorizontal,
    FlipVertical,
    ThemeChange(bool),
    TextChange(String),
    SizeOption(SizeOption),
    None,
}

struct App {
    title: String,
    value: u8,
    check: bool,
    toggle: bool,
    theme: iced::Theme,
    flip_h: bool,
    flip_v: bool,
    dark_mode: bool,
    text: String,
    size_option: SizeOption,
}
impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let theme = iced::Theme::custom(
            "Custom Theme".into(),
            theme::Palette {
                primary: Color::from([0.45, 0.25, 0.57]),
                ..iced::Theme::Light.palette()
            }
        );

        (
            Self {
                title: "Menu Test".to_string(),
                value: 0,
                check: false,
                toggle: false,
                theme,
                flip_h: false,
                flip_v: false,
                dark_mode: false,
                text: "Text Input".into(),
                size_option: SizeOption::DynamicHeight,
            },
            iced::Command::none(),
        )
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        use iced::keyboard;
        use keyboard::key::Named;

        iced::event::listen().map(|event|{
            match event{
                Event::Keyboard(keyboard::Event::KeyPressed { key, ..}) => {
                    match key {
                        keyboard::Key::Named(Named::F1) => Message::FlipHorizontal,
                        keyboard::Key::Named(Named::F2) => Message::FlipVertical,
                        _ => Message::None
                    }
                },
                _ => Message::None
            }
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        println!("app update");
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
                    "Color Change".into(),
                    theme::Palette {
                        primary: c,
                        ..self.theme.palette()
                    }
                );
                self.title = format!("[{:.2}, {:.2}, {:.2}]", c.r, c.g, c.b);
            }
            Message::FlipHorizontal => self.flip_h = !self.flip_h,
            Message::FlipVertical => self.flip_v = !self.flip_v,
            Message::ThemeChange(b) => {
                self.dark_mode = b;
                let primary = self.theme.palette().primary;
                if b {
                    self.theme = iced::Theme::custom(
                        "Dark".into(),
                        theme::Palette {
                            primary,
                            ..iced::Theme::Dark.palette()
                        }
                    )
                } else {
                    self.theme = iced::Theme::custom(
                        "Light".into(),
                        theme::Palette {
                            primary,
                            ..iced::Theme::Light.palette()
                        }
                    )
                }
            }
            Message::TextChange(s) => {
                self.text = s.clone();
                self.title = s;
            }
            Message::SizeOption(so) => {
                self.size_option = so;
                self.title = self.size_option.to_string();
            }
            Message::None => {}
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Theme, iced::Renderer> {
        println!("app view");
        let pick_size_option = pick_list(
            &SizeOption::ALL[..],
            Some(self.size_option),
            Message::SizeOption,
        );

        let mb = row![
            Menux::new(button("content").on_press(Message::Debug("content".into())).into(), vec![
                debug_button("abc").width(180.0).into(),
                debug_button("def").width(180.0).into(),
                debug_button("xxx").width(180.0).into(),
                Menux::new(debug_button("htrsth").width(180.0).into(), vec![
                    debug_button("ccgh").width(180.0).into(),
                    debug_button("kuyg").width(180.0).into(),
                    debug_button("vcsa").width(180.0).into(),
                    debug_button("kiug").width(180.0).into(),
                ]).axis(Axis::Horizontal).into()
            ]).open_condition(OpenCondition::Click),
            Menux::new(debug_button("aaa").into(), vec![
                debug_button("abc").width(180.0).into(),
                debug_button("def").width(180.0).into(),
                debug_button("xxx").width(180.0).into(),
                Menux::new(debug_button("syjdtyjd").width(180.0).into(), vec![
                    debug_button("hghg").width(180.0).into(),
                    debug_button("kuyg").width(180.0).into(),
                    debug_button("arga").width(180.0).into(),
                    debug_button("abcd").width(180.0).into(),
                    debug_button("vcsa").width(180.0).into(),
                    Menux::new(debug_button("htrsthfs").width(180.0).into(), vec![
                        debug_button("hghg").width(180.0).into(),
                        debug_button("kuyg").width(180.0).into(),
                        debug_button("vcsa").width(180.0).into(),
                        debug_button("kiug").width(180.0).into(),
                    ]).axis(Axis::Horizontal).into(),
                    debug_button("kiug").width(180.0).into(),
                ]).axis(Axis::Horizontal).into(),
                debug_button("abc").width(180.0).into(),
                debug_button("def").width(180.0).into(),
                debug_button("xxx").width(180.0).into(),
            ]).open_condition(OpenCondition::Click),
            Menux::new(debug_button("pondjssbah").into(), vec![
                debug_button("abc").width(180.0).into(),
                debug_button("def").width(180.0).into(),
                debug_button("xxx").width(180.0).into(),
                debug_button("htrsrt").width(180.0).into(),
                debug_button("htrdf").width(180.0).into(),
                debug_button("ngfcgng").width(180.0).into(),
                debug_button("hytfy").width(180.0).into(),
                debug_button("kuyg").width(180.0).into(),
                debug_button("qegvd").width(180.0).into(),
                debug_button("iuoiy").width(180.0).into(),
                debug_button("rzsajf").width(180.0).into(),
                debug_button("pkmehs").width(180.0).into(),
                debug_button("ivrye").width(180.0).into(),
                debug_button("zhdkr").width(180.0).into(),
                debug_button("vjdiwo").width(180.0).into(),
                Menux::new(debug_button("syjdtyjd").width(180.0).into(), vec![
                    debug_button("hghg").width(180.0).into(),
                    debug_button("kuyg").width(180.0).into(),
                    debug_button("arga").width(180.0).into(),
                    debug_button("abcd").width(180.0).into(),
                    debug_button("vcsa").width(180.0).into(),
                    Menux::new(debug_button("htrsthfs").width(180.0).into(), vec![
                        debug_button("hghg").width(180.0).into(),
                        debug_button("kuyg").width(180.0).into(),
                        debug_button("vcsa").width(180.0).into(),
                        debug_button("kiug").width(180.0).into(),
                    ]).axis(Axis::Horizontal).into(),
                    debug_button("kiug").width(180.0).into(),
                ]).axis(Axis::Horizontal).into(),
                debug_button("abc").width(180.0).into(),
                debug_button("def").width(180.0).into(),
                debug_button("xxx").width(180.0).into(),
                debug_button("htrsrt").width(180.0).into(),
                debug_button("htrdf").width(180.0).into(),
                debug_button("ngfcgng").width(180.0).into(),
                debug_button("hytfy").width(180.0).into(),
                debug_button("kuyg").width(180.0).into(),
                debug_button("qegvd").width(180.0).into(),
                debug_button("iuoiy").width(180.0).into(),
                debug_button("rzsajf").width(180.0).into(),
                debug_button("pkmehs").width(180.0).into(),
                debug_button("ivrye").width(180.0).into(),
                debug_button("zhdkr").width(180.0).into(),
                debug_button("vjdiwo").width(180.0).into(),
            ]).open_condition(OpenCondition::Click),
        ];

        // let mb = MenuBar::new(
        //     [
        //         MenuTree::with_children(
        //             button("content").on_press(Message::Debug("content".into())), 
        //             [
        //                 MenuTree::new(button("abc")),
        //                 MenuTree::new(button("def")),
        //                 MenuTree::new(button("wagrarga")),
        //                 MenuTree::new(button("jfuykyfuk")),
        //             ].into()
        //         ),
        //         MenuTree::with_children(
        //             button("xxx").on_press(Message::Debug("xxx".into())), 
        //             [
        //                 MenuTree::new(button("abc")),
        //                 MenuTree::new(button("def")),
        //                 MenuTree::new(button("wagrarga")),
        //                 MenuTree::new(button("jfuykyfuk")),
        //             ].into()
        //         )
        //     ].into()
        // );

        /* let mb = match self.size_option {
            SizeOption::Uniform => {
                menu_bar!(menu_1(self), menu_2(self), menu_3(self), menu_4(self))
                    .item_width(ItemWidth::Uniform(180))
                    .item_height(ItemHeight::Uniform(27))
            }
            SizeOption::Static => menu_bar!(
                menu_1(self),
                menu_2(self),
                menu_3(self),
                menu_4(self),
                menu_5(self),
            )
            .item_width(ItemWidth::Static(180))
            .item_height(ItemHeight::Static(35)),
            SizeOption::DynamicHeight => menu_bar!(
                menu_1(self),
                menu_2(self),
                menu_3(self),
                menu_4(self),
                menu_6(self),
            )
            .item_width(ItemWidth::Static(180))
            .item_height(ItemHeight::Dynamic(35)),
        }
        .spacing(4.0)
        .bounds_expand(30)
        .main_offset(13)
        .cross_offset(16)
        .path_highlight(Some(PathHighlight::MenuActive))
        .close_condition(CloseCondition {
            leave: true,
            click_outside: false,
            click_inside: false,
        }); */

        let r = if self.flip_h {
            row!(pick_size_option, horizontal_space(Length::Fill), 
                mb,
            )
        } else {
            row!(
                mb, 
                horizontal_space(Length::Fill), pick_size_option
            )
        }
        .padding([2, 8])
        .align_items(alignment::Alignment::Center);

        let top_bar_style: fn(&iced::Theme) -> container::Appearance =
            |_theme| container::Appearance {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            };
        let top_bar = container(r).width(Length::Fill).style(top_bar_style);

        let back_style: fn(&iced::Theme) -> container::Appearance = |theme| container::Appearance {
            background: Some(theme.extended_palette().primary.base.color.into()),
            ..Default::default()
        };
        let back = container(col![])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(back_style);

        let c = if self.flip_v {
            col![back, top_bar,]
        } else {
            col![top_bar, back,]
        };

        c.into()
    }
}

struct ButtonStyle;
impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: style.extended_palette().background.base.text,
            background: Some(Color::TRANSPARENT.into()),
            border: Border{
                radius: [4.0;4].into(),
                ..Default::default()
            },
            ..Default::default()
            
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let plt = style.extended_palette();

        button::Appearance {
            background: Some(plt.primary.weak.color.into()),
            text_color: plt.primary.weak.text,
            ..self.active(style)
        }
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Theme, iced::Renderer>>,
    msg: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(msg)
}

fn labeled_button<'a>(label: &str, msg: Message) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    base_button(
        text(label)
            // .width(Length::Fill)
            // .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}

fn debug_button<'a>(label: &str) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    labeled_button(label, Message::Debug(label.into()))
}

fn debug_item<'a>(label: &str) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    menu_tree!(debug_button(label).width(Length::Fill).height(Length::Fill))
}

fn debug_item2<'a>(label: &str) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    menu_tree!(debug_button(label)
        .width(Length::Fill)
        .height(Length::Shrink))
}

fn debug_item3<'a>(label: &str, h: f32) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    menu_tree!(debug_button(label)
        .width(Length::Fill)
        .height(Length::Fixed(h)))
}

fn color_item<'a>(color: impl Into<Color>) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let color = color.into();
    menu_tree!(base_button(circle(color), Message::ColorChange(color)))
}

fn sub_menu<'a>(
    label: &str,
    msg: Message,
    children: Vec<MenuTree<'a, Message, iced::Theme, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let handle = svg::Handle::from_path(format!(
        "{}/caret-right-fill.svg",
        env!("CARGO_MANIFEST_DIR")
    ));
    let arrow = svg(handle)
        .width(Length::Shrink)
        .style(theme::Svg::custom_fn(|theme| svg::Appearance {
            color: Some(theme.extended_palette().background.base.text),
        }));

    menu_tree(
        base_button(
            row![
                text(label)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .vertical_alignment(alignment::Vertical::Center),
                arrow
            ]
            .align_items(iced::Alignment::Center),
            msg,
        )
        .width(Length::Fill)
        .height(Length::Fill),
        children,
    )
}

fn debug_sub_menu<'a>(
    label: &str,
    children: Vec<MenuTree<'a, Message, iced::Theme, iced::Renderer>>,
) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    sub_menu(label, Message::Debug(label.into()), children)
}

fn separator<'a>() -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    menu_tree!(quad::Quad {
        color: [0.5; 3].into(),
        border_radius: [4.0; 4],
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    })
}

fn dot_separator<'a>() -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    menu_tree!(text("·························")
        .size(30)
        .width(Length::Fill)
        .height(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center))
}

fn labeled_separator(label: &'_ str) -> MenuTree<'_, Message, iced::Theme, iced::Renderer> {
    let q_1 = quad::Quad {
        color: [0.5; 3].into(),
        border_radius: [4.0; 4],
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    };
    let q_2 = quad::Quad {
        color: [0.5; 3].into(),
        border_radius: [4.0; 4],
        inner_bounds: quad::InnerBounds::Ratio(0.98, 0.1),
        ..Default::default()
    };

    menu_tree!(row![
        q_1,
        text(label)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
        q_2,
    ])
}

fn circle(color: Color) -> quad::Quad {
    let radius = 10.0;

    quad::Quad {
        color,
        inner_bounds: quad::InnerBounds::Square(radius * 2.0),
        border_radius: [radius; 4],
        ..Default::default()
    }
}

fn menu_1<'a>(_app: &App) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let sub_5 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );
    let sub_4 = debug_sub_menu(
        "SUB",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(180);
    let sub_3 = debug_sub_menu(
        "More sub menus",
        vec![
            debug_item("You can"),
            debug_item("nest menus"),
            sub_4,
            debug_item("how ever"),
            debug_item("You like"),
            sub_5,
        ],
    );
    let sub_2 = debug_sub_menu(
        "Another sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            sub_3,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(140);
    let sub_1 = debug_sub_menu(
        "A sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_2,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(220);

    let root = menu_tree(
        debug_button("Nested Menus"),
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_1,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(110);

    root
}

fn menu_2<'a>(app: &App) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let sub_1 = menu_tree(
        container(toggler(
            Some("Or as a sub menu item".to_string()),
            app.toggle,
            Message::ToggleChange,
        ))
        .padding([0, 8])
        .height(Length::Fill)
        .align_y(alignment::Vertical::Center),
        vec![
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );

    let bt = menu_tree!(button(
        text("Button")
            .width(Length::Fill)
            .height(Length::Fill)
            .vertical_alignment(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .on_press(Message::Debug("Button".into())));

    let cb = menu_tree!(checkbox("Checkbox", app.check, Message::CheckChange).width(Length::Fill));

    let sld = menu_tree!(row![
        "Slider",
        horizontal_space(Length::Fixed(8.0)),
        slider(0..=255, app.value, Message::ValueChange)
    ]);

    let txn = menu_tree!(text_input("", &app.text).on_input(Message::TextChange));

    let root = menu_tree(
        debug_button("Widgets"),
        vec![
            debug_item("You can use any widget"),
            debug_item("as a menu item"),
            bt,
            cb,
            sld,
            txn,
            sub_1,
            separator(),
            debug_item("Seperators are also widgets"),
            labeled_separator("Separator"),
            debug_item("Item"),
            debug_item("Item"),
            dot_separator(),
            debug_item("Item"),
            debug_item("Item"),
        ],
    );

    root
}

fn menu_3<'a>(app: &App) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let primary = debug_sub_menu(
        "Primary",
        vec![
            menu_tree!(slider(0..=255, r, move |x| {
                Message::ColorChange(Color::from_rgb8(x, g, b))
            })),
            menu_tree!(slider(0..=255, g, move |x| {
                Message::ColorChange(Color::from_rgb8(r, x, b))
            })),
            menu_tree!(slider(0..=255, b, move |x| {
                Message::ColorChange(Color::from_rgb8(r, g, x))
            })),
        ],
    );

    let root = menu_tree(
        debug_button("Controls"),
        vec![
            menu_tree!(labeled_button("Flip Horizontal", Message::FlipHorizontal)
                .width(Length::Fill)
                .height(Length::Fill)),
            menu_tree!(labeled_button("Flip Vertical", Message::FlipVertical)
                .width(Length::Fill)
                .height(Length::Fill)),
            separator(),
            menu_tree!(row![toggler(
                Some("Dark Mode".into()),
                app.dark_mode,
                Message::ThemeChange
            )]
            .padding([0, 8])),
            color_item([0.45, 0.25, 0.57]),
            color_item([0.15, 0.59, 0.64]),
            color_item([0.76, 0.82, 0.20]),
            color_item([0.17, 0.27, 0.33]),
            primary,
        ],
    );

    root
}

fn menu_4<'a>(_app: &App) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let dekjdaud = debug_sub_menu(
        "dekjdaud",
        vec![
            debug_item("ajrs"),
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"),
            debug_item("ajrs"),
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"),
        ],
    );

    let luyortp = debug_sub_menu(
        "luyortp",
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 13
        ],
    );

    let jcsu = debug_sub_menu(
        "jcsu",
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            luyortp, // 11
            debug_item("mmdyrc"),
            debug_item("nquc"), // 13
        ],
    );

    let root = menu_tree(
        debug_button("Scroll"),
        vec![
            debug_item("ajrs"), // 0
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            jcsu, // 9
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 13
            debug_item("ajrs"), // 14
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 27
            debug_item("ajrs"), // 28
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            dekjdaud,
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"),
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 41
            debug_item("ajrs"), // 42
            debug_item("bsdfho"),
            debug_item("clkjhbf"),
            debug_item("dekjdaud"),
            debug_item("ecsh"),
            debug_item("fweiu"),
            debug_item("giwe"),
            debug_item("heruyv"),
            debug_item("isabe"),
            debug_item("jcsu"),
            debug_item("kaljkahd"), // 52
            debug_item("luyortp"),
            debug_item("mmdyrc"),
            debug_item("nquc"), // 55
        ],
    );

    root
}

fn menu_5<'a>(app: &App) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let slider_count = 3;
    let slider_width = 30;
    let spacing = 4;

    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let sliders = menu_tree!(row![
        vertical_slider(0..=255, r, move |x| Message::ColorChange(Color::from_rgb8(
            x, g, b
        )))
        .width(30),
        vertical_slider(0..=255, g, move |x| Message::ColorChange(Color::from_rgb8(
            r, x, b
        )))
        .width(30),
        vertical_slider(0..=255, b, move |x| Message::ColorChange(Color::from_rgb8(
            r, g, x
        )))
        .width(30),
    ]
    .spacing(4))
    .height(100);

    let root = menu_tree(
        debug_button("Static"),
        vec![labeled_separator("Primary"), sliders],
    )
    .width(slider_width * slider_count + (slider_count - 1) * spacing);

    root
}

fn menu_6<'a>(app: &App) -> MenuTree<'a, Message, iced::Theme, iced::Renderer> {
    let slider_count = 3;
    let slider_width = 30;
    let spacing = 4;

    let [r, g, b, _] = app.theme.palette().primary.into_rgba8();

    let sliders = menu_tree!(row![
        vertical_slider(0..=255, r, move |x| Message::ColorChange(Color::from_rgb8(
            x, g, b
        )))
        .width(30),
        vertical_slider(0..=255, g, move |x| Message::ColorChange(Color::from_rgb8(
            r, x, b
        )))
        .width(30),
        vertical_slider(0..=255, b, move |x| Message::ColorChange(Color::from_rgb8(
            r, g, x
        )))
        .width(30),
    ]
    .spacing(4)
    .height(100));

    let root = menu_tree(
        debug_button("Dynamic Height"),
        vec![
            labeled_separator("Primary"),
            sliders,
            debug_item2("AABB"),
            debug_item3("CCDD", 50.0),
            debug_item2("EEFF"),
            debug_item("GGHH").height(100),
            debug_item2("IIJJ"),
        ],
    )
    .width(slider_width * slider_count + (slider_count - 1) * spacing);

    root
}
