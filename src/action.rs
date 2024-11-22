pub trait 動作: Fn() + Copy + 'static {}
impl<T> 動作 for T where T: Fn() + Copy + 'static {}

pub struct 未有();
pub type 成功失敗 = Result<(), 未有>;

pub trait 動作得一結果<R = 成功失敗>: Fn() -> R + Copy + 'static {}
impl<R, T> 動作得一結果<R> for T where T: Fn() -> R + Copy + 'static {}

pub trait 動作給一參數得一結果<P, R = 成功失敗>: Fn(P) -> R + Copy + 'static {}
impl<P, R, T> 動作給一參數得一結果<P, R> for T where T: Fn(P) -> R + Copy + 'static {}
