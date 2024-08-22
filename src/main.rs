
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::*;

fn main() {
    let current = CFRunLoop::get_current();
    let event_tap = CGEventTap::new(
        CGEventTapLocation::Session, 
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![(CGEventType::KeyDown)],
        |_a, b, d| {
            cg_event_callback(b, d.as_ref());
            None
        },
        
    ).unwrap();
    unsafe {
        let loop_source = event_tap
            .mach_port
            .create_runloop_source(0)
            .expect("failed");
        current.add_source(&loop_source, kCFRunLoopCommonModes);
        event_tap.enable();
        CFRunLoop::run_current();
    }
}

fn cg_event_callback(e_type: CGEventType, event: &CGEventRef) -> &CGEventRef {
    let mut last_flags: CGEventFlags = CGEventFlags::CGEventFlagNull;

    if (e_type as u8 != CGEventType::KeyDown as u8) && (e_type as u8 != CGEventType::FlagsChanged as u8) {
        return event;
    }

    let flags: CGEventFlags = event.to_owned().get_flags();
    let key_code: CGKeyCode = event.to_owned().get_integer_value_field(EventField::KEYBOARD_EVENT_KEYCODE) as u16;

    let mut down: bool = false;

    if e_type as u8 == CGEventType::FlagsChanged as u8 {
        match key_code {
            55 => {
                down = (flags & CGEventFlags::CGEventFlagCommand == CGEventFlags::CGEventFlagCommand) && !(last_flags & CGEventFlags::CGEventFlagCommand == CGEventFlags::CGEventFlagCommand);
            },
            60 => {
                down = (flags & CGEventFlags::CGEventFlagShift == CGEventFlags::CGEventFlagShift) && !(last_flags & CGEventFlags::CGEventFlagShift == CGEventFlags::CGEventFlagShift);  
            },
            61 => {
                down = (flags & CGEventFlags::CGEventFlagAlternate == CGEventFlags::CGEventFlagAlternate) && !(last_flags & CGEventFlags::CGEventFlagAlternate == CGEventFlags::CGEventFlagAlternate);
            },
            62 => {
                down = (flags & CGEventFlags::CGEventFlagControl == CGEventFlags::CGEventFlagControl) && !(last_flags & CGEventFlags::CGEventFlagControl == CGEventFlags::CGEventFlagControl);
            },
            57 => {
                down = (flags & CGEventFlags::CGEventFlagAlphaShift == CGEventFlags::CGEventFlagAlphaShift) && !(last_flags & CGEventFlags::CGEventFlagAlphaShift == CGEventFlags::CGEventFlagAlphaShift);
            },
            _ => {},
        };
    } else if e_type as u8 == CGEventType::KeyDown as u8 {
        down = true;
    }
    last_flags = flags;

    if !down {
        return event;
    }

    let shift: bool = flags & CGEventFlags::CGEventFlagShift == CGEventFlags::CGEventFlagShift;
    let caps: bool = flags & CGEventFlags::CGEventFlagAlphaShift == CGEventFlags::CGEventFlagAlphaShift;

    println!("keycode: {} - {}", key_code, convert_key_code(key_code, shift, caps));

    return event;
}

