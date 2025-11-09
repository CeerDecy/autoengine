use auto_engine_core::types::{KeyBoardKeyMode, ToKeyMode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub scripts: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "event_type")]
pub enum Event {
    EM {
        delay: u64,
        action_type: MouseAction,
        action: MouseActionParam,
    },
    EK {
        delay: u64,
        action_type: KeyAction,
        action: KeyActionParam,
    },
    EX {
        delay: u64,
        action_type: String,
        action: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MouseAction {
    #[serde(rename = "mouse left down")]
    LeftDown,
    #[serde(rename = "mouse left up")]
    LeftUp,
    #[serde(rename = "mouse right down")]
    RightDown,
    #[serde(rename = "mouse right up")]
    RightUp,
    #[serde(rename = "mouse move")]
    Move,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MouseActionParam {
    /// 使用百分比坐标 ["0.2604%", "0.5556%"]
    Percent([String; 2]),
    /// 当前鼠标位置 [-1, -1]
    CurrentPosition([i32; 2]),
}

/// 键盘动作枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KeyAction {
    #[serde(rename = "key down")]
    KeyDown,
    #[serde(rename = "key up")]
    KeyUp,
}

impl ToKeyMode for KeyAction {
    fn to_key_mode(&self) -> KeyBoardKeyMode {
        match self {
            KeyAction::KeyDown => KeyBoardKeyMode::Down,
            KeyAction::KeyUp => KeyBoardKeyMode::Up,
        }
    }
}

impl Display for KeyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyAction::KeyDown => {
                write!(f, "keydown")
            }
            KeyAction::KeyUp => {
                write!(f, "keyup")
            }
        }
    }
}

/// 键盘参数
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum KeyActionParam {
    /// 数组 [keycode, 字符, 修饰键标识]
    Key(Vec<serde_json::Value>),
}
