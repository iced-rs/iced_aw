# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes
- Split removed in favor of Iced pane grid
- Modal and Floating element removed in favor of Iced Stack.
- Segmented Button Removed use iced button. 
- cupertino Removed as we are not going to support these anymore.

## [0.9.3] - 2024-05-08

### Fixed
- pub mod required being renamed to pub mod bootstrap via cfg.

## [0.9.2] - 2024-05-08

### Fixed
- missing Bootstrap

## [0.9.1] - 2024-05-07

### Added
- ability to disable scroll events for NumberInput. @airblast-dev 

### Fixed
- number input over/underflow when T is set to Max and Min and T is signed.

## [0.9.0] - 2024-05-07

### Added
- font loading example @Redhawk18
- Add font size to date_picker @Strosel

### Changed
- Replace Length with impl Into<Length> in width/height APIs @TitouanReal
- use bounds directly for segmented_button @spamviech
- Updated bootstrap fonts @RedHawk.
- Migrated SF_UI fonts under extended fonts type removing cupertino feature requirement to use them.

### Fixed
- number input over/underflow @TitouanReal
- Numberinput fixes & shortcut support @the-marenga
- hide all fill_quad-calls behind intersects(viewport) or width/height>0 checks @spamviech
- add missing set_active_tab @spamviech
- button doesn't highlight when it is hovered @Sherlock-Holo
- broken grid generic parameters @Exidex
- nerd font not being loaded by using font family name @Redhawk18
- font Icon enums @Redhawk18
- modal overlay fixed
- fixed tabs

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
