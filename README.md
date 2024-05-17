# Iced - Additional Widgets

[![Documentation](https://docs.rs/iced_aw/badge.svg)](https://docs.rs/iced_aw)
[![Crates.io](https://img.shields.io/crates/v/iced_aw.svg)](https://crates.io/crates/iced)
[![License](https://img.shields.io/crates/l/iced_aw.svg)](https://github.com/iced-rs/iced/blob/master/LICENSE)

This crate contains additional widgets for the GUI library [Iced](https://github.com/hecrj/iced).

Every widget is hidden by a feature gate. This allows you to cherry pick the widgets you actually need.

## Usage:

Include `iced_aw` as a dependency in your `Cargo.toml`:
```toml
[dependencies]
iced = "0.12.0"
iced_aw = { version = "0.9.3", default-features = false, features = [...] }
```

## Versioning

| `iced` version | `iced_aw` version |
| -------------- | ----------------- |
| 0.8            | 0.4               |
| 0.9            | 0.5               |
| 0.10           | 0.6, 0.7          |
| 0.12           | 0.8, 0.9          |

## Widgets

### Badge

<div align="center">
  
  ![Badge showcase](./images/showcase/badge.png)
</div>

Please take a look into our examples on how to use badges.

Enable this widget with the feature `badge`.

### Card

<div align="center">
  
![Card showcase](./images/showcase/card.png)
</div>

Please take a look into our examples on how to use cards.

Enable this widget with the feature `card`.

### Color Picker

<div align="center">

![Color Picker showcase](./images/showcase/color_picker.png)
</div>

Please take a look into our examples on how to use color pickers.

Enable this widget with the feature `color_picker`.

### Date Picker

<div align="center">
  
  ![Date Picker showcase](./images/showcase/date_picker.png)
</div>

Please take a look into our examples on how to use date pickers.

Enable this widget with the feature `date_picker`.

### NumberInput

Just like TextInput, but only for numbers.

<div align="center">

![NumberInput showcase](./images/showcase/number_input.png)
</div>

Please take a look into our examples on how to use number inputs.

Enable this widget with the feature `number_input`.

*This widget does currently not support web*

### SelectionList

A selection space to show any options passed in.

<div align="center">

![SelectionList showcase](./images/showcase/selection_list.gif)
</div>

Enable this widget with the feature `selection_list`.

### TabBar and Tabs

<div align="center">

![Tabs showcase](./images/showcase/tabs_example.gif)
</div>

Please take a look into our examples on how to use TabBars and Tabs.

Enable TabBars with the feature `tab_bar` and Tabs with `tabs`.

### Time Picker

<div align="center">

![Modal showcase](./images/showcase/time_picker.png)
</div>

Please take a look into our examples on how to use time pickers.

Enable this widget with the feature `time_picker`.

### Menu

<div align="center">

![Menu showcase](./images/showcase/menu.gif)
</div>

Please take a look into our examples on how to use menus.

Enable this widget with the feature `menu`.

You might also want to enable the feature `quad` for drawing separators.

### Slide Bar

Please take a look into our examples on how to use slidebars.

Enable this widget with the feature `slide_bar`.

### Context Menu

See the example [here](./examples/context_menu/src/main.rs)

### Drop Down Menu

See the example [here](./examples/drop_down/src/main.rs)

## Quickstart features

Quickstart features are pretty handy to start and experiment having everything like colors or icons available. Nevertheless, it is recommended to disable these features once the GUI is ready for production and to only include the things you really need.

### Color palette

This crate adds a predefined color palette based on the [CSS color palette](https://www.w3schools.com/cssref/css_colors.asp).

### Bootstrap icons

Thanks to [Bootstrap](https://icons.getbootstrap.com), iced_aw now contains ~1,200 icons to be used in an Iced GUI.

Enable icons with the feature `icons`.

*Note: the icon font with ~1,200 weights around 0.274 MB. This features should only be used for experimenting with all the icons.*
