use iced::{Column, Container, Element, Length, Sandbox, Settings, Text};
use iced_aw::{menu, Menu};
use menu::{Entry, Section};

fn main() -> iced::Result {
    MenuExample::run(Settings::default())
}

#[derive(Clone, Debug)]
enum Message {
    None,
    Menu(MenuMessage),
}

struct MenuExample {
    menu: menu::State,
    last_message: Message,
    config: MenuConfig,
}
#[derive(Default)]
struct MenuConfig {
    enable_reopen_closed_editor: bool,

    enable_save_all: bool,

    enable_forward: bool,

    enable_group_3: bool,
    enable_group_4: bool,
    enable_group_5: bool,

    enable_next_group: bool,
    enable_previous_group: bool,

    enable_group_left: bool,
    enable_group_right: bool,
    enable_group_above: bool,
    enable_group_below: bool,

    enable_go_to_definition: bool,
    enable_go_to_declaration: bool,
    enable_go_to_type_definition: bool,
    enable_go_to_implementations: bool,
    enable_go_to_references: bool,

    enable_stop_debugging: bool,
    enable_restart_debugging: bool,

    enable_open_configuration: bool,

    enable_step_over: bool,
    enable_step_into: bool,
    enable_step_out: bool,
    enable_continue: bool,

    enable_show_running_tasks: bool,
    enable_restart_running_tasks: bool,
    enable_terminate_tasks: bool,
}

impl Sandbox for MenuExample {
    type Message = Message;

