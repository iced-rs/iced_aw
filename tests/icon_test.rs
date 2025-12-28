//! Test to print icon Unicode values

use iced_aw::iced_aw_font::advanced_text::{cancel, down_open, left_open, ok, right_open, up_open};

#[test]
fn print_all_icon_values() {
    let icons = vec![
        ("cancel", cancel()),
        ("down_open", down_open()),
        ("left_open", left_open()),
        ("ok", ok()),
        ("right_open", right_open()),
        ("up_open", up_open()),
    ];

    println!("\n=== All Icon Unicode Values ===");
    println!("Total icons: {}", icons.len());
    println!();

    for (name, (content, _, _)) in icons {
        if let Some(c) = content.chars().next() {
            println!("{:<15} → \\u{{{:x}}} (U+{:04X})", name, c as u32, c as u32);
        }
    }

    println!("================================\n");
}

#[test]
fn print_icon_values() {
    let (cancel_str, _, _) = cancel();
    let (left_str, _, _) = left_open();
    let (ok_str, _, _) = ok();
    let (right_str, _, _) = right_open();

    println!("\n=== Icon Unicode Values ===");

    if let Some(c) = cancel_str.chars().next() {
        println!("cancel: \\u{{{:x}}} (char code: U+{:04X})", c as u32, c as u32);
    }

    if let Some(c) = left_str.chars().next() {
        println!("left_open: \\u{{{:x}}} (char code: U+{:04X})", c as u32, c as u32);
    }

    if let Some(c) = ok_str.chars().next() {
        println!("ok: \\u{{{:x}}} (char code: U+{:04X})", c as u32, c as u32);
    }

    if let Some(c) = right_str.chars().next() {
        println!("right_open: \\u{{{:x}}} (char code: U+{:04X})", c as u32, c as u32);
    }

    println!("===========================\n");
}
