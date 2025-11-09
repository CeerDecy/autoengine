use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct QuickInputMacro {
    /// Character set used when saving, typically `"UTF8"`.
    pub document_charset: String,
    /// Source application identifier, usually `"QuickInput"`.
    pub app: String,
    /// Macro type tag, defaults to `"QuickInputMacro"`.
    #[serde(rename = "type")]
    pub macro_type: String,
    /// App version/date that produced this macro.
    pub ver: String,
    /// Display name shown inside QuickInput.
    pub name: String,
    /// Whether the macro is enabled in the list.
    pub state: bool,
    /// Blocks trigger hotkeys during execution to prevent reentry.
    #[serde(rename = "keyBlock", default)]
    pub key_block: bool,
    /// Blocks mouse movement while the macro runs.
    #[serde(rename = "curBlock", default)]
    pub cur_block: bool,
    /// Whether the macro is bound to a specific window.
    #[serde(rename = "wndState", default)]
    pub wnd_state: bool,
    /// Bound window title (UTF-16 converted to UTF-8).
    #[serde(rename = "wndName", default)]
    pub wnd_name: String,
    /// Bound window class name.
    #[serde(rename = "wndClass", default)]
    pub wnd_class: String,
    /// Whether to target a child window.
    #[serde(rename = "wndChild", default)]
    pub wnd_child: bool,
    /// Legacy hotkey bitmask (32-bit) kept for compatibility.
    #[serde(default)]
    pub key: i32,
    /// Primary trigger virtual-key code.
    #[serde(default)]
    pub key1: i32,
    /// Secondary trigger virtual-key code (modifier).
    #[serde(default)]
    pub key2: i32,
    /// Trigger mode: 0=toggle, 1=on press, 2=on release.
    pub mode: i32,
    /// Loop count; 0 means infinite.
    pub count: i32,
    /// Whether scheduled execution is enabled.
    pub timer: bool,
    /// Allowed start time in seconds since midnight.
    #[serde(rename = "timerStart", default)]
    pub timer_start: i64,
    /// Allowed end time in seconds since midnight.
    #[serde(rename = "timerEnd", default)]
    pub timer_end: i64,
    /// Global execution speed multiplier (0.1–10.0).
    pub speed: f32,
    /// Scale factor for relative mouse movement on X.
    #[serde(rename = "moveScaleX", default)]
    pub move_scale_x: f32,
    /// Scale factor for relative mouse movement on Y.
    #[serde(rename = "moveScaleY", default)]
    pub move_scale_y: f32,
    /// Correction factor for absolute X coordinates.
    #[serde(rename = "posScaleX", default)]
    pub pos_scale_x: f32,
    /// Correction factor for absolute Y coordinates.
    #[serde(rename = "posScaleY", default)]
    pub pos_scale_y: f32,
    /// Initialization script string run before execution.
    #[serde(default)]
    pub script: String,
    /// Primary action list.
    #[serde(default)]
    pub actions: Vec<Action>,
    /// Tail actions run after completion.
    #[serde(rename = "actionsEnding", default)]
    pub actions_ending: Vec<Action>,
}
/// Single action node; populated fields depend on `kind`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    /// Action type matching QuickInput `QiType` (e.g., 2=delay, 3=key).
    #[serde(rename = "type")]
    pub kind: i32,
    /// Skip this action when set to true.
    #[serde(default)]
    pub dis: bool,
    /// Note shown in the editor.
    #[serde(default)]
    pub mark: String,
    /// Minimum delay in milliseconds.
    #[serde(default)]
    pub min: Option<i32>,
    /// Maximum delay in milliseconds.
    #[serde(default)]
    pub max: Option<i32>,
    /// Script expression overriding `min`.
    #[serde(default)]
    pub v_min: Option<String>,
    /// Script expression overriding `max`.
    #[serde(default)]
    pub v_max: Option<String>,
    /// Legacy minimum delay (mapped to `min` when present).
    #[serde(default)]
    pub ms: Option<i32>,
    /// Legacy maximum delay (mapped to `max` when present).
    #[serde(default)]
    pub ex: Option<i32>,
    /// Virtual-key code for key actions.
    #[serde(default)]
    pub vk: Option<i32>,
    /// Key state: 0=release, 1=press, 2=click.
    #[serde(default)]
    pub state: Option<i32>,
    /// Mouse-move flag for pointer actions (QiType 4).
    #[serde(rename = "move", default)]
    pub move_flag: Option<bool>,
    /// Track target window during pointer actions.
    #[serde(rename = "trk", default)]
    pub track: Option<bool>,
    /// Pointer move speed hint.
    #[serde(rename = "spd", default)]
    pub speed: Option<i32>,
    /// Target X coordinate for pointer actions.
    #[serde(default)]
    pub x: Option<i32>,
    /// Target Y coordinate for pointer actions.
    #[serde(default)]
    pub y: Option<i32>,
}
// Constants mirroring QiType values for quick comparisons.
pub const ACTION_END: i32 = 1;
pub const ACTION_DELAY: i32 = 2;
pub const ACTION_KEY: i32 = 3;
