// Stolen from https://github.com/HaoboGu/rmk
// Included under The MIT License (https://github.com/HaoboGu/rmk/blob/main/LICENSE-MIT)
/// KeyCode is the internal representation of all keycodes, keyboard operations, etc.
#[allow(dead_code)]
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyCode {
    /// Reserved, no-key.
    No = 0x0000,
    /// Keyboard roll over error, too many keys are pressed simultaneously, not a physical key.
    /// NKRO: n-key rollover.
    ErrorRollover = 0x0001,
    /// Keyboard post fail error, not a physical key.
    PostFail = 0x0002,
    /// An undefined error, not a physical key.
    ErrorUndefined = 0x0003,
    /// `a` and `A`
    A = 0x0004,
    /// `b` and `B`
    B = 0x0005,
    /// `c` and `C`
    C = 0x0006,
    /// `d` and `D`
    D = 0x0007,
    /// `e` and `E`
    E = 0x0008,
    /// `f` and `F`
    F = 0x0009,
    /// `g` and `G`
    G = 0x000a,
    /// `h` and `H`
    H = 0x000b,
    /// `i` and `I`
    I = 0x000c,
    /// `j` and `J`
    J = 0x000d,
    /// `k` and `K`
    K = 0x000e,
    /// `l` and `L`
    L = 0x000f,
    /// `m` and `M`
    M = 0x0010,
    /// `n` and `N`
    N = 0x0011,
    /// `o` and `O`
    O = 0x0012,
    /// `p` and `P`
    P = 0x0013,
    /// `q` and `Q`
    Q = 0x0014,
    /// `r` and `R`
    R = 0x0015,
    /// `s` and `S`
    S = 0x0016,
    /// `t` and `T`
    T = 0x0017,
    /// `u` and `U`
    U = 0x0018,
    /// `v` and `V`
    V = 0x0019,
    /// `w` and `W`
    W = 0x001a,
    /// `x` and `X`
    X = 0x001b,
    /// `y` and `Y`
    Y = 0x001c,
    /// `z` and `Z`
    Z = 0x001d,
    /// `1` and `!`
    Kc1 = 0x001e,
    /// `2` and `@`
    Kc2 = 0x001f,
    /// `3` and `#`
    Kc3 = 0x0020,
    /// `4` and `$`
    Kc4 = 0x0021,
    /// `5` and `%`
    Kc5 = 0x0022,
    /// `6` and `^`
    Kc6 = 0x0023,
    /// `7` and `&`
    Kc7 = 0x0024,
    /// `8` and `*`
    Kc8 = 0x0025,
    /// `9` and `(`
    Kc9 = 0x0026,
    /// `0` and `)`
    Kc0 = 0x0027,
    /// `Enter`
    Enter = 0x0028,
    /// `Esc`
    Escape = 0x0029,
    /// `Backspace`
    Backspace = 0x002a,
    /// `Tab`
    Tab = 0x002b,
    /// `Space`
    Space = 0x002c,
    /// `-` and `_`
    Minus = 0x002d,
    /// `=` and `+`
    Equal = 0x002e,
    /// `[` and `{`
    LeftBracket = 0x002f,
    /// `]` and `}`
    RightBracket = 0x0030,
    /// `\` and `|`
    Backslash = 0x0031,
    /// Non-US `#` and `~`
    NonusHash = 0x0032,
    /// `;` and `:`
    Semicolon = 0x0033,
    /// `'` and `"`
    Quote = 0x0034,
    /// `~` and `\``
    Grave = 0x0035,
    /// `,` and `<`
    Comma = 0x0036,
    /// `.` and `>`
    Dot = 0x0037,
    /// `/` and `?`
    Slash = 0x0038,
    /// `CapsLock`
    CapsLock = 0x0039,
    /// `F1`
    F1 = 0x003a,
    /// `F2`
    F2 = 0x003b,
    /// `F3`
    F3 = 0x003c,
    /// `F4`
    F4 = 0x003d,
    /// `F5`
    F5 = 0x003e,
    /// `F6`
    F6 = 0x003f,
    /// `F7`
    F7 = 0x0040,
    /// `F8`
    F8 = 0x0041,
    /// `F9`
    F9 = 0x0042,
    /// `F10`
    F10 = 0x0043,
    /// `F11`
    F11 = 0x0044,
    /// `F12`
    F12 = 0x0045,
    /// Print Screen
    PrintScreen = 0x0046,
    /// Scroll Lock
    ScrollLock = 0x0047,
    /// Pause
    Pause = 0x0048,
    /// Insert
    Insert = 0x0049,
    /// Home
    Home = 0x004a,
    /// Page Up
    PageUp = 0x004b,
    /// Delete
    Delete = 0x004c,
    /// End
    End = 0x004d,
    /// Page Down
    PageDown = 0x004e,
    /// Right arrow
    Right = 0x004f,
    /// Left arrow
    Left = 0x0050,
    /// Down arrow
    Down = 0x0051,
    /// Up arrow
    Up = 0x0052,
    /// Nums Lock
    NumLock = 0x0053,
    /// `/` on keypad
    KpSlash = 0x0054,
    /// `*` on keypad
    KpAsterisk = 0x0055,
    /// `-` on keypad
    KpMinus = 0x0056,
    /// `+` on keypad
    KpPlus = 0x0057,
    /// `Enter` on keypad
    KpEnter = 0x0058,
    /// `1` on keypad
    Kp1 = 0x0059,
    /// `2` on keypad
    Kp2 = 0x005a,
    /// `3` on keypad
    Kp3 = 0x005b,
    /// `4` on keypad
    Kp4 = 0x005c,
    /// `5` on keypad
    Kp5 = 0x005d,
    /// `6` on keypad
    Kp6 = 0x005e,
    /// `7` on keypad
    Kp7 = 0x005f,
    /// `8` on keypad
    Kp8 = 0x0060,
    /// `9` on keypad
    Kp9 = 0x0061,
    /// `0` on keypad
    Kp0 = 0x0062,
    /// `.` on keypad
    KpDot = 0x0063,
    /// Non-US `\` or `|`
    NonusBackslash = 0x0064,
    /// `Application`
    Application = 0x0065,
    /// `Power`
    KbPower = 0x0066,
    /// `=` on keypad
    KpEqual = 0x0067,
    /// `F13`
    F13 = 0x0068,
    /// `F14`
    F14 = 0x0069,
    /// `F15`
    F15 = 0x006a,
    /// `F16`
    F16 = 0x006b,
    /// `F17`
    F17 = 0x006c,
    /// `F18`
    F18 = 0x006d,
    /// `F19`
    F19 = 0x006e,
    /// `F20`
    F20 = 0x006f,
    /// `F21`
    F21 = 0x0070,
    /// `F22`
    F22 = 0x0071,
    /// `F23`
    F23 = 0x0072,
    /// `F24`
    F24 = 0x0073,
    Execute = 0x0074,
    Help = 0x0075,
    Menu = 0x0076,
    Select = 0x0077,
    Stop = 0x0078,
    Again = 0x0079,
    Undo = 0x007a,
    Cut = 0x007b,
    Copy = 0x007c,
    Paste = 0x007d,
    Find = 0x007e,
    /// Mute
    KbMute = 0x007f,
    /// Volume Up
    KbVolumeUp = 0x0080,
    /// Volume Down
    KbVolumeDown = 0x0081,
    /// Locking Caps Lock
    LockingCapsLock = 0x0082,
    /// Locking Num Lock
    LockingNumLock = 0x0083,
    /// Locking scroll lock
    LockingScrollLock = 0x0084,
    KpComma = 0x0085,
    KpEqualAs400 = 0x0086,
    International1 = 0x0087,
    International2 = 0x0088,
    International3 = 0x0089,
    International4 = 0x008a,
    International5 = 0x008b,
    International6 = 0x008c,
    International7 = 0x008d,
    International8 = 0x008e,
    International9 = 0x008f,
    Language1 = 0x0090,
    Language2 = 0x0091,
    Language3 = 0x0092,
    Language4 = 0x0093,
    Language5 = 0x0094,
    Language6 = 0x0095,
    Language7 = 0x0096,
    Language8 = 0x0097,
    Language9 = 0x0098,
    AlternateErase = 0x0099,
    SystemRequest = 0x009a,
    Cancel = 0x009b,
    Clear = 0x009c,
    Prior = 0x009d,
    Return = 0x009e,
    Separator = 0x009f,
    Out = 0x00a0,
    Oper = 0x00a1,
    ClearAgain = 0x00a2,
    Crsel = 0x00a3,
    Exsel = 0x00a4,
    SystemPower = 0x00a5,
    SystemSleep = 0x00a6,
    SystemWake = 0x00a7,
    AudioMute = 0x00a8,
    AudioVolUp = 0x00a9,
    AudioVolDown = 0x00aa,
    MediaNextTrack = 0x00ab,
    MediaPrevTrack = 0x00ac,
    MediaStop = 0x00ad,
    MediaPlayPause = 0x00ae,
    MediaSelect = 0x00af,
    MediaEject = 0x00b0,
    Mail = 0x00b1,
    Calculator = 0x00b2,
    MyComputer = 0x00b3,
    WwwSearch = 0x00b4,
    WwwHome = 0x00b5,
    WwwBack = 0x00b6,
    WwwForward = 0x00b7,
    WwwStop = 0x00b8,
    WwwRefresh = 0x00b9,
    WwwFavorites = 0x00ba,
    MediaFastForward = 0x00bb,
    MediaRewind = 0x00bc,
    /// Brightness Up
    BrightnessUp = 0x00bd,
    /// Brightness Down
    BrightnessDown = 0x00be,
    ControlPanel = 0x00bf,
    Assistant = 0x00c0,
    MissionControl = 0x00c1,
    Launchpad = 0x00c2,
    /// Mouse Up
    MouseUp = 0x00cd,
    /// Mouse Down
    MouseDown = 0x00ce,
    /// Mouse Left
    MouseLeft = 0x00cf,
    /// Mouse Right
    MouseRight = 0x00d0,
    /// Mouse Button 1(Left)
    MouseBtn1 = 0x00d1,
    /// Mouse Button 2(Right)
    MouseBtn2 = 0x00d2,
    /// Mouse Button 3(Middle)
    MouseBtn3 = 0x00d3,
    /// Mouse Button 4(Back)
    MouseBtn4 = 0x00d4,
    /// Mouse Button 5(Forward)
    MouseBtn5 = 0x00d5,
    MouseBtn6 = 0x00d6,
    MouseBtn7 = 0x00d7,
    MouseBtn8 = 0x00d8,
    MouseWheelUp = 0x00d9,
    MouseWheelDown = 0x00da,
    MouseWheelLeft = 0x00db,
    MouseWheelRight = 0x00dc,
    MouseAccel0 = 0x00dd,
    MouseAccel1 = 0x00de,
    MouseAccel2 = 0x00df,
    /// Left Control
    LCtrl = 0x00e0,
    /// Left Shift
    LShift = 0x00e1,
    /// Left Alt
    LAlt = 0x00e2,
    /// Left GUI
    LGui = 0x00e3,
    /// Right Control
    RCtrl = 0x00e4,
    /// Right Shift
    RShift = 0x00e5,
    /// Right Alt
    RAlt = 0x00e6,
    /// Right GUI
    RGui = 0x00e7,
}

