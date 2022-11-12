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
iced = "0.4.2"
iced_aw = { version = "0.2", default-features = false, features = [...] }
```

## Versioning

| `iced` version | `iced_aw` version |
| -------------- | ----------------- |
| 0.4            | 0.2               |

## Widgets


### Badge

<center>

![Badge showcase](./images/showcase/badge.png)

</center>

Please take a look into our examples on how to use badges.

Enable this widget with the feature `badge`.
To enable predefined styles, enable the feature `colors`.

### Card

<center>

![Card showcase](./images/showcase/card.png)

</center>


Please take a look into our examples on how to use cards.

Enable this widget with the feature `card`.
To enable predefined styles, enable the feature `colors`.

### Color Picker

<center>

![Color Picker showcase](./images/showcase/color_picker.png)

</center>

Please take a look into our examples on how to use color pickers.

Enable this widget with the feature `color_picker`.

### Date Picker

<center>

![Modal showcase](./images/showcase/date_picker.png)

</center>

Please take a look into our examples on how to use date pickers.

Enable this widget with the feature `date_picker`.

### Floating Action Button

<center>

![Floating Element showcase](./images/showcase/floating_button.png)

</center>

Please take a look into our examples on how to use floating elements.

Enable this widget with the feature `floating_element`.
To enable predefined styles for buttons, enable the feature `colors`.

### Modal

Modals are useful for showing some content as an overlay on top. In combination with the Card widget, modals can be used to create some kind of dialog panels.

<center>

![Modal showcase](./images/showcase/modal.png)

</center>


Please take a look into our examples on how to use modals.

Enable this widget with the feature `modal`.
To enable predefined styles, enable the feature `colors`.

### NumberInput

Just like TextInput, but only for numbers.

<center>

![NumberInput showcase](./images/showcase/number_input.png)

</center>

Please take a look into our examples on how to use number inputs.

Enable this widget with the feature `number_input`.

*This widget does currently not support web*

### Split

A split divides the available space to display two different elements.

<center>

![Split showcase](./images/showcase/split_example.gif)

</center>

Please take a look into our examples on how to use Splits.

Enable Splits with the feature `split`.

*This widget is currently not supporting web*

### TabBar and Tabs

<center>

![Tabs showcase](./images/showcase/tabs_example.gif)

</center>

Please take a look into our examples on how to use TabBars and Tabs.

Enable TabBars with the feature `tab_bar` and Tabs with `tabs`.

### Time Picker

<center>

![Modal showcase](./images/showcase/time_picker.png)

</center>

Please take a look into our examples on how to use time pickers.

Enable this widget with the feature `time_picker`.



## Quickstart features

Quickstart features are pretty handy to start and experiment having everything like colors or icons available. Nevertheless, it is recommended to disable these features once the GUI is ready for production and to only include the things you really need.

### Color palette

This crate adds a predefined color palette based on the [CSS color palette](https://www.w3schools.com/cssref/css_colors.asp).

Enable colors with the feature `colors`.

### Bootstrap icons

Thanks to [Bootstrap](https://icons.getbootstrap.com), iced_aw now contains ~1,200 icons to be used in an Iced GUI.

Enable icons with the feature `icons`.

*Note: the icon font with ~1,200 weights around 0.274 MB. This features should only be used for experimenting with all the icons.*
