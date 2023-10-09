use bevy::prelude::{KeyCode, Input, Resource};
use bevy_asset::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyActionDuo {
    action: String,
    key_code: String,
}

#[derive(Debug, Clone)]
pub struct KeyBind {
    pub action: String,
    pub key_code: KeyCode,
}

#[derive(Resource, Debug)]
pub struct KeyBinds {
    pub defaults: Vec<KeyBind>, // Default values
    pub binds: Vec<KeyBind>,    // Assigned values
    pub await_update: i128,     // If user selected a bind to update in options, it will be placed here.
}


impl KeyBinds {
    // Init
    pub fn new(defaults: Vec<KeyBind>) -> Self {
        let binds: Vec<KeyBind> = defaults.clone();
        
        Self {
            defaults,
            binds,
            await_update: -1,
        }
    }

    pub fn new_with_config(defaults: Vec<KeyBind>, vector: &Vec<KeyActionDuo>) -> Self {
        let pre_binds: Vec<KeyBind> = defaults.clone();
        let mut binds: Vec<KeyBind> = Vec::new();

        let has_duplicates = pre_binds.iter().any(|main1| {
            pre_binds.iter().filter(|main2| main2.action == main1.action).count() > 1
        });

        if has_duplicates {
            panic!("Duplicate action detected.");
        }

        // Last KeyActionDuo overrides previous if they have the same action.
        for val in vector.iter() {
            for v in pre_binds.iter() {
                let val_key_code_ind = keycode_to_usize(v.key_code);
                match val_key_code_ind {
                    Ok(_) => {
                        if v.action == val.action && keycode_to_string(v.key_code).unwrap() != val.key_code {
                            // There IS a used action. Remove the default and replace it with the new.
                            binds.push(KeyBind {
                                action: val.action.clone(),
                                key_code: string_to_keycode(val.key_code.clone()).unwrap(),
                            });
                        } else if v.action == val.action && keycode_to_string(v.key_code).unwrap() == val.key_code {
                            binds.push(v.clone());
                        }
                    },
                    Err(_) => { panic!("KeyCode {:?} does not exist.", v.key_code); }
                }
            }
        }

        Self {
            defaults,
            binds,
            await_update: -1,
        }
    }

    // Names
    pub fn get_keycode_name(&self, key_code: KeyCode) -> &'static str {
        let nbr: usize = key_code as usize;
        return INPUT_NAMES[nbr];
    }

    // Main Interactions
    pub fn _just_pressed(&self, input:  &Input<KeyCode>, name: &str) -> bool {
        let opt_index = self.binds.iter().find(|x| x.action == name);
        match opt_index {
            Some(index) => {
                input.just_pressed(index.key_code)
            }
            None => {panic!("Unrecognized action {}", name); },
        }
    }
    pub fn pressed(&self, input:  &Input<KeyCode>, name: &str) -> bool {
        let opt_index = self.binds.iter().find(|x| x.action == name);
        match opt_index {
            Some(index) => {
                input.pressed(index.key_code)
            }
            None => {panic!("Unrecognized action {}", name); },
        }
    }
    pub fn _just_released(&self, input:  &Input<KeyCode>, name: &str) -> bool {
        let opt_index = self.binds.iter().find(|x| x.action == name);
        match opt_index {
            Some(index) => {
                input.just_released(index.key_code)
            }
            None => {panic!("Unrecognized action {}", name); },
        }
    }

    // Other interactions
    pub fn keybinds_to_keyactionduo(&self) -> Vec<KeyActionDuo> {
        let mut duos: Vec<KeyActionDuo> = Vec::new();

        for key in self.binds.iter() {
            duos.push(KeyActionDuo { action: key.action.clone(), key_code: String::from(self.get_keycode_name(key.key_code)) })
        }

        duos
    }

    pub fn reset(&mut self) {
        let binds: Vec<KeyBind> = self.defaults.clone(); 

        self.binds = binds;
    }
}

fn string_to_keycode(code: String) -> Result<KeyCode, Error> {
    for (key, &value) in INPUT_NAMES.iter().enumerate() {
        if value == code {
            return usize_to_keycode(key)
        }
    }
    Err(Error::msg(format!("[!]: Code for {:?} not found", code)))
}