pub fn 网页键值转换(键值: &str) -> KeyCode {
    match 键值 {
        "KeyA" => KeyCode::A,
        "KeyB" => KeyCode::B,
        "KeyC" => KeyCode::C,
        "KeyD" => KeyCode::D,
        "KeyE" => KeyCode::E,
        "KeyF" => KeyCode::F,
        "KeyG" => KeyCode::G,
        "KeyH" => KeyCode::H,
        "KeyI" => KeyCode::I,
        "KeyJ" => KeyCode::J,
        "KeyK" => KeyCode::K,
        "KeyL" => KeyCode::L,
        "KeyM" => KeyCode::M,
        "KeyN" => KeyCode::N,
        "KeyO" => KeyCode::O,
        "KeyP" => KeyCode::P,
        "KeyQ" => KeyCode::Q,
        "KeyR" => KeyCode::R,
        "KeyS" => KeyCode::S,
        "KeyT" => KeyCode::T,
        "KeyU" => KeyCode::U,
        "KeyV" => KeyCode::V,
        "KeyW" => KeyCode::W,
        "KeyX" => KeyCode::X,
        "KeyY" => KeyCode::Y,
        "KeyZ" => KeyCode::Z,
        "Digit0" => KeyCode::Kc0,
        "Digit1" => KeyCode::Kc1,
        "Digit2" => KeyCode::Kc2,
        "Digit3" => KeyCode::Kc3,
        "Digit4" => KeyCode::Kc4,
        "Digit5" => KeyCode::Kc5,
        "Digit6" => KeyCode::Kc6,
        "Digit7" => KeyCode::Kc7,
        "Digit8" => KeyCode::Kc8,
        "Digit9" => KeyCode::Kc9,
        "Semicolon" => KeyCode::Semicolon,
        "Comma" => KeyCode::Comma,
        "Period" => KeyCode::Dot,
        "Slash" => KeyCode::Slash,
        "Backquote" => KeyCode::Grave,
        "Quote" => KeyCode::Quote,
        "Minus" => KeyCode::Minus,
        "Equal" => KeyCode::Equal,
        "BracketLeft" => KeyCode::LeftBracket,
        "BracketRight" => KeyCode::RightBracket,
        "Backslash" => KeyCode::Backslash,
        "Space" => KeyCode::Space,
        "Escape" => KeyCode::Escape,
        "Tab" => KeyCode::Tab,
        "Backspace" => KeyCode::Backspace,
        "Enter" => KeyCode::Enter,
        _ => KeyCode::No,
    }
}
