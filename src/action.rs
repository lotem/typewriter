#![allow(dead_code)]

pub trait 动作: Fn() + Copy + 'static {}
impl<T> 动作 for T where T: Fn() + Copy + 'static {}

pub struct 未有();
pub type 成功失败 = Result<(), 未有>;

pub trait 动作得一结果<R = 成功失败>: Fn() -> R + Copy + 'static {}
impl<R, T> 动作得一结果<R> for T where T: Fn() -> R + Copy + 'static {}

pub trait 动作给一参数<P>: Fn(P) + Copy + 'static {}
impl<P, T> 动作给一参数<P> for T where T: Fn(P) + Copy + 'static {}

pub trait 动作给一参数得一结果<P, R = 成功失败>: Fn(P) -> R + Copy + 'static {}
impl<P, R, T> 动作给一参数得一结果<P, R> for T where T: Fn(P) -> R + Copy + 'static {}
