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
    G = 0x000A,
    /// `h` and `H`
    H = 0x000B,
    /// `i` and `I`
    I = 0x000C,
    /// `j` and `J`
    J = 0x000D,
    /// `k` and `K`
    K = 0x000E,
    /// `l` and `L`
    L = 0x000F,
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
    W = 0x001A,
    /// `x` and `X`
    X = 0x001B,
    /// `y` and `Y`
    Y = 0x001C,
    /// `z` and `Z`
    Z = 0x001D,
    /// `1` and `!`
    Kc1 = 0x001E,
    /// `2` and `@`
    Kc2 = 0x001F,
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
    Backspace = 0x002A,
    /// `Tab`
    Tab = 0x002B,
    /// `Space`
    Space = 0x002C,
    /// `-` and `_`
    Minus = 0x002D,
    /// `=` and `+`
    Equal = 0x002E,
    /// `[` and `{`
    LeftBracket = 0x002F,
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
    F1 = 0x003A,
    /// `F2`
    F2 = 0x003B,
    /// `F3`
    F3 = 0x003C,
    /// `F4`
    F4 = 0x003D,
    /// `F5`
    F5 = 0x003E,
    /// `F6`
    F6 = 0x003F,
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
    Home = 0x004A,
    /// Page Up
    PageUp = 0x004B,
    /// Delete
    Delete = 0x004C,
    /// End
    End = 0x004D,
    /// Page Down
    PageDown = 0x004E,
    /// Right arrow
    Right = 0x004F,
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
    Kp2 = 0x005A,
    /// `3` on keypad
    Kp3 = 0x005B,
    /// `4` on keypad
    Kp4 = 0x005C,
    /// `5` on keypad
    Kp5 = 0x005D,
    /// `6` on keypad
    Kp6 = 0x005E,
    /// `7` on keypad
    Kp7 = 0x005F,
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
    F15 = 0x006A,
    /// `F16`
    F16 = 0x006B,
    /// `F17`
    F17 = 0x006C,
    /// `F18`
    F18 = 0x006D,
    /// `F19`
    F19 = 0x006E,
    /// `F20`
    F20 = 0x006F,
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
    Undo = 0x007A,
    Cut = 0x007B,
    Copy = 0x007C,
    Paste = 0x007D,
    Find = 0x007E,
    /// Mute
    KbMute = 0x007F,
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
    International4 = 0x008A,
    International5 = 0x008B,
    International6 = 0x008C,
    International7 = 0x008D,
    International8 = 0x008E,
    International9 = 0x008F,
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
    SystemRequest = 0x009A,
    Cancel = 0x009B,
    Clear = 0x009C,
    Prior = 0x009D,
    Return = 0x009E,
    Separator = 0x009F,
    Out = 0x00A0,
    Oper = 0x00A1,
    ClearAgain = 0x00A2,
    Crsel = 0x00A3,
    Exsel = 0x00A4,
    SystemPower = 0x00A5,
    SystemSleep = 0x00A6,
    SystemWake = 0x00A7,
    AudioMute = 0x00A8,
    AudioVolUp = 0x00A9,
    AudioVolDown = 0x00AA,
    MediaNextTrack = 0x00AB,
    MediaPrevTrack = 0x00AC,
    MediaStop = 0x00AD,
    MediaPlayPause = 0x00AE,
    MediaSelect = 0x00AF,
    MediaEject = 0x00B0,
    Mail = 0x00B1,
    Calculator = 0x00B2,
    MyComputer = 0x00B3,
    WwwSearch = 0x00B4,
    WwwHome = 0x00B5,
    WwwBack = 0x00B6,
    WwwForward = 0x00B7,
    WwwStop = 0x00B8,
    WwwRefresh = 0x00B9,
    WwwFavorites = 0x00BA,
    MediaFastForward = 0x00BB,
    MediaRewind = 0x00BC,
    /// Brightness Up
    BrightnessUp = 0x00BD,
    /// Brightness Down
    BrightnessDown = 0x00BE,
    ControlPanel = 0x00BF,
    Assistant = 0x00C0,
    MissionControl = 0x00C1,
    Launchpad = 0x00C2,
    /// Mouse Up
    MouseUp = 0x00CD,
    /// Mouse Down
    MouseDown = 0x00CE,
    /// Mouse Left
    MouseLeft = 0x00CF,
    /// Mouse Right
    MouseRight = 0x00D0,
    /// Mouse Button 1(Left)
    MouseBtn1 = 0x00D1,
    /// Mouse Button 2(Right)
    MouseBtn2 = 0x00D2,
    /// Mouse Button 3(Middle)
    MouseBtn3 = 0x00D3,
    /// Mouse Button 4(Back)
    MouseBtn4 = 0x00D4,
    /// Mouse Button 5(Forward)
    MouseBtn5 = 0x00D5,
    MouseBtn6 = 0x00D6,
    MouseBtn7 = 0x00D7,
    MouseBtn8 = 0x00D8,
    MouseWheelUp = 0x00D9,
    MouseWheelDown = 0x00DA,
    MouseWheelLeft = 0x00DB,
    MouseWheelRight = 0x00DC,
    MouseAccel0 = 0x00DD,
    MouseAccel1 = 0x00DE,
    MouseAccel2 = 0x00DF,
    /// Left Control
    LCtrl = 0x00E0,
    /// Left Shift
    LShift = 0x00E1,
    /// Left Alt
    LAlt = 0x00E2,
    /// Left GUI
    LGui = 0x00E3,
    /// Right Control
    RCtrl = 0x00E4,
    /// Right Shift
    RShift = 0x00E5,
    /// Right Alt
    RAlt = 0x00E6,
    /// Right GUI
    RGui = 0x00E7,
}

pub fn 網頁鍵值轉換(鍵值: &str) -> KeyCode {
    match 鍵值 {
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