fn convert_key_code(key_code: CGKeyCode, shift: bool, caps: bool) -> String {
    match key_code as u8 {
        0 => {if shift || caps {return "A".to_owned()} else {return "a".to_owned()}},
        1 => {if shift || caps {return "S".to_owned()} else {return "s".to_owned()}},
        2 => {if shift || caps {return "D".to_owned()} else {return "d".to_owned()}},
        3 => {if shift || caps {return "F".to_owned()} else {return "f".to_owned()}},
        4 => {if shift || caps {return "H".to_owned()} else {return "h".to_owned()}},
        5 => {if shift || caps {return "G".to_owned()} else {return "g".to_owned()}},
        6 => {if shift || caps {return "Z".to_owned()} else {return "z".to_owned()}},
        7 => {if shift || caps {return "X".to_owned()} else {return "x".to_owned()}},
        8 => {if shift || caps {return "C".to_owned()} else {return "c".to_owned()}},
        9 => {if shift || caps {return "V".to_owned()} else {return "v".to_owned()}},
        11 => {if shift || caps {return "B".to_owned()} else {return "b".to_owned()}},
        12 => {if shift || caps {return "Q".to_owned()} else {return "q".to_owned()}},
        13 => {if shift || caps {return "W".to_owned()} else {return "w".to_owned()}},
        14 => {if shift || caps {return "E".to_owned()} else {return "e".to_owned()}},
        15 => {if shift || caps {return "R".to_owned()} else {return "r".to_owned()}},
        16 => {if shift || caps {return "Y".to_owned()} else {return "y".to_owned()}},
        17 => {if shift || caps {return "T".to_owned()} else {return "t".to_owned()}},
        18 => {if shift {return "!".to_owned()} else {return "1".to_owned()}},
        19 => {if shift {return "@".to_owned()} else {return "2".to_owned()}},
        20 => {if shift {return "#".to_owned()} else {return "3".to_owned()}},
        21 => {if shift {return "$".to_owned()} else {return "4".to_owned()}},
        22 => {if shift {return "^".to_owned()} else {return "6".to_owned()}},
        23 => {if shift {return "%".to_owned()} else {return "5".to_owned()}},
        24 => {if shift {return "+".to_owned()} else {return "=".to_owned()}},
        25 => {if shift {return "(".to_owned()} else {return "9".to_owned()}},
        26 => {if shift {return "&".to_owned()} else {return "7".to_owned()}},
        27 => {if shift {return "_".to_owned()} else {return "-".to_owned()}},
        28 => {if shift {return "*".to_owned()} else {return "8".to_owned()}},
        29 => {if shift {return ")".to_owned()} else {return "0".to_owned()}},
        30 => {if shift {return ".to_owned()}".to_owned()} else {return "]".to_owned()}},
        31 => {if shift || caps {return "O".to_owned()} else {return "o".to_owned()}},
        32 => {if shift || caps {return "U".to_owned()} else {return "u".to_owned()}},
        33 => {if shift {return "{".to_owned()} else {return "[".to_owned()}},
        34 => {if shift || caps {return "I".to_owned()} else {return "i".to_owned()}},
        35 => {if shift || caps {return "P".to_owned()} else {return "p".to_owned()}},
        37 => {if shift || caps {return "L".to_owned()} else {return "l".to_owned()}},
        38 => {if shift || caps {return "J".to_owned()} else {return "j".to_owned()}},
        39 => {if shift {return "\"".to_owned()} else {return "'".to_owned()}},
        40 => {if shift || caps {return "K".to_owned()} else {return "k".to_owned()}},
        41 => {if shift {return ":".to_owned()} else {return ";".to_owned()}},
        42 => {if shift {return "|".to_owned()} else {return "\\".to_owned()}},
        43 => {if shift {return "<".to_owned()} else {return ",".to_owned()}},
        44 => {if shift {return "?".to_owned()} else {return "/".to_owned()}},
        45 => {if shift || caps {return "N".to_owned()} else {return "n".to_owned()}},
        46 => {if shift || caps {return "M".to_owned()} else {return "m".to_owned()}},
        47 => {if shift {return ">".to_owned()} else {return ".".to_owned()}},
        50 => {if shift {return "~".to_owned()} else {return "`".to_owned()}},
        65 => {return "[decimal]".to_owned()},
        67 => {return "[asterisk]".to_owned()},
        69 => {return "[plus]".to_owned()},
        71 => {return "[clear]".to_owned()},
        75 => {return "[divide]".to_owned()},
        76 => {return "[enter]".to_owned()},
        78 => {return "[hyphen]".to_owned()},
        81 => {return "[equals]".to_owned()},
        82 => {return "0".to_owned()},
        83 => {return "1".to_owned()},
        84 => {return "2".to_owned()},
        85 => {return "3".to_owned()},
        86 => {return "4".to_owned()},
        87 => {return "5".to_owned()},
        88 => {return "6".to_owned()},
        89 => {return "7".to_owned()},
        91 => {return "8".to_owned()},
        92 => {return "9".to_owned()},
        36 => {return "[return]".to_owned()},
        48 => {return "[tab]".to_owned()},
        49 => {return " ".to_owned()},
        51 => {return "[del]".to_owned()},
        53 => {return "[esc]".to_owned()},
        54 => {return "[right-cmd]".to_owned()},
        55 => {return "[left-cmd]".to_owned()},
        56 => {return "[left-shift]".to_owned()},
        57 => {return "[caps]".to_owned()},
        58 => {return "[left-option]".to_owned()},
        59 => {return "[left-ctrl]".to_owned()},
        60 => {return "[right-shift]".to_owned()},
        61 => {return "[right-option]".to_owned()},
        62 => {return "[right-ctrl]".to_owned()},
        63 => {return "[fn]".to_owned()},
        64 => {return "[f17]".to_owned()},
        72 => {return "[volup]".to_owned()},
        73 => {return "[voldown]".to_owned()},
        74 => {return "[mute]".to_owned()},
        79 => {return "[f18]".to_owned()},
        80 => {return "[f19]".to_owned()},
        90 => {return "[f20]".to_owned()},
        96 => {return "[f5]".to_owned()},
        97 => {return "[f6]".to_owned()},
        98 => {return "[f7]".to_owned()},
        99 => {return "[f3]".to_owned()},
        100 => {return "[f8]".to_owned()},
        101 => {return "[f9]".to_owned()},
        103 => {return "[f11]".to_owned()},
        105 => {return "[f13]".to_owned()},
        106 => {return "[f16]".to_owned()},
        107 => {return "[f14]".to_owned()},
        109 => {return "[f10]".to_owned()},
        111 => {return "[f12]".to_owned()},
        113 => {return "[f15]".to_owned()},
        114 => {return "[help]".to_owned()},
        115 => {return "[home]".to_owned()},
        116 => {return "[pgup]".to_owned()},
        117 => {return "[fwddel]".to_owned()},
        118 => {return "[f4]".to_owned()},
        119 => {return "[end]".to_owned()},
        120 => {return "[f2]".to_owned()},
        121 => {return "[pgdown]".to_owned()},
        122 => {return "[f1]".to_owned()},
        123 => {return "[left]".to_owned()},
        124 => {return "[right]".to_owned()},
        125 => {return "[down]".to_owned()},
        126 => {return "[up]".to_owned()},
        _ => {return "[unknown]".to_owned()},
    };
}