    fn new() -> Self {
        MenuExample {
            menu: menu::State::new(),
            last_message: Message::None,
            config: MenuConfig::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Menu example")
    }

    fn update(&mut self, message: Self::Message) {
        self.last_message = message;
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let menu: Element<'_, MenuMessage> = Menu::new(&mut self.menu)
            .push(
                Section::new(
                    Text::new("File"),
                    vec![
                        Entry::Item(Text::new("New File").into(), Some(FileMessage::NewFile)),
                        Entry::Item(Text::new("New Window").into(), Some(FileMessage::NewWindow)),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Open File...").into(),
                            Some(FileMessage::OpenFile),
                        ),
                        Entry::Item(
                            Text::new("Open Folder...").into(),
                            Some(FileMessage::OpenFolder),
                        ),
                        Entry::Item(
                            Text::new("Open Workspace...").into(),
                            Some(FileMessage::OpenWorkspace),
                        ),
                        Entry::Group(
                            Text::new("Open Recent...").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Reopen Closed Editor").into(),
                                    if self.config.enable_reopen_closed_editor {
                                        Some(OpenRecentMessage::ReopenClosedEditor)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Separator,
                                Entry::Item(Text::new("Foo").into(), Some(OpenRecentMessage::Foo)),
                                Entry::Item(Text::new("Bar").into(), Some(OpenRecentMessage::Bar)),
                                Entry::Item(Text::new("Baz").into(), Some(OpenRecentMessage::Baz)),
                                Entry::Separator,
                                Entry::Item(Text::new("Foo").into(), Some(OpenRecentMessage::Foo)),
                                Entry::Item(Text::new("Bar").into(), Some(OpenRecentMessage::Bar)),
                                Entry::Item(Text::new("Baz").into(), Some(OpenRecentMessage::Baz)),
                                Entry::Separator,
                                Entry::Group(
                                    Text::new("More...").into(),
                                    vec![
                                        Entry::Item(
                                            Text::new("Foo").into(),
                                            Some(MoreMessage::Foo),
                                        ),
                                        Entry::Item(
                                            Text::new("Bar").into(),
                                            Some(MoreMessage::Bar),
                                        ),
                                        Entry::Item(
                                            Text::new("Baz").into(),
                                            Some(MoreMessage::Baz),
                                        ),
                                    ],
                                )
                                .map(OpenRecentMessage::More),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Clear Recently Opened").into(),
                                    Some(OpenRecentMessage::ClearRecentlyOpened),
                                ),
                            ],
                        )
                        .map(FileMessage::OpenRecent),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Add folder to Workspace...").into(),
                            Some(FileMessage::AddFolderToWorkspace),
                        ),
                        Entry::Item(
                            Text::new("Save Workspace As...").into(),
                            Some(FileMessage::SaveWorkspaceAs),
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Save").into(), Some(FileMessage::Save)),
                        Entry::Item(Text::new("Save As...").into(), Some(FileMessage::SaveAs)),
                        Entry::Item(
                            Text::new("Save All").into(),
                            if self.config.enable_save_all {
                                Some(FileMessage::SaveAll)
                            } else {
                                None
                            },
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Auto Save").into(), Some(FileMessage::AutoSave)),
                        Entry::Group(
                            Text::new("Preferences").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Settings").into(),
                                    Some(PreferencesMessage::Settings),
                                ),
                                Entry::Item(
                                    Text::new("Online Services Settings").into(),
                                    Some(PreferencesMessage::OnlineServiceSettings),
                                ),
                                Entry::Item(
                                    Text::new("Extensions").into(),
                                    Some(PreferencesMessage::Extensions),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Keyboard Shortcuts [Ctrl+K Ctrl+S]").into(),
                                    Some(PreferencesMessage::KeyboardShortcuts),
                                ),
                                Entry::Item(
                                    Text::new("Keymaps [Ctrl+K Ctrl+M]").into(),
                                    Some(PreferencesMessage::Keymaps),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("User Snippets").into(),
                                    Some(PreferencesMessage::UserSnippets),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Color Theme [Ctrl+K Ctrl+T]").into(),
                                    Some(PreferencesMessage::ColorTheme),
                                ),
                                Entry::Item(
                                    Text::new("File Icon Theme").into(),
                                    Some(PreferencesMessage::FileIconTheme),
                                ),
                                Entry::Item(
                                    Text::new("Product Icon Theme").into(),
                                    Some(PreferencesMessage::ProductIconTheme),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Turn on Settings Sync...").into(),
                                    Some(PreferencesMessage::TurnOnSettingsSync),
                                ),
                            ],
                        )
                        .map(FileMessage::Preferences),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Revert File").into(),
                            Some(FileMessage::RevertFile),
                        ),
                        Entry::Item(
                            Text::new("Close Editor").into(),
                            Some(FileMessage::CloseEditor),
                        ),
                        Entry::Item(
                            Text::new("Close Folder").into(),
                            Some(FileMessage::CloseFolder),
                        ),
                        Entry::Item(
                            Text::new("Close Window").into(),
                            Some(FileMessage::CloseWindow),
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Exit").into(), Some(FileMessage::Exit)),
                    ],
                )
                .map(MenuMessage::File),
            )
            .push(
                Section::new(
                    Text::new("Edit"),
                    vec![
                        Entry::Item(Text::new("Undo").into(), Some(EditMessage::Undo)),
                        Entry::Item(Text::new("Redo").into(), Some(EditMessage::Redo)),
                        Entry::Separator,
                        Entry::Item(Text::new("Cut").into(), Some(EditMessage::Cut)),
                        Entry::Item(Text::new("Copy").into(), Some(EditMessage::Copy)),
                        Entry::Item(Text::new("Paste").into(), Some(EditMessage::Paste)),
                        Entry::Separator,
                        Entry::Item(Text::new("Find").into(), Some(EditMessage::Find)),
                        Entry::Item(Text::new("Replace").into(), Some(EditMessage::Replace)),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Find in Files").into(),
                            Some(EditMessage::FindInFiles),
                        ),
                        Entry::Item(
                            Text::new("Replace in Files").into(),
                            Some(EditMessage::ReplaceInFiles),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Line Comment").into(),
                            Some(EditMessage::ToggleLineComment),
                        ),
                        Entry::Item(
                            Text::new("Toggle Block Comment").into(),
                            Some(EditMessage::ToggleBlockComment),
                        ),
                        Entry::Item(
                            Text::new("Emmet: Expand Abbreviation").into(),
                            Some(EditMessage::EmmetExpandAbbreviation),
                        ),
                    ],
                )
                .map(MenuMessage::Edit),
            )
            .push(
                Section::new(
                    Text::new("Selection"),
                    vec![
                        Entry::Item(
                            Text::new("Select All").into(),
                            Some(SelectionMessage::SelectAll),
                        ),
                        Entry::Item(
                            Text::new("Expand Selection").into(),
                            Some(SelectionMessage::ExpandSelection),
                        ),
                        Entry::Item(
                            Text::new("Shrink Selection").into(),
                            Some(SelectionMessage::ShrinkSelection),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Copy Line Up").into(),
                            Some(SelectionMessage::CopyLineUp),
                        ),
                        Entry::Item(
                            Text::new("Copy Line Down").into(),
                            Some(SelectionMessage::CopyLineDown),
                        ),
                        Entry::Item(
                            Text::new("Move Line Up").into(),
                            Some(SelectionMessage::MoveLineUp),
                        ),
                        Entry::Item(
                            Text::new("Move Line Down").into(),
                            Some(SelectionMessage::MoveLineDown),
                        ),
                        Entry::Item(
                            Text::new("Duplicate Selection").into(),
                            Some(SelectionMessage::DuplicateSelection),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Add Cursor Above").into(),
                            Some(SelectionMessage::AddCursorAbove),
                        ),
                        Entry::Item(
                            Text::new("Add Cursor Below").into(),
                            Some(SelectionMessage::AddCursorBelow),
                        ),
                        Entry::Item(
                            Text::new("Add Cursors to Line Ends").into(),
                            Some(SelectionMessage::AddCursorsToLineEnds),
                        ),
                        Entry::Item(
                            Text::new("Add Next Occurrence").into(),
                            Some(SelectionMessage::AddNextOccurrence),
                        ),
                        Entry::Item(
                            Text::new("Add Previous Occurrence").into(),
                            Some(SelectionMessage::AddPreviousOccurrence),
                        ),
                        Entry::Item(
                            Text::new("Select All Occurrences").into(),
                            Some(SelectionMessage::SelectAllOccurrences),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Switch to Ctrl+Click for Multi-Cursor").into(),
                            Some(SelectionMessage::SwitchToCtrlClickForMultiCursor),
                        ),
                        Entry::Item(
                            Text::new("Column Selection Mode").into(),
                            Some(SelectionMessage::ColumnSelectionMode),
                        ),
                    ],
                )
                .map(MenuMessage::Selection),
            )
            .push(
                Section::new(
                    Text::new("View"),
                    vec![
                        Entry::Item(
                            Text::new("Command Palette...").into(),
                            Some(ViewMessage::CommandPalette),
                        ),
                        Entry::Item(
                            Text::new("Open View...").into(),
                            Some(ViewMessage::OpenView),
                        ),
                        Entry::Separator,
                        Entry::Group(
                            Text::new("Appearance").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Full Screen").into(),
                                    Some(AppearanceMessage::FullScreen),
                                ),
                                Entry::Item(
                                    Text::new("Zen Mode [Ctrl+K Z]").into(),
                                    Some(AppearanceMessage::ZenMode),
                                ),
                                Entry::Item(
                                    Text::new("Centered Layout").into(),
                                    Some(AppearanceMessage::CenteredLayout),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Show Menu Bar").into(),
                                    Some(AppearanceMessage::ShowMenuBar),
                                ),
                                Entry::Item(
                                    Text::new("Show Side Bar").into(),
                                    Some(AppearanceMessage::ShowSideBar),
                                ),
                                Entry::Item(
                                    Text::new("Show Status Bar").into(),
                                    Some(AppearanceMessage::ShowStatusBar),
                                ),
                                Entry::Item(
                                    Text::new("Show Activity Bar").into(),
                                    Some(AppearanceMessage::ShowActivityBar),
                                ),
                                Entry::Item(
                                    Text::new("Show Editor Bar").into(),
                                    Some(AppearanceMessage::ShowEditorBar),
                                ),
                                Entry::Item(
                                    Text::new("Show Panel Bar").into(),
                                    Some(AppearanceMessage::ShowPanelBar),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Move Side Bar Right").into(),
                                    Some(AppearanceMessage::MoveSideBarRight),
                                ),
                                Entry::Item(
                                    Text::new("Move Panel Left").into(),
                                    Some(AppearanceMessage::MovePanelLeft),
                                ),
                                Entry::Item(
                                    Text::new("Move Panel Right").into(),
                                    Some(AppearanceMessage::MovePanelRight),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Zoom In").into(),
                                    Some(AppearanceMessage::ZoomIn),
                                ),
                                Entry::Item(
                                    Text::new("Zoom Out").into(),
                                    Some(AppearanceMessage::ZoomOut),
                                ),
                                Entry::Item(
                                    Text::new("Reset Zoom [Ctrl+NumPad0").into(),
                                    Some(AppearanceMessage::ResetZoom),
                                ),
                            ],
                        )
                        .map(ViewMessage::Appearance),
                        Entry::Group(
                            Text::new("Editor Layout").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Split Up").into(),
                                    Some(EditorLayoutMessage::SplitUp),
                                ),
                                Entry::Item(
                                    Text::new("Split Down").into(),
                                    Some(EditorLayoutMessage::SplitDown),
                                ),
                                Entry::Item(
                                    Text::new("Split Left").into(),
                                    Some(EditorLayoutMessage::SplitLeft),
                                ),
                                Entry::Item(
                                    Text::new("Split Right").into(),
                                    Some(EditorLayoutMessage::SplitRight),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Single").into(),
                                    Some(EditorLayoutMessage::Single),
                                ),
                                Entry::Item(
                                    Text::new("Two Columns").into(),
                                    Some(EditorLayoutMessage::TwoColumns),
                                ),
                                Entry::Item(
                                    Text::new("Three Columns").into(),
                                    Some(EditorLayoutMessage::ThreeColumns),
                                ),
                                Entry::Item(
                                    Text::new("Two Rows").into(),
                                    Some(EditorLayoutMessage::TwoRows),
                                ),
                                Entry::Item(
                                    Text::new("Three Rows").into(),
                                    Some(EditorLayoutMessage::ThreeRows),
                                ),
                                Entry::Item(
                                    Text::new("Grid (2x2)").into(),
                                    Some(EditorLayoutMessage::Grid),
                                ),
                                Entry::Item(
                                    Text::new("Two Rows Right").into(),
                                    Some(EditorLayoutMessage::TwoRowsRight),
                                ),
                                Entry::Item(
                                    Text::new("Two Columns Bottom").into(),
                                    Some(EditorLayoutMessage::TwoColumnsBottom),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Flip Layout").into(),
                                    Some(EditorLayoutMessage::FlipLayout),
                                ),
                            ],
                        )
                        .map(ViewMessage::EditorLayout),
                        Entry::Separator,
                        Entry::Item(Text::new("Explorer").into(), Some(ViewMessage::Explorer)),
                        Entry::Item(Text::new("Search").into(), Some(ViewMessage::Search)),
                        Entry::Item(
                            Text::new("SCM [Ctrl+Shift+G G]").into(),
                            Some(ViewMessage::Scm),
                        ),
                        Entry::Item(Text::new("Run").into(), Some(ViewMessage::Run)),
                        Entry::Item(
                            Text::new("Extensions").into(),
                            Some(ViewMessage::Extensions),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Output [Ctrl+K Ctrl+H]").into(),
                            Some(ViewMessage::Output),
                        ),
                        Entry::Item(
                            Text::new("Debug Console").into(),
                            Some(ViewMessage::DebugConsole),
                        ),
                        Entry::Item(Text::new("Terminal").into(), Some(ViewMessage::Terminal)),
                        Entry::Item(Text::new("Problems").into(), Some(ViewMessage::Problems)),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Word Wrap").into(),
                            Some(ViewMessage::ToggleWordWrap),
                        ),
                        Entry::Item(
                            Text::new("Show Minimap").into(),
                            Some(ViewMessage::ShowMinimap),
                        ),
                        Entry::Item(
                            Text::new("Show Breadcrumbs").into(),
                            Some(ViewMessage::ShowBreadcrumbs),
                        ),
                        Entry::Item(
                            Text::new("Render Whitespace").into(),
                            Some(ViewMessage::RenderWhitespace),
                        ),
                        Entry::Item(
                            Text::new("Render Control Characters").into(),
                            Some(ViewMessage::RenderControlCharacters),
                        ),
                    ],
                )
                .map(MenuMessage::View),
            )
            .push(
                Section::new(
                    Text::new("Go"),
                    vec![
                        Entry::Item(Text::new("Back").into(), Some(GoMessage::Back)),
                        Entry::Item(
                            Text::new("Forward").into(),
                            if self.config.enable_forward {
                                Some(GoMessage::Forward)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Last Edit Location [Ctrl+K Ctrl+Q]").into(),
                            Some(GoMessage::LastEditLocation),
                        ),
                        Entry::Separator,
                        Entry::Group(
                            Text::new("Switch Editor").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Next Editor").into(),
                                    Some(SwitchEditorMessage::NextEditor),
                                ),
                                Entry::Item(
                                    Text::new("Previous Editor").into(),
                                    Some(SwitchEditorMessage::PreviousEditor),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Used Editor").into(),
                                    Some(SwitchEditorMessage::NextUsedEditor),
                                ),
                                Entry::Item(
                                    Text::new("Previous Used Editor").into(),
                                    Some(SwitchEditorMessage::PreviousUsedEditor),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Editor in Group [Ctrl+K Ctrl+PageDown]").into(),
                                    Some(SwitchEditorMessage::NextEditorInGroup),
                                ),
                                Entry::Item(
                                    Text::new("Previous Editor in Group [Ctrl+K Ctrl+PageUp]")
                                        .into(),
                                    Some(SwitchEditorMessage::PreviousEditorInGroup),
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Used Editor in Group").into(),
                                    Some(SwitchEditorMessage::NextUsedEditorInGroup),
                                ),
                                Entry::Item(
                                    Text::new("Previous Used Editor in Group").into(),
                                    Some(SwitchEditorMessage::PreviousUsedEditorInGroup),
                                ),
                            ],
                        )
                        .map(GoMessage::SwitchEditor),
                        Entry::Group(
                            Text::new("Switch Group").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Group 1").into(),
                                    Some(SwitchGroupMessage::Group1),
                                ),
                                Entry::Item(
                                    Text::new("Group 2").into(),
                                    Some(SwitchGroupMessage::Group2),
                                ),
                                Entry::Item(
                                    Text::new("Group 3").into(),
                                    if self.config.enable_group_3 {
                                        Some(SwitchGroupMessage::Group3)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Item(
                                    Text::new("Group 4").into(),
                                    if self.config.enable_group_4 {
                                        Some(SwitchGroupMessage::Group4)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Item(
                                    Text::new("Group 5").into(),
                                    if self.config.enable_group_5 {
                                        Some(SwitchGroupMessage::Group5)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Group").into(),
                                    if self.config.enable_next_group {
                                        Some(SwitchGroupMessage::NextGroup)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Item(
                                    Text::new("Previous Group").into(),
                                    if self.config.enable_previous_group {
                                        Some(SwitchGroupMessage::PreviousGroup)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Group Left [Ctrl+K Ctrl+LeftArrow]").into(),
                                    if self.config.enable_group_left {
                                        Some(SwitchGroupMessage::GroupLeft)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Item(
                                    Text::new("Group Right [Ctrl+K Ctrl+RightArrow]").into(),
                                    if self.config.enable_group_right {
                                        Some(SwitchGroupMessage::GroupRight)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Item(
                                    Text::new("Group Above [Ctrl+K Ctrl+UpArrow]").into(),
                                    if self.config.enable_group_above {
                                        Some(SwitchGroupMessage::GroupAbove)
                                    } else {
                                        None
                                    },
                                ),
                                Entry::Item(
                                    Text::new("Group Below [Ctrl+K Ctrl+DownArrow]").into(),
                                    if self.config.enable_group_below {
                                        Some(SwitchGroupMessage::GroupBelow)
                                    } else {
                                        None
                                    },
                                ),
                            ],
                        )
                        .map(GoMessage::SwitchGroup),
                        Entry::Separator,
                        Entry::Item(Text::new("Go to File...").into(), Some(GoMessage::GoToFile)),
                        Entry::Item(
                            Text::new("Go to Symbol in Workspace...").into(),
                            Some(GoMessage::GoToSymbolInWorkspace),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Go to Symbol in Editor...").into(),
                            Some(GoMessage::GoToSymbolInEditor),
                        ),
                        Entry::Item(
                            Text::new("Go to Definition").into(),
                            if self.config.enable_go_to_definition {
                                Some(GoMessage::GoToDefinition)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Go to Declaration").into(),
                            if self.config.enable_go_to_declaration {
                                Some(GoMessage::GoToDeclaration)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Go to Type Definition").into(),
                            if self.config.enable_go_to_type_definition {
                                Some(GoMessage::GoToTypeDefinition)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Go to Implementations").into(),
                            if self.config.enable_go_to_implementations {
                                Some(GoMessage::GoToImplementations)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Go to References").into(),
                            if self.config.enable_go_to_references {
                                Some(GoMessage::GoToReferences)
                            } else {
                                None
                            },
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Go to Line/Column...").into(),
                            Some(GoMessage::GoToLineColumn),
                        ),
                        Entry::Item(
                            Text::new("Go to Bracket").into(),
                            Some(GoMessage::GoToBracket),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Next Problem").into(),
                            Some(GoMessage::NextProblem),
                        ),
                        Entry::Item(
                            Text::new("Previous Problem").into(),
                            Some(GoMessage::PreviousProblem),
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Next Change").into(), Some(GoMessage::NextChange)),
                        Entry::Item(
                            Text::new("Previous Change").into(),
                            Some(GoMessage::PreviousChange),
                        ),
                    ],
                )
                .map(MenuMessage::Go),
            )
            .push(
                Section::new(
                    Text::new("Run"),
                    vec![
                        Entry::Item(
                            Text::new("Start Debugging").into(),
                            Some(RunMessage::StartDebugging),
                        ),
                        Entry::Item(
                            Text::new("Run Without Debugging").into(),
                            Some(RunMessage::RunWithoutDebugging),
                        ),
                        Entry::Item(
                            Text::new("Stop Debugging").into(),
                            if self.config.enable_stop_debugging {
                                Some(RunMessage::StopDebugging)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Restart Debugging").into(),
                            if self.config.enable_restart_debugging {
                                Some(RunMessage::RestartDebugging)
                            } else {
                                None
                            },
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Open Configurations").into(),
                            if self.config.enable_open_configuration {
                                Some(RunMessage::OpenConfigurations)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Add Configuration...").into(),
                            Some(RunMessage::AddConfiguration),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Step Over").into(),
                            if self.config.enable_step_over {
                                Some(RunMessage::StepOver)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Step Into").into(),
                            if self.config.enable_step_into {
                                Some(RunMessage::StepInto)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Step Out").into(),
                            if self.config.enable_step_out {
                                Some(RunMessage::StepOut)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Continue").into(),
                            if self.config.enable_continue {
                                Some(RunMessage::Continue)
                            } else {
                                None
                            },
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Breakpoint").into(),
                            Some(RunMessage::ToggleBreakpoint),
                        ),
                        Entry::Group(
                            Text::new("New Breakpoint").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Conditional Breakpoint...").into(),
                                    Some(NewBreakpointMessage::ConditionalBreakpoint),
                                ),
                                Entry::Item(
                                    Text::new("Inline Breakpoint").into(),
                                    Some(NewBreakpointMessage::InlineBreakpoint),
                                ),
                                Entry::Item(
                                    Text::new("Function Breakpoint...").into(),
                                    Some(NewBreakpointMessage::FunctionBreakpoint),
                                ),
                                Entry::Item(
                                    Text::new("Logpoint...").into(),
                                    Some(NewBreakpointMessage::Logpoint),
                                ),
                            ],
                        )
                        .map(RunMessage::NewBreakpoint),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Enable All Breakpoints").into(),
                            Some(RunMessage::EnableAllBreakpoints),
                        ),
                        Entry::Item(
                            Text::new("Disable All Breakpoints").into(),
                            Some(RunMessage::DisableAllBreakpoints),
                        ),
                        Entry::Item(
                            Text::new("Remove All Breakpoints").into(),
                            Some(RunMessage::RemoveAllBreakpoints),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Install Additional Debuggers...").into(),
                            Some(RunMessage::InstallAdditionalDebuggers),
                        ),
                    ],
                )
                .map(MenuMessage::Run),
            )
            .push(
                Section::new(
                    Text::new("Terminal"),
                    vec![
                        Entry::Item(
                            Text::new("New Terminal").into(),
                            Some(TerminalMessage::NewTerminal),
                        ),
                        Entry::Item(
                            Text::new("Split Terminal").into(),
                            Some(TerminalMessage::SplitTerminal),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Run Task...").into(),
                            Some(TerminalMessage::RunTask),
                        ),
                        Entry::Item(
                            Text::new("Run Build Task...").into(),
                            Some(TerminalMessage::RunBuildTask),
                        ),
                        Entry::Item(
                            Text::new("Run Active File").into(),
                            Some(TerminalMessage::RunActiveFile),
                        ),
                        Entry::Item(
                            Text::new("Run Selected Text").into(),
                            Some(TerminalMessage::RunSelectedText),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Show Running Tasks...").into(),
                            if self.config.enable_show_running_tasks {
                                Some(TerminalMessage::ShowRunningTasks)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Restart Running Tasks...").into(),
                            if self.config.enable_restart_running_tasks {
                                Some(TerminalMessage::RestartRunningTasks)
                            } else {
                                None
                            },
                        ),
                        Entry::Item(
                            Text::new("Terminate Tasks...").into(),
                            if self.config.enable_terminate_tasks {
                                Some(TerminalMessage::TerminateTasks)
                            } else {
                                None
                            },
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Configure Tasks...").into(),
                            Some(TerminalMessage::ConfigureTasks),
                        ),
                        Entry::Item(
                            Text::new("Configure Default Build Task...").into(),
                            Some(TerminalMessage::ConfigureDefaultBuildTasks),
                        ),
                    ],
                )
                .map(MenuMessage::Terminal),
            )
            .push(
                Section::new(
                    Text::new("Help"),
                    vec![
                        Entry::Item(Text::new("Welcome").into(), Some(HelpMessage::Welcome)),
                        Entry::Item(
                            Text::new("Interactive Playground").into(),
                            Some(HelpMessage::InteractivePlayground),
                        ),
                        Entry::Item(
                            Text::new("Documentation").into(),
                            Some(HelpMessage::Documentation),
                        ),
                        Entry::Item(
                            Text::new("Release Notes").into(),
                            Some(HelpMessage::ReleaseNotes),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Keyboard Shortcuts Reference [Ctrl+K Ctrl+R]").into(),
                            Some(HelpMessage::KeyboardShortCutsReference),
                        ),
                        Entry::Item(
                            Text::new("Introductory Videos").into(),
                            Some(HelpMessage::IntroductoryVideos),
                        ),
                        Entry::Item(
                            Text::new("Tips and Tricks").into(),
                            Some(HelpMessage::TipsAndTricks),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Join Us on Twitter").into(),
                            Some(HelpMessage::JoinUsOnTwitter),
                        ),
                        Entry::Item(
                            Text::new("Search Feature Requests").into(),
                            Some(HelpMessage::SearchFeatureRequests),
                        ),
                        Entry::Item(
                            Text::new("Report Issue").into(),
                            Some(HelpMessage::ReportIssue),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("View License").into(),
                            Some(HelpMessage::ViewLicense),
                        ),
                        Entry::Item(
                            Text::new("Privacy Statement").into(),
                            Some(HelpMessage::PrivacyStatement),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Developer Tools").into(),
                            Some(HelpMessage::ToggleDeveloperTools),
                        ),
                        Entry::Item(
                            Text::new("Open Process Explorer").into(),
                            Some(HelpMessage::OpenProcessExplorer),
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Download Available Update").into(),
                            Some(HelpMessage::DownloadAvailableUpdate),
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("About").into(), Some(HelpMessage::About)),
                    ],
                )
                .map(MenuMessage::Help),
            )
            .into();

        let menu = menu.map(Message::Menu);

        Column::new()
            .push(menu)
            .push(
                Container::new(Text::new(format!("Last Message: {:?}", self.last_message)))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x()
                    .center_y(),
            )
            .into()
    }
}

#[derive(Clone, Debug)]
enum MenuMessage {
    File(FileMessage),
    Edit(EditMessage),
    Selection(SelectionMessage),
    View(ViewMessage),
    Go(GoMessage),
    Run(RunMessage),
    Terminal(TerminalMessage),
    Help(HelpMessage),
}

#[derive(Clone, Debug)]
enum FileMessage {
    NewFile,
    NewWindow,
    OpenFile,
    OpenFolder,
    OpenWorkspace,
    OpenRecent(OpenRecentMessage),
    AddFolderToWorkspace,
    SaveWorkspaceAs,
    Save,
    SaveAs,
    SaveAll,
    AutoSave,
    Preferences(PreferencesMessage),
    RevertFile,
    CloseEditor,
    CloseFolder,
    CloseWindow,
    Exit,
}

#[derive(Clone, Debug)]
enum OpenRecentMessage {
    ReopenClosedEditor,
    Foo,
    Bar,
    Baz,
    More(MoreMessage),
    ClearRecentlyOpened,
}

#[derive(Clone, Debug)]
enum MoreMessage {
    Foo,
    Bar,
    Baz,
}

#[derive(Clone, Debug)]
enum PreferencesMessage {
    Settings,
    OnlineServiceSettings,
    Extensions,
    KeyboardShortcuts,
    Keymaps,
    UserSnippets,
    ColorTheme,
    FileIconTheme,
    ProductIconTheme,
    TurnOnSettingsSync,
}

#[derive(Clone, Debug)]
enum EditMessage {
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Find,
    Replace,
    FindInFiles,
    ReplaceInFiles,
    ToggleLineComment,
    ToggleBlockComment,
    EmmetExpandAbbreviation,
}

#[derive(Clone, Debug)]
enum SelectionMessage {
    SelectAll,
    ExpandSelection,
    ShrinkSelection,
    CopyLineUp,
    CopyLineDown,
    MoveLineUp,
    MoveLineDown,
    DuplicateSelection,
    AddCursorAbove,
    AddCursorBelow,
    AddCursorsToLineEnds,
    AddNextOccurrence,
    AddPreviousOccurrence,
    SelectAllOccurrences,
    SwitchToCtrlClickForMultiCursor,
    ColumnSelectionMode,
}

#[derive(Clone, Debug)]
enum ViewMessage {
    CommandPalette,
    OpenView,
    Appearance(AppearanceMessage),
    EditorLayout(EditorLayoutMessage),
    Explorer,
    Search,
    Scm,
    Run,
    Extensions,
    Output,
    DebugConsole,
    Terminal,
    Problems,
    ToggleWordWrap,
    ShowMinimap,
    ShowBreadcrumbs,
    RenderWhitespace,
    RenderControlCharacters,
}

#[derive(Clone, Debug)]
enum AppearanceMessage {
    FullScreen,
    ZenMode,
    CenteredLayout,
    ShowMenuBar,
    ShowSideBar,
    ShowStatusBar,
    ShowActivityBar,
    ShowEditorBar,
    ShowPanelBar,
    MoveSideBarRight,
    MovePanelLeft,
    MovePanelRight,
    ZoomIn,
    ZoomOut,
    ResetZoom,
}

#[derive(Clone, Debug)]
enum EditorLayoutMessage {
    SplitUp,
    SplitDown,
    SplitLeft,
    SplitRight,
    Single,
    TwoColumns,
    ThreeColumns,
    TwoRows,
    ThreeRows,
    Grid,
    TwoRowsRight,
    TwoColumnsBottom,
    FlipLayout,
}

#[derive(Clone, Debug)]
enum GoMessage {
    Back,
    Forward,
    LastEditLocation,
    SwitchEditor(SwitchEditorMessage),
    SwitchGroup(SwitchGroupMessage),
    GoToFile,
    GoToSymbolInWorkspace,
    GoToSymbolInEditor,
    GoToDefinition,
    GoToDeclaration,
    GoToTypeDefinition,
    GoToImplementations,
    GoToReferences,
    GoToLineColumn,
    GoToBracket,
    NextProblem,
    PreviousProblem,
    NextChange,
    PreviousChange,
}

#[derive(Clone, Debug)]
enum SwitchEditorMessage {
    NextEditor,
    PreviousEditor,
    NextUsedEditor,
    PreviousUsedEditor,
    NextEditorInGroup,
    PreviousEditorInGroup,
    NextUsedEditorInGroup,
    PreviousUsedEditorInGroup,
}

#[derive(Clone, Debug)]
enum SwitchGroupMessage {
    Group1,
    Group2,
    Group3,
    Group4,
    Group5,
    NextGroup,
    PreviousGroup,
    GroupLeft,
    GroupRight,
    GroupAbove,
    GroupBelow,
}

#[derive(Clone, Debug)]
enum RunMessage {
    StartDebugging,
    RunWithoutDebugging,
    StopDebugging,
    RestartDebugging,
    OpenConfigurations,
    AddConfiguration,
    StepOver,
    StepInto,
    StepOut,
    Continue,
    ToggleBreakpoint,
    NewBreakpoint(NewBreakpointMessage),
    EnableAllBreakpoints,
    DisableAllBreakpoints,
    RemoveAllBreakpoints,
    InstallAdditionalDebuggers,
}

#[derive(Clone, Debug)]
enum NewBreakpointMessage {
    ConditionalBreakpoint,
    InlineBreakpoint,
    FunctionBreakpoint,
    Logpoint,
}

#[derive(Clone, Debug)]
enum TerminalMessage {
    NewTerminal,
    SplitTerminal,
    RunTask,
    RunBuildTask,
    RunActiveFile,
    RunSelectedText,
    ShowRunningTasks,
    RestartRunningTasks,
    TerminateTasks,
    ConfigureTasks,
    ConfigureDefaultBuildTasks,
}

#[derive(Clone, Debug)]
enum HelpMessage {
    Welcome,
    InteractivePlayground,
    Documentation,
    ReleaseNotes,
    KeyboardShortCutsReference,
    IntroductoryVideos,
    TipsAndTricks,
    JoinUsOnTwitter,
    SearchFeatureRequests,
    ReportIssue,
    ViewLicense,
    PrivacyStatement,
    ToggleDeveloperTools,
    OpenProcessExplorer,
    DownloadAvailableUpdate,
    About,
}
