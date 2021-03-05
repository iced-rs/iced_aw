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
}

impl Sandbox for MenuExample {
    type Message = Message;

    fn new() -> Self {
        MenuExample {
            menu: menu::State::new(),
            last_message: Message::None,
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
                        Entry::Item(Text::new("New File").into(), FileMessage::NewFile),
                        Entry::Item(Text::new("New Window").into(), FileMessage::NewWindow),
                        Entry::Separator,
                        Entry::Item(Text::new("Open File...").into(), FileMessage::OpenFile),
                        Entry::Item(Text::new("Open Folder...").into(), FileMessage::OpenFolder),
                        Entry::Item(
                            Text::new("Open Workspace...").into(),
                            FileMessage::OpenWorkspace,
                        ),
                        Entry::Group(
                            Text::new("Open Recent...").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Reopen Closed Editor").into(),
                                    OpenRecentMessage::ReopenClosedEditor,
                                ),
                                Entry::Separator,
                                Entry::Item(Text::new("Foo").into(), OpenRecentMessage::Foo),
                                Entry::Item(Text::new("Bar").into(), OpenRecentMessage::Bar),
                                Entry::Item(Text::new("Baz").into(), OpenRecentMessage::Baz),
                                Entry::Separator,
                                Entry::Item(Text::new("Foo").into(), OpenRecentMessage::Foo),
                                Entry::Item(Text::new("Bar").into(), OpenRecentMessage::Bar),
                                Entry::Item(Text::new("Baz").into(), OpenRecentMessage::Baz),
                                Entry::Separator,
                                Entry::Group(
                                    Text::new("More...").into(),
                                    vec![
                                        Entry::Item(Text::new("Foo").into(), MoreMessage::Foo),
                                        Entry::Item(Text::new("Bar").into(), MoreMessage::Bar),
                                        Entry::Item(Text::new("Baz").into(), MoreMessage::Baz),
                                    ],
                                )
                                .map(OpenRecentMessage::More),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Clear Recently Opened").into(),
                                    OpenRecentMessage::ClearRecentlyOpened,
                                ),
                            ],
                        )
                        .map(FileMessage::OpenRecent),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Add folder to Workspace...").into(),
                            FileMessage::AddFolderToWorkspace,
                        ),
                        Entry::Item(
                            Text::new("Save Workspace As...").into(),
                            FileMessage::SaveWorkspaceAs,
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Save").into(), FileMessage::Save),
                        Entry::Item(Text::new("Save As...").into(), FileMessage::SaveAs),
                        Entry::Item(Text::new("Save All").into(), FileMessage::SaveAll),
                        Entry::Separator,
                        Entry::Item(Text::new("Auto Save").into(), FileMessage::AutoSave),
                        Entry::Group(
                            Text::new("Preferences").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Settings").into(),
                                    PreferencesMessage::Settings,
                                ),
                                Entry::Item(
                                    Text::new("Online Services Settings").into(),
                                    PreferencesMessage::OnlineServiceSettings,
                                ),
                                Entry::Item(
                                    Text::new("Extensions").into(),
                                    PreferencesMessage::Extensions,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Keyboard Shortcuts [Ctrl+K Ctrl+S]").into(),
                                    PreferencesMessage::KeyboardShortcuts,
                                ),
                                Entry::Item(
                                    Text::new("Keymaps [Ctrl+K Ctrl+M]").into(),
                                    PreferencesMessage::Keymaps,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("User Snippets").into(),
                                    PreferencesMessage::UserSnippets,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Color Theme [Ctrl+K Ctrl+T]").into(),
                                    PreferencesMessage::ColorTheme,
                                ),
                                Entry::Item(
                                    Text::new("File Icon Theme").into(),
                                    PreferencesMessage::FileIconTheme,
                                ),
                                Entry::Item(
                                    Text::new("Product Icon Theme").into(),
                                    PreferencesMessage::ProductIconTheme,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Turn on Settings Sync...").into(),
                                    PreferencesMessage::TurnOnSettingsSync,
                                ),
                            ],
                        )
                        .map(FileMessage::Preferences),
                        Entry::Separator,
                        Entry::Item(Text::new("Revert File").into(), FileMessage::RevertFile),
                        Entry::Item(Text::new("Close Editor").into(), FileMessage::CloseEditor),
                        Entry::Item(Text::new("Close Folder").into(), FileMessage::CloseFolder),
                        Entry::Item(Text::new("Close Window").into(), FileMessage::CloseWindow),
                        Entry::Separator,
                        Entry::Item(Text::new("Exit").into(), FileMessage::Exit),
                    ],
                )
                .map(MenuMessage::File),
            )
            .push(
                Section::new(
                    Text::new("Edit"),
                    vec![
                        Entry::Item(Text::new("Undo").into(), EditMessage::Undo),
                        Entry::Item(Text::new("Redo").into(), EditMessage::Redo),
                        Entry::Separator,
                        Entry::Item(Text::new("Cut").into(), EditMessage::Cut),
                        Entry::Item(Text::new("Copy").into(), EditMessage::Copy),
                        Entry::Item(Text::new("Paste").into(), EditMessage::Paste),
                        Entry::Separator,
                        Entry::Item(Text::new("Find").into(), EditMessage::Find),
                        Entry::Item(Text::new("Replace").into(), EditMessage::Replace),
                        Entry::Separator,
                        Entry::Item(Text::new("Find in Files").into(), EditMessage::FindInFiles),
                        Entry::Item(
                            Text::new("Replace in Files").into(),
                            EditMessage::ReplaceInFiles,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Line Comment").into(),
                            EditMessage::ToggleLineComment,
                        ),
                        Entry::Item(
                            Text::new("Toggle Block Comment").into(),
                            EditMessage::ToggleBlockComment,
                        ),
                        Entry::Item(
                            Text::new("Emmet: Expand Abbreviation").into(),
                            EditMessage::EmmetExpandAbbreviation,
                        ),
                    ],
                )
                .map(MenuMessage::Edit),
            )
            .push(
                Section::new(
                    Text::new("Selection"),
                    vec![
                        Entry::Item(Text::new("Select All").into(), SelectionMessage::SelectAll),
                        Entry::Item(
                            Text::new("Expand Selection").into(),
                            SelectionMessage::ExpandSelection,
                        ),
                        Entry::Item(
                            Text::new("Shrink Selection").into(),
                            SelectionMessage::ShrinkSelection,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Copy Line Up").into(),
                            SelectionMessage::CopyLineUp,
                        ),
                        Entry::Item(
                            Text::new("Copy Line Down").into(),
                            SelectionMessage::CopyLineDown,
                        ),
                        Entry::Item(
                            Text::new("Move Line Up").into(),
                            SelectionMessage::MoveLineUp,
                        ),
                        Entry::Item(
                            Text::new("Move Line Down").into(),
                            SelectionMessage::MoveLineDown,
                        ),
                        Entry::Item(
                            Text::new("Duplicate Selection").into(),
                            SelectionMessage::DuplicateSelection,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Add Cursor Above").into(),
                            SelectionMessage::AddCursorAbove,
                        ),
                        Entry::Item(
                            Text::new("Add Cursor Below").into(),
                            SelectionMessage::AddCursorBelow,
                        ),
                        Entry::Item(
                            Text::new("Add Cursors to Line Ends").into(),
                            SelectionMessage::AddCursorsToLineEnds,
                        ),
                        Entry::Item(
                            Text::new("Add Next Occurrence").into(),
                            SelectionMessage::AddNextOccurrence,
                        ),
                        Entry::Item(
                            Text::new("Add Previous Occurrence").into(),
                            SelectionMessage::AddPreviousOccurrence,
                        ),
                        Entry::Item(
                            Text::new("Select All Occurrences").into(),
                            SelectionMessage::SelectAllOccurrences,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Switch to Ctrl+Click for Multi-Cursor").into(),
                            SelectionMessage::SwitchToCtrlClickForMultiCursor,
                        ),
                        Entry::Item(
                            Text::new("Column Selection Mode").into(),
                            SelectionMessage::ColumnSelectionMode,
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
                            ViewMessage::CommandPalette,
                        ),
                        Entry::Item(Text::new("Open View...").into(), ViewMessage::OpenView),
                        Entry::Separator,
                        Entry::Group(
                            Text::new("Appearance").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Full Screen").into(),
                                    AppearanceMessage::FullScreen,
                                ),
                                Entry::Item(
                                    Text::new("Zen Mode [Ctrl+K Z]").into(),
                                    AppearanceMessage::ZenMode,
                                ),
                                Entry::Item(
                                    Text::new("Centered Layout").into(),
                                    AppearanceMessage::CenteredLayout,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Show Menu Bar").into(),
                                    AppearanceMessage::ShowMenuBar,
                                ),
                                Entry::Item(
                                    Text::new("Show Side Bar").into(),
                                    AppearanceMessage::ShowSideBar,
                                ),
                                Entry::Item(
                                    Text::new("Show Status Bar").into(),
                                    AppearanceMessage::ShowStatusBar,
                                ),
                                Entry::Item(
                                    Text::new("Show Activity Bar").into(),
                                    AppearanceMessage::ShowActivityBar,
                                ),
                                Entry::Item(
                                    Text::new("Show Editor Bar").into(),
                                    AppearanceMessage::ShowEditorBar,
                                ),
                                Entry::Item(
                                    Text::new("Show Panel Bar").into(),
                                    AppearanceMessage::ShowPanelBar,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Move Side Bar Right").into(),
                                    AppearanceMessage::MoveSideBarRight,
                                ),
                                Entry::Item(
                                    Text::new("Move Panel Left").into(),
                                    AppearanceMessage::MovePanelLeft,
                                ),
                                Entry::Item(
                                    Text::new("Move Panel Right").into(),
                                    AppearanceMessage::MovePanelRight,
                                ),
                                Entry::Separator,
                                Entry::Item(Text::new("Zoom In").into(), AppearanceMessage::ZoomIn),
                                Entry::Item(
                                    Text::new("Zoom Out").into(),
                                    AppearanceMessage::ZoomOut,
                                ),
                                Entry::Item(
                                    Text::new("Reset Zoom [Ctrl+NumPad0").into(),
                                    AppearanceMessage::ResetZoom,
                                ),
                            ],
                        )
                        .map(ViewMessage::Appearance),
                        Entry::Group(
                            Text::new("Editor Layout").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Split Up").into(),
                                    EditorLayoutMessage::SplitUp,
                                ),
                                Entry::Item(
                                    Text::new("Split Down").into(),
                                    EditorLayoutMessage::SplitDown,
                                ),
                                Entry::Item(
                                    Text::new("Split Left").into(),
                                    EditorLayoutMessage::SplitLeft,
                                ),
                                Entry::Item(
                                    Text::new("Split Right").into(),
                                    EditorLayoutMessage::SplitRight,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Single").into(),
                                    EditorLayoutMessage::Single,
                                ),
                                Entry::Item(
                                    Text::new("Two Columns").into(),
                                    EditorLayoutMessage::TwoColumns,
                                ),
                                Entry::Item(
                                    Text::new("Three Columns").into(),
                                    EditorLayoutMessage::ThreeColumns,
                                ),
                                Entry::Item(
                                    Text::new("Two Rows").into(),
                                    EditorLayoutMessage::TwoRows,
                                ),
                                Entry::Item(
                                    Text::new("Three Rows").into(),
                                    EditorLayoutMessage::ThreeRows,
                                ),
                                Entry::Item(
                                    Text::new("Grid (2x2)").into(),
                                    EditorLayoutMessage::Grid,
                                ),
                                Entry::Item(
                                    Text::new("Two Rows Right").into(),
                                    EditorLayoutMessage::TwoRowsRight,
                                ),
                                Entry::Item(
                                    Text::new("Two Columns Bottom").into(),
                                    EditorLayoutMessage::TwoColumnsBottom,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Flip Layout").into(),
                                    EditorLayoutMessage::FlipLayout,
                                ),
                            ],
                        )
                        .map(ViewMessage::EditorLayout),
                        Entry::Separator,
                        Entry::Item(Text::new("Explorer").into(), ViewMessage::Explorer),
                        Entry::Item(Text::new("Search").into(), ViewMessage::Search),
                        Entry::Item(Text::new("SCM [Ctrl+Shift+G G]").into(), ViewMessage::Scm),
                        Entry::Item(Text::new("Run").into(), ViewMessage::Run),
                        Entry::Item(Text::new("Extensions").into(), ViewMessage::Extensions),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Output [Ctrl+K Ctrl+H]").into(),
                            ViewMessage::Output,
                        ),
                        Entry::Item(Text::new("Debug Console").into(), ViewMessage::DebugConsole),
                        Entry::Item(Text::new("Terminal").into(), ViewMessage::Terminal),
                        Entry::Item(Text::new("Problems").into(), ViewMessage::Problems),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Word Wrap").into(),
                            ViewMessage::ToggleWordWrap,
                        ),
                        Entry::Item(Text::new("Show Minimap").into(), ViewMessage::ShowMinimap),
                        Entry::Item(
                            Text::new("Show Breadcrumbs").into(),
                            ViewMessage::ShowBreadcrumbs,
                        ),
                        Entry::Item(
                            Text::new("Render Whitespace").into(),
                            ViewMessage::RenderWhitespace,
                        ),
                        Entry::Item(
                            Text::new("Render Control Characters").into(),
                            ViewMessage::RenderControlCharacters,
                        ),
                    ],
                )
                .map(MenuMessage::View),
            )
            .push(
                Section::new(
                    Text::new("Go"),
                    vec![
                        Entry::Item(Text::new("Back").into(), GoMessage::Back),
                        Entry::Item(Text::new("Forward").into(), GoMessage::Forward),
                        Entry::Item(
                            Text::new("Last Edit Location [Ctrl+K Ctrl+Q]").into(),
                            GoMessage::LastEditLocation,
                        ),
                        Entry::Separator,
                        Entry::Group(
                            Text::new("Switch Editor").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Next Editor").into(),
                                    SwitchEditorMessage::NextEditor,
                                ),
                                Entry::Item(
                                    Text::new("Previous Editor").into(),
                                    SwitchEditorMessage::PreviousEditor,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Used Editor").into(),
                                    SwitchEditorMessage::NextUsedEditor,
                                ),
                                Entry::Item(
                                    Text::new("Previous Used Editor").into(),
                                    SwitchEditorMessage::PreviousUsedEditor,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Editor in Group [Ctrl+K Ctrl+PageDown]").into(),
                                    SwitchEditorMessage::NextEditorInGroup,
                                ),
                                Entry::Item(
                                    Text::new("Previous Editor in Group [Ctrl+K Ctrl+PageUp]")
                                        .into(),
                                    SwitchEditorMessage::PreviousEditorInGroup,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Used Editor in Group").into(),
                                    SwitchEditorMessage::NextUsedEditorInGroup,
                                ),
                                Entry::Item(
                                    Text::new("Previous Used Editor in Group").into(),
                                    SwitchEditorMessage::PreviousUsedEditorInGroup,
                                ),
                            ],
                        )
                        .map(GoMessage::SwitchEditor),
                        Entry::Group(
                            Text::new("Switch Group").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Group 1").into(),
                                    SwitchGroupMessage::Group1,
                                ),
                                Entry::Item(
                                    Text::new("Group 2").into(),
                                    SwitchGroupMessage::Group2,
                                ),
                                Entry::Item(
                                    Text::new("Group 3").into(),
                                    SwitchGroupMessage::Group3,
                                ),
                                Entry::Item(
                                    Text::new("Group 4").into(),
                                    SwitchGroupMessage::Group4,
                                ),
                                Entry::Item(
                                    Text::new("Group 5").into(),
                                    SwitchGroupMessage::Group5,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Next Group").into(),
                                    SwitchGroupMessage::NextGroup,
                                ),
                                Entry::Item(
                                    Text::new("Previous Group").into(),
                                    SwitchGroupMessage::PreviousGroup,
                                ),
                                Entry::Separator,
                                Entry::Item(
                                    Text::new("Group Left [Ctrl+K Ctrl+LeftArrow]").into(),
                                    SwitchGroupMessage::GroupLeft,
                                ),
                                Entry::Item(
                                    Text::new("Group Right [Ctrl+K Ctrl+RightArrow]").into(),
                                    SwitchGroupMessage::GroupRight,
                                ),
                                Entry::Item(
                                    Text::new("Group Above [Ctrl+K Ctrl+UpArrow]").into(),
                                    SwitchGroupMessage::GroupAbove,
                                ),
                                Entry::Item(
                                    Text::new("Group Below [Ctrl+K Ctrl+DownArrow]").into(),
                                    SwitchGroupMessage::GroupBelow,
                                ),
                            ],
                        )
                        .map(GoMessage::SwitchGroup),
                        Entry::Separator,
                        Entry::Item(Text::new("Go to File...").into(), GoMessage::GoToFile),
                        Entry::Item(
                            Text::new("Go to Symbol in Workspace...").into(),
                            GoMessage::GoToSymbolInWorkspace,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Go to Symbol in Editor...").into(),
                            GoMessage::GoToSymbolInEditor,
                        ),
                        Entry::Item(
                            Text::new("Go to Definition").into(),
                            GoMessage::GoToDefinition,
                        ),
                        Entry::Item(
                            Text::new("Go to Declaration").into(),
                            GoMessage::GoToDeclaration,
                        ),
                        Entry::Item(
                            Text::new("Go to Type Definition").into(),
                            GoMessage::GoToTypeDefinition,
                        ),
                        Entry::Item(
                            Text::new("Go to Implementations").into(),
                            GoMessage::GoToImplementations,
                        ),
                        Entry::Item(
                            Text::new("Go to References").into(),
                            GoMessage::GoToReferences,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Go to Line/Column...").into(),
                            GoMessage::GoToLineColumn,
                        ),
                        Entry::Item(Text::new("Go to Bracket").into(), GoMessage::GoToBracket),
                        Entry::Separator,
                        Entry::Item(Text::new("Next Problem").into(), GoMessage::NextProblem),
                        Entry::Item(
                            Text::new("Previous Problem").into(),
                            GoMessage::PreviousProblem,
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Next Change").into(), GoMessage::NextChange),
                        Entry::Item(
                            Text::new("Previous Change").into(),
                            GoMessage::PreviousChange,
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
                            RunMessage::StartDebugging,
                        ),
                        Entry::Item(
                            Text::new("Run Without Debugging").into(),
                            RunMessage::RunWithoutDebugging,
                        ),
                        Entry::Item(
                            Text::new("Stop Debugging").into(),
                            RunMessage::StopDebugging,
                        ),
                        Entry::Item(
                            Text::new("Restart Debugging").into(),
                            RunMessage::RestartDebugging,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Open Configurations").into(),
                            RunMessage::OpenConfigurations,
                        ),
                        Entry::Item(
                            Text::new("Add Configuration...").into(),
                            RunMessage::AddConfiguration,
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Step Over").into(), RunMessage::StepOver),
                        Entry::Item(Text::new("Step Into").into(), RunMessage::StepInto),
                        Entry::Item(Text::new("Step Out").into(), RunMessage::StepOut),
                        Entry::Item(Text::new("Continue").into(), RunMessage::Continue),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Breakpoint").into(),
                            RunMessage::ToggleBreakpoint,
                        ),
                        Entry::Group(
                            Text::new("New Breakpoint").into(),
                            vec![
                                Entry::Item(
                                    Text::new("Conditional Breakpoint...").into(),
                                    NewBreakpointMessage::ConditionalBreakpoint,
                                ),
                                Entry::Item(
                                    Text::new("Inline Breakpoint").into(),
                                    NewBreakpointMessage::InlineBreakpoint,
                                ),
                                Entry::Item(
                                    Text::new("Function Breakpoint...").into(),
                                    NewBreakpointMessage::FunctionBreakpoint,
                                ),
                                Entry::Item(
                                    Text::new("Logpoint...").into(),
                                    NewBreakpointMessage::Logpoint,
                                ),
                            ],
                        )
                        .map(RunMessage::NewBreakpoint),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Enable All Breakpoints").into(),
                            RunMessage::EnableAllBreakpoints,
                        ),
                        Entry::Item(
                            Text::new("Disable All Breakpoints").into(),
                            RunMessage::DisableAllBreakpoints,
                        ),
                        Entry::Item(
                            Text::new("Remove All Breakpoints").into(),
                            RunMessage::RemoveAllBreakpoints,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Install Additional Debuggers...").into(),
                            RunMessage::InstallAdditionalDebuggers,
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
                            TerminalMessage::NewTerminal,
                        ),
                        Entry::Item(
                            Text::new("Split Terminal").into(),
                            TerminalMessage::SplitTerminal,
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("Run Task...").into(), TerminalMessage::RunTask),
                        Entry::Item(
                            Text::new("Run Build Task...").into(),
                            TerminalMessage::RunBuildTask,
                        ),
                        Entry::Item(
                            Text::new("Run Active File").into(),
                            TerminalMessage::RunActiveFile,
                        ),
                        Entry::Item(
                            Text::new("Run Selected Text").into(),
                            TerminalMessage::RunSelectedText,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Show Running Tasks...").into(),
                            TerminalMessage::ShowRunningTasks,
                        ),
                        Entry::Item(
                            Text::new("Restart Running Tasks...").into(),
                            TerminalMessage::RestartRunningTasks,
                        ),
                        Entry::Item(
                            Text::new("Terminate Tasks...").into(),
                            TerminalMessage::TerminateTasks,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Configure Tasks...").into(),
                            TerminalMessage::ConfigureTasks,
                        ),
                        Entry::Item(
                            Text::new("Configure Default Build Task...").into(),
                            TerminalMessage::ConfigureDefaultBuildTasks,
                        ),
                    ],
                )
                .map(MenuMessage::Terminal),
            )
            .push(
                Section::new(
                    Text::new("Help"),
                    vec![
                        Entry::Item(Text::new("Welcome").into(), HelpMessage::Welcome),
                        Entry::Item(
                            Text::new("Interactive Playground").into(),
                            HelpMessage::InteractivePlayground,
                        ),
                        Entry::Item(
                            Text::new("Documentation").into(),
                            HelpMessage::Documentation,
                        ),
                        Entry::Item(Text::new("Release Notes").into(), HelpMessage::ReleaseNotes),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Keyboard Shortcuts Reference [Ctrl+K Ctrl+R]").into(),
                            HelpMessage::KeyboardShortCutsReference,
                        ),
                        Entry::Item(
                            Text::new("Introductory Videos").into(),
                            HelpMessage::IntroductoryVideos,
                        ),
                        Entry::Item(
                            Text::new("Tips and Tricks").into(),
                            HelpMessage::TipsAndTricks,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Join Us on Twitter").into(),
                            HelpMessage::JoinUsOnTwitter,
                        ),
                        Entry::Item(
                            Text::new("Search Feature Requests").into(),
                            HelpMessage::SearchFeatureRequests,
                        ),
                        Entry::Item(Text::new("Report Issue").into(), HelpMessage::ReportIssue),
                        Entry::Separator,
                        Entry::Item(Text::new("View License").into(), HelpMessage::ViewLicense),
                        Entry::Item(
                            Text::new("Privacy Statement").into(),
                            HelpMessage::PrivacyStatement,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Toggle Developer Tools").into(),
                            HelpMessage::ToggleDeveloperTools,
                        ),
                        Entry::Item(
                            Text::new("Open Process Explorer").into(),
                            HelpMessage::OpenProcessExplorer,
                        ),
                        Entry::Separator,
                        Entry::Item(
                            Text::new("Download Available Update").into(),
                            HelpMessage::DownloadAvailableUpdate,
                        ),
                        Entry::Separator,
                        Entry::Item(Text::new("About").into(), HelpMessage::About),
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