pub fn keycode_to_string(code: KeyCode) -> Result<String, Error> {
    match keycode_to_usize(code) {
        Ok(x) => {
            Ok(INPUT_NAMES[x].to_string())
        },
        Err(x) => Err(x),
    }
}

// I hate it.
fn usize_to_keycode(key: usize) -> Result<KeyCode, Error> {
    match key {
    0 => {Ok(KeyCode::Key1) },
    1 => {Ok(KeyCode::Key2) },
    2 => {Ok(KeyCode::Key3) },
    3 => {Ok(KeyCode::Key4) },
    4 => {Ok(KeyCode::Key5) },
    5 => {Ok(KeyCode::Key6) },
    6 => {Ok(KeyCode::Key7) },
    7 => {Ok(KeyCode::Key8) },
    8 => {Ok(KeyCode::Key9) },
    9 => {Ok(KeyCode::Key0) },
    10 => {Ok(KeyCode::A) },
    11 => {Ok(KeyCode::B) },
    12 => {Ok(KeyCode::C) },
    13 => {Ok(KeyCode::D) },
    14 => {Ok(KeyCode::E) },
    15 => {Ok(KeyCode::F) },
    16 => {Ok(KeyCode::G) },
    17 => {Ok(KeyCode::H) },
    18 => {Ok(KeyCode::I) },
    19 => {Ok(KeyCode::J) },
    20 => {Ok(KeyCode::K) },
    21 => {Ok(KeyCode::L) },
    22 => {Ok(KeyCode::M) },
    23 => {Ok(KeyCode::N) },
    24 => {Ok(KeyCode::O) },
    25 => {Ok(KeyCode::P) },
    26 => {Ok(KeyCode::Q) },
    27 => {Ok(KeyCode::R) },
    28 => {Ok(KeyCode::S) },
    29 => {Ok(KeyCode::T) },
    30 => {Ok(KeyCode::U) },
    31 => {Ok(KeyCode::V) },
    32 => {Ok(KeyCode::W) },
    33 => {Ok(KeyCode::X) },
    34 => {Ok(KeyCode::Y) },
    35 => {Ok(KeyCode::Z) },
    36 => {Ok(KeyCode::Escape) },
    37 => {Ok(KeyCode::F1) },
    38 => {Ok(KeyCode::F2) },
    39 => {Ok(KeyCode::F3) },
    40 => {Ok(KeyCode::F4) },
    41 => {Ok(KeyCode::F5) },
    42 => {Ok(KeyCode::F6) },
    43 => {Ok(KeyCode::F7) },
    44 => {Ok(KeyCode::F8) },
    45 => {Ok(KeyCode::F9) },
    46 => {Ok(KeyCode::F10) },
    47 => {Ok(KeyCode::F11) },
    48 => {Ok(KeyCode::F12) },
    49 => {Ok(KeyCode::F13) },
    50 => {Ok(KeyCode::F14) },
    51 => {Ok(KeyCode::F15) },
    52 => {Ok(KeyCode::F16) },
    53 => {Ok(KeyCode::F17) },
    54 => {Ok(KeyCode::F18) },
    55 => {Ok(KeyCode::F19) },
    56 => {Ok(KeyCode::F20) },
    57 => {Ok(KeyCode::F21) },
    58 => {Ok(KeyCode::F22) },
    59 => {Ok(KeyCode::F23) },
    60 => {Ok(KeyCode::F24) },
    61 => {Ok(KeyCode::Snapshot) },
    62 => {Ok(KeyCode::Scroll) },
    63 => {Ok(KeyCode::Pause) },
    64 => {Ok(KeyCode::Insert) },
    65 => {Ok(KeyCode::Home) },
    66 => {Ok(KeyCode::Delete) },
    67 => {Ok(KeyCode::End) },
    68 => {Ok(KeyCode::PageDown) },
    69 => {Ok(KeyCode::PageUp) },
    70 => {Ok(KeyCode::Left) },
    71 => {Ok(KeyCode::Up) },
    72 => {Ok(KeyCode::Right) },
    73 => {Ok(KeyCode::Down) },
    74 => {Ok(KeyCode::Back) },
    75 => {Ok(KeyCode::Return) },
    76 => {Ok(KeyCode::Space) },
    77 => {Ok(KeyCode::Compose) },
    78 => {Ok(KeyCode::Caret) },
    79 => {Ok(KeyCode::Numlock) },
    80 => {Ok(KeyCode::Numpad0) },
    81 => {Ok(KeyCode::Numpad1) },
    82 => {Ok(KeyCode::Numpad2) },
    83 => {Ok(KeyCode::Numpad3) },
    84 => {Ok(KeyCode::Numpad4) },
    85 => {Ok(KeyCode::Numpad5) },
    86 => {Ok(KeyCode::Numpad6) },
    87 => {Ok(KeyCode::Numpad7) },
    88 => {Ok(KeyCode::Numpad8) },
    89 => {Ok(KeyCode::Numpad9) },
    90 => {Ok(KeyCode::AbntC1) },
    91 => {Ok(KeyCode::AbntC2) },
    92 => {Ok(KeyCode::NumpadAdd) },
    93 => {Ok(KeyCode::Apostrophe) },
    94 => {Ok(KeyCode::Apps) },
    95 => {Ok(KeyCode::Asterisk) },
    96 => {Ok(KeyCode::Plus) },
    97 => {Ok(KeyCode::At) },
    98 => {Ok(KeyCode::Ax) },
    99 => {Ok(KeyCode::Backslash) },
    100 => {Ok(KeyCode::Calculator) },
    101 => {Ok(KeyCode::Capital) },
    102 => {Ok(KeyCode::Colon) },
    103 => {Ok(KeyCode::Comma) },
    104 => {Ok(KeyCode::Convert) },
    105 => {Ok(KeyCode::NumpadDecimal) },
    106 => {Ok(KeyCode::NumpadDivide) },
    107 => {Ok(KeyCode::Equals) },
    108 => {Ok(KeyCode::Grave) },
    109 => {Ok(KeyCode::Kana) },
    110 => {Ok(KeyCode::Kanji) },
    111 => {Ok(KeyCode::LAlt) },
    112 => {Ok(KeyCode::LBracket) },
    113 => {Ok(KeyCode::LControl) },
    114 => {Ok(KeyCode::LShift) },
    115 => {Ok(KeyCode::LWin) },
    116 => {Ok(KeyCode::Mail) },
    117 => {Ok(KeyCode::MediaSelect) },
    118 => {Ok(KeyCode::MediaStop) },
    119 => {Ok(KeyCode::Minus) },
    120 => {Ok(KeyCode::NumpadMultiply) },
    121 => {Ok(KeyCode::Mute) },
    122 => {Ok(KeyCode::MyComputer) },
    123 => {Ok(KeyCode::NavigateForward) },
    124 => {Ok(KeyCode::NavigateBackward) },
    125 => {Ok(KeyCode::NextTrack) },
    126 => {Ok(KeyCode::NoConvert) },
    127 => {Ok(KeyCode::NumpadComma) },
    128 => {Ok(KeyCode::NumpadEnter) },
    129 => {Ok(KeyCode::NumpadEquals) },
    130 => {Ok(KeyCode::Oem102) },
    131 => {Ok(KeyCode::Period) },
    132 => {Ok(KeyCode::PlayPause) },
    133 => {Ok(KeyCode::Power) },
    134 => {Ok(KeyCode::PrevTrack) },
    135 => {Ok(KeyCode::RAlt) },
    136 => {Ok(KeyCode::RBracket) },
    137 => {Ok(KeyCode::RControl) },
    138 => {Ok(KeyCode::RShift) },
    139 => {Ok(KeyCode::RWin) },
    140 => {Ok(KeyCode::Semicolon) },
    141 => {Ok(KeyCode::Slash) },
    142 => {Ok(KeyCode::Sleep) },
    143 => {Ok(KeyCode::Stop) },
    144 => {Ok(KeyCode::NumpadSubtract) },
    145 => {Ok(KeyCode::Sysrq) },
    146 => {Ok(KeyCode::Tab) },
    147 => {Ok(KeyCode::Underline) },
    148 => {Ok(KeyCode::Unlabeled) },
    149 => {Ok(KeyCode::VolumeDown) },
    150 => {Ok(KeyCode::VolumeUp) },
    151 => {Ok(KeyCode::Wake) },
    152 => {Ok(KeyCode::WebBack) },
    153 => {Ok(KeyCode::WebFavorites) },
    154 => {Ok(KeyCode::WebForward) },
    155 => {Ok(KeyCode::WebHome) },
    156 => {Ok(KeyCode::WebRefresh) },
    157 => {Ok(KeyCode::WebSearch) },
    158 => {Ok(KeyCode::WebStop) },
    159 => {Ok(KeyCode::Yen) },
    160 => {Ok(KeyCode::Copy) },
    161 => {Ok(KeyCode::Paste) },
    162 => {Ok(KeyCode::Cut) },
    _ => { Err(Error::msg(format!("[!]: KeyBinds of code {:?} does not exist", key)))},
    }
}

