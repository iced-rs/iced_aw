# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0] - 2024-02-24
### Added
- Tab bar now follows native iced theme @Redhawk.
- Added nerd fonts @RedHawk
- Slider Bar widget.
- DropDown Widget @wiiznokes.

### Changed
- Updated `Menu` @latidoremi.
- Updated to latest iced 0.12.0.
- Updated bootstrap fonts @RedHawk.
- Updated to support latest winit changes for input.

### Fixed
- Tabs Icons and text not rendering correctly.
- use_24h not working on time picker.
- Tabs content offset not taking into consideration the Tabs height.
- Grid not rendering and aligning correctly.
- Floating Element now can use other overlay widgets inside its content.

## [0.7.0] - 2023-08-30

### Added
- DynamicHeight to menu bar @latidoremi .
- [Breaking] Custom Style Options for all widgets.
- Align Option for modal @wiiznokes .
- width setting to `NumberInput`.`

### Changed
- Changed current width to content_width for `NumberInput`.
- (Breaking) Removed `Icon_text`, Use `Iced::widget::Text` instead.

### Fixed
- TabBars hieght issue within container.
- number input buttons not rendering correctly when they are oversized.
- Fixed SFUIRounded family name to be correct.
- number input scrolling to be  normal scrolling instead of inversed scrolling @Redhawk18. 

## [0.6.0] - 2023-07-28

### Added
- Selection List now will clear Selected if a new Item is added to the same ID location as the last and its Hash is different.
- Manual Override will always be used over the internal selected if set to Some(). If the # doesn't exist it defaults Selected to None.
- Added Helper functions for Widgets.

### Changed
- Breaking Selection List now Takes in Font and Manual Selected override on new_with.
- Breaking Selection list Message Type is now Name((usize, T)) for on_select.
- Upgraded to Latest Iced 0.10.0.
- Depreciating Older Versions of Iced_aw.
- Switched lazy_static to OnceCell

### Fixed
- Floating Element Position is corrected. Original position issue was due to Center_x containing both X and Width/2.
