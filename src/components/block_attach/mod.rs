use bevy::prelude::Component;

use self::{
    fan::Fan,
    switch::{SwitchReceiver, SwitchType},
};

pub mod fan;
pub mod switch;

/// ブロックに取り付けられるもの
#[derive(Clone, Debug)]
pub enum BlockAttachment {
    /// スイッチが押されたことを検知して対象に影響を与える
    SwitchReceiver {
        receiver: SwitchReceiver,
    },
    Fan(Fan),
}
