/// ゲームのルールを表す. グローバルなリソースとして使う.
#[derive(Clone, Copy, Debug)]
pub enum GameRule {
    /// 制限時間に入れたボールによるスコア
    BallScore,
    /// 1つ目のボールを入れるまでの操作時間が短いほど高スコア
    LittleOperation,
    /// 1つ目のボールを入れるまでの時間を競う
    TimeAttack,
}

pub struct GameOverEvent;

/// ゴールしたボール
pub struct GoaledBall(pub u32);
/// 最初のボールの総数
pub struct InitialBallNum(pub u32);

/// 経過時間（フレーム）
pub struct PassedTime(pub u32);
/// 矢印キーを操作した量
pub struct OperationAmount(pub u32);
pub struct Score(pub u32);
/// ResultScoreの拡張, 様々な情報を保存しておく
pub struct ResultInfoStorage {
    pub score: u32,
}
impl ResultInfoStorage {
    pub fn to_vector(&self) -> std::vec::Vec<(&str, u32)> {
        vec![("score", self.score)]
    }
}

/// ゲームオーバー状態かどうかを表すフラグ用リソース
pub struct NowGameOver;
