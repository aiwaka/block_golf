use self::{fan::Fan, switch::SwitchReceiver};

pub mod fan;
pub mod switch;
pub mod updater;

/// ブロックに取り付けられるもの
#[derive(Clone, Debug)]
pub enum BlockAttachment {
    /// スイッチが押されたことを検知して対象に影響を与える
    SwitchReceiver {
        receiver: SwitchReceiver,
    },
    Fan(Fan),
}