// I hate it too.
fn keycode_to_usize(key: KeyCode) -> Result<usize, Error> {
    match key {
        KeyCode::Key1 => {Ok(0)},
        KeyCode::Key2 => {Ok(1)},
        KeyCode::Key3 => {Ok(2)},
        KeyCode::Key4 => {Ok(3)},
        KeyCode::Key5 => {Ok(4)},
        KeyCode::Key6 => {Ok(5)},
        KeyCode::Key7 => {Ok(6)},
        KeyCode::Key8 => {Ok(7)},
        KeyCode::Key9 => {Ok(8)},
        KeyCode::Key0 => {Ok(9)},
        KeyCode::A => {Ok(10)},
        KeyCode::B => {Ok(11)},
        KeyCode::C => {Ok(12)},
        KeyCode::D => {Ok(13)},
        KeyCode::E => {Ok(14)},
        KeyCode::F => {Ok(15)},
        KeyCode::G => {Ok(16)},
        KeyCode::H => {Ok(17)},
        KeyCode::I => {Ok(18)},
        KeyCode::J => {Ok(19)},
        KeyCode::K => {Ok(20)},
        KeyCode::L => {Ok(21)},
        KeyCode::M => {Ok(22)},
        KeyCode::N => {Ok(23)},
        KeyCode::O => {Ok(24)},
        KeyCode::P => {Ok(25)},
        KeyCode::Q => {Ok(26)},
        KeyCode::R => {Ok(27)},
        KeyCode::S => {Ok(28)},
        KeyCode::T => {Ok(29)},
        KeyCode::U => {Ok(30)},
        KeyCode::V => {Ok(31)},
        KeyCode::W => {Ok(32)},
        KeyCode::X => {Ok(33)},
        KeyCode::Y => {Ok(34)},
        KeyCode::Z => {Ok(35)},
        KeyCode::Escape => {Ok(36)},
        KeyCode::F1 => {Ok(37)},    
        KeyCode::F2 => {Ok(38)},
        KeyCode::F3 => {Ok(39)},
        KeyCode::F4 => {Ok(40)},
        KeyCode::F5 => {Ok(41)},
        KeyCode::F6 => {Ok(42)},
        KeyCode::F7 => {Ok(43)},
        KeyCode::F8 => {Ok(44)},
        KeyCode::F9 => {Ok(45)},
        KeyCode::F10 => {Ok(46)},
        KeyCode::F11 => {Ok(47)},
        KeyCode::F12 => {Ok(48)},
        KeyCode::F13 => {Ok(49)},
        KeyCode::F14 => {Ok(50)},
        KeyCode::F15 => {Ok(51)},
        KeyCode::F16 => {Ok(52)},
        KeyCode::F17 => {Ok(53)},
        KeyCode::F18 => {Ok(54)},
        KeyCode::F19 => {Ok(55)},
        KeyCode::F20 => {Ok(56)},
        KeyCode::F21 => {Ok(57)},
        KeyCode::F22 => {Ok(58)},
        KeyCode::F23 => {Ok(59)},
        KeyCode::F24 => {Ok(60)},
        KeyCode::Snapshot => {Ok(61)},
        KeyCode::Scroll => {Ok(62)},
        KeyCode::Pause => {Ok(63)},
        KeyCode::Insert => {Ok(64)},
        KeyCode::Home => {Ok(65)},
        KeyCode::Delete => {Ok(66)},
        KeyCode::End => {Ok(67)},
        KeyCode::PageDown => {Ok(68)},
        KeyCode::PageUp => {Ok(69)},
        KeyCode::Left => {Ok(70)},
        KeyCode::Up => {Ok(71)},
        KeyCode::Right => {Ok(72)},
        KeyCode::Down => {Ok(73)},
        KeyCode::Back => {Ok(74)},
        KeyCode::Return => {Ok(75)},
        KeyCode::Space => {Ok(76)},
        KeyCode::Compose => {Ok(77)},
        KeyCode::Caret => {Ok(78)},
        KeyCode::Numlock => {Ok(79)},
        KeyCode::Numpad0 => {Ok(80)},
        KeyCode::Numpad1 => {Ok(81)},
        KeyCode::Numpad2 => {Ok(82)},
        KeyCode::Numpad3 => {Ok(83)},
        KeyCode::Numpad4 => {Ok(84)},
        KeyCode::Numpad5 => {Ok(85)},
        KeyCode::Numpad6 => {Ok(86)},
        KeyCode::Numpad7 => {Ok(87)},
        KeyCode::Numpad8 => {Ok(88)},
        KeyCode::Numpad9 => {Ok(89)},
        KeyCode::AbntC1 => {Ok(90)},
        KeyCode::AbntC2 => {Ok(91)},
        KeyCode::NumpadAdd => {Ok(92)},
        KeyCode::Apostrophe => {Ok(93)},
        KeyCode::Apps => {Ok(94)},
        KeyCode::Asterisk => {Ok(95)},
        KeyCode::Plus => {Ok(96)},
        KeyCode::At => {Ok(97)},
        KeyCode::Ax => {Ok(98)},
        KeyCode::Backslash => {Ok(99)},
        KeyCode::Calculator => {Ok(100)}
        KeyCode::Capital => {Ok(101)},
        KeyCode::Colon => {Ok(102)},
        KeyCode::Comma => {Ok(103)},
        KeyCode::Convert => {Ok(104)},
        KeyCode::NumpadDecimal => {Ok(105)},
        KeyCode::NumpadDivide => {Ok(106)},
        KeyCode::Equals => {Ok(107)},
        KeyCode::Grave => {Ok(108)},
        KeyCode::Kana => {Ok(109)},
        KeyCode::Kanji => {Ok(110)},
        KeyCode::LAlt => {Ok(111)},
        KeyCode::LBracket => {Ok(112)},
        KeyCode::LControl => {Ok(113)},
        KeyCode::LShift => {Ok(114)},
        KeyCode::LWin => {Ok(115)},
        KeyCode::Mail => {Ok(116)},
        KeyCode::MediaSelect => {Ok(117)},
        KeyCode::MediaStop => {Ok(118)},
        KeyCode::Minus => {Ok(119)},
        KeyCode::NumpadMultiply => {Ok(120)},
        KeyCode::Mute => {Ok(121)},
        KeyCode::MyComputer => {Ok(122)},
        KeyCode::NavigateForward => {Ok(123)},
        KeyCode::NavigateBackward => {Ok(124)},
        KeyCode::NextTrack => {Ok(125)},
        KeyCode::NoConvert => {Ok(126)},
        KeyCode::NumpadComma => {Ok(127)},
        KeyCode::NumpadEnter => {Ok(128)},
        KeyCode::NumpadEquals => {Ok(129)},
        KeyCode::Oem102 => {Ok(130)},
        KeyCode::Period => {Ok(131)},
        KeyCode::PlayPause => {Ok(132)},
        KeyCode::Power => {Ok(133)},
        KeyCode::PrevTrack => {Ok(134)},
        KeyCode::RAlt => {Ok(135)},
        KeyCode::RBracket => {Ok(136)},
        KeyCode::RControl => {Ok(137)},
        KeyCode::RShift => {Ok(138)},
        KeyCode::RWin => {Ok(139)},
        KeyCode::Semicolon => {Ok(140)},
        KeyCode::Slash => {Ok(141)},
        KeyCode::Sleep => {Ok(142)},
        KeyCode::Stop => {Ok(143)},
        KeyCode::NumpadSubtract => {Ok(144)},
        KeyCode::Sysrq => {Ok(145)},
        KeyCode::Tab => {Ok(146)},
        KeyCode::Underline => {Ok(147)},
        KeyCode::Unlabeled => {Ok(148)},
        KeyCode::VolumeDown => {Ok(149)},
        KeyCode::VolumeUp => {Ok(150)},
        KeyCode::Wake => {Ok(151)},
        KeyCode::WebBack => {Ok(152)},
        KeyCode::WebFavorites => {Ok(153)},
        KeyCode::WebForward => {Ok(154)},
        KeyCode::WebHome => {Ok(155)},
        KeyCode::WebRefresh => {Ok(156)},
        KeyCode::WebSearch => {Ok(157)},
        KeyCode::WebStop => {Ok(158)},
        KeyCode::Yen => {Ok(159)},
        KeyCode::Copy => {Ok(160)},
        KeyCode::Paste => {Ok(161)},
        KeyCode::Cut => {Ok(162)},
        _ => {Err(Error::msg(format!("[!]: action of code {:?} has no associated BindNames", key)))},
    }
}

