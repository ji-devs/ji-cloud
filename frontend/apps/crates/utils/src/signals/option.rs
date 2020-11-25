use futures_signals::{
    map_ref,map_mut,
    signal::{Mutable, MutableSignal,MutableSignalCloned, SignalExt, Signal, always, Map},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use std::pin::Pin;
use std::marker::Unpin;
use std::future::Future;
use std::task::{Context, Poll};
use crate::math::*;

// TODO - simplify with macros and implement more!

// i32 
pub struct OptionI32Signal <S> 
where
    S: Signal<Item = i32>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionI32Signal <S> 
where
    S: Signal<Item = i32>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionI32Signal<S>
where
    S: Signal<Item = i32> + Unpin
{
    type Item = Option<i32>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}
// f64 
pub struct OptionF64Signal <S> 
where
    S: Signal<Item = f64>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionF64Signal <S> 
where
    S: Signal<Item = f64>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionF64Signal<S>
where
    S: Signal<Item = f64> + Unpin
{
    type Item = Option<f64>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}

// Bounds 
pub struct OptionBoundsF64Signal <S> 
where
    S: Signal<Item = BoundsF64>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionBoundsF64Signal <S> 
where
    S: Signal<Item = BoundsF64>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionBoundsF64Signal<S>
where
    S: Signal<Item = BoundsF64> + Unpin
{
    type Item = Option<BoundsF64>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}
// String 
pub struct OptionStringSignal <S> 
where
    S: Signal<Item = String>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionStringSignal <S> 
where
    S: Signal<Item = String>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionStringSignal<S>
where
    S: Signal<Item = String> + Unpin
{
    type Item = Option<String>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}

// Usize
pub struct OptionUsizeSignal <S> 
where
    S: Signal<Item = usize>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionUsizeSignal <S> 
where
    S: Signal<Item = usize>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionUsizeSignal<S>
where
    S: Signal<Item = usize> + Unpin
{
    type Item = Option<usize>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}

// PointI32 
pub struct OptionPointI32Signal <S> 
where
    S: Signal<Item = PointI32>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionPointI32Signal <S> 
where
    S: Signal<Item = PointI32>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionPointI32Signal<S>
where
    S: Signal<Item = PointI32> + Unpin
{
    type Item = Option<PointI32>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}
// PointF64 
pub struct OptionPointF64Signal <S> 
where
    S: Signal<Item = PointF64>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionPointF64Signal <S> 
where
    S: Signal<Item = PointF64>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionPointF64Signal<S>
where
    S: Signal<Item = PointF64> + Unpin
{
    type Item = Option<PointF64>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}

// RectF64 
pub struct OptionRectF64Signal <S> 
where
    S: Signal<Item = RectF64>
{
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> OptionRectF64Signal <S> 
where
    S: Signal<Item = RectF64>
{
    pub fn new(value_signal: Option<S>) -> Self {
        Self {
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for OptionRectF64Signal<S>
where
    S: Signal<Item = RectF64> + Unpin
{
    type Item = Option<RectF64>;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(Some(None))
                }
            }
            Some(value) => {
                Pin::new(value)
                    .poll_change(cx)
                    .map(|value| Some(value))
            }
        }
    }
}