pub const INPUT_NAMES: &'static [&'static str] = &[
    "Key1",
    "Key2",
    "Key3",
    "Key4",
    "Key5",
    "Key6",
    "Key7",
    "Key8",
    "Key9",
    "Key0",

    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",

    "Escape",

    "F1",
    "F2",
    "F3",
    "F4",
    "F5",
    "F6",
    "F7",
    "F8",
    "F9",
    "F10",
    "F11",
    "F12",
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",

    "Snapshot",
    "Scroll",
    "Pause",

    "Insert",
    "Home",
    "Delete",
    "End",
    "PageDown",
    "PageUp",

    "Left",
    "Up",
    "Right",
    "Down",

    "Back",
    "Return",
    "Space",

    "Compose",
    "Caret",

    "Numlock",
    "Numpad0",
    "Numpad1",
    "Numpad2",
    "Numpad3",
    "Numpad4",
    "Numpad5",
    "Numpad6",
    "Numpad7",
    "Numpad8",
    "Numpad9",

    "AbntC1",
    "AbntC2",

    "NumpadAdd",
    "Apostrophe",
    "Apps",
    "Asterisk",
    "Plus",
    "At",
    "Ax",
    "Backslash",
    "Calculator",
    "Capital",
    "Colon",
    "Comma",
    "Convert",
    "NumpadDecimal",
    "NumpadDivide",
    "Equals",
    "Grave",
    "Kana",
    "Kanji",

    "LAlt",
    "LBracket",
    "LControl",
    "LShift",
    "LWin",

    "Mail",
    "MediaSelect",
    "MediaStop",
    "Minus",
    "NumpadMultiply",
    "Mute",
    "MyComputer",
    "NavigateForward",
    "NavigateBackward",
    "NextTrack",
    "NoConvert",
    "NumpadComma",
    "NumpadEnter",
    "NumpadEquals",
    "Oem102",
    "Period",
    "PlayPause",
    "Power",
    "PrevTrack",

    "RAlt",
    "RBracket",
    "RControl",
    "RShift",
    "RWin",

    "Semicolon",
    "Slash",
    "Sleep",
    "Stop",
    "NumpadSubtract",
    "Sysrq",
    "Tab",
    "Underline",
    "Unlabeled",

    "VolumeDown",
    "VolumeUp",

    "Wake",

    "WebBack",
    "WebFavorites",
    "WebForward",
    "WebHome",
    "WebRefresh",
    "WebSearch",
    "WebStop",

    "Yen",

    "Copy",
    "Paste",
    "Cut",
];