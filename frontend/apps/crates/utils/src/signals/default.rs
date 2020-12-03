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
// When GAT's arrive we can probably change to just DefaultCloneSignal / DefaultCopySignal
// And then with specializaion maybe just DefaultSignal :)


// String
pub struct DefaultStringSignal <S> 
where
    S: Signal<Item = String>
{
    default: Option<String>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultStringSignal <S> 
where
    S: Signal<Item = String>
{
    pub fn new(default:String, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultStringSignal<S>
where
    S: Signal<Item = String> + Unpin
{
    type Item = String;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}

// Usize
pub struct DefaultUsizeSignal <S> 
where
    S: Signal<Item = usize>
{
    default: Option<usize>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultUsizeSignal <S> 
where
    S: Signal<Item = usize>
{
    pub fn new(default:usize, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultUsizeSignal<S>
where
    S: Signal<Item = usize> + Unpin
{
    type Item = usize;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}

// i32
pub struct DefaultI32Signal <S> 
where
    S: Signal<Item = i32>
{
    default: Option<i32>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultI32Signal <S> 
where
    S: Signal<Item = i32>
{
    pub fn new(default:i32, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultI32Signal<S>
where
    S: Signal<Item = i32> + Unpin
{
    type Item = i32;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}

// f64
pub struct DefaultF64Signal <S> 
where
    S: Signal<Item = f64>
{
    default: Option<f64>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultF64Signal <S> 
where
    S: Signal<Item = f64>
{
    pub fn new(default:f64, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultF64Signal<S>
where
    S: Signal<Item = f64> + Unpin
{
    type Item = f64;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}


// PointI32
pub struct DefaultPointI32Signal <S> 
where
    S: Signal<Item = PointI32>
{
    default: Option<PointI32>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultPointI32Signal <S> 
where
    S: Signal<Item = PointI32>
{
    pub fn new(default:PointI32, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultPointI32Signal<S>
where
    S: Signal<Item = PointI32> + Unpin
{
    type Item = PointI32;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}

// PointF64
pub struct DefaultPointF64Signal <S> 
where
    S: Signal<Item = PointF64>
{
    default: Option<PointF64>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultPointF64Signal <S> 
where
    S: Signal<Item = PointF64>
{
    pub fn new(default:PointF64, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultPointF64Signal<S>
where
    S: Signal<Item = PointF64> + Unpin
{
    type Item = PointF64;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}


// RectF64
pub struct DefaultRectF64Signal <S> 
where
    S: Signal<Item = RectF64>
{
    default: Option<RectF64>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultRectF64Signal <S> 
where
    S: Signal<Item = RectF64>
{
    pub fn new(default:RectF64, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultRectF64Signal<S>
where
    S: Signal<Item = RectF64> + Unpin
{
    type Item = RectF64;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}


// BoundsF64
pub struct DefaultBoundsF64Signal <S> 
where
    S: Signal<Item = BoundsF64>
{
    default: Option<BoundsF64>,
    value_signal: Option<S>,
    const_has_fired: bool
}

impl <S> DefaultBoundsF64Signal <S> 
where
    S: Signal<Item = BoundsF64>
{
    pub fn new(default:BoundsF64, value_signal: Option<S>) -> Self {
        Self {
            default: Some(default),
            value_signal,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for DefaultBoundsF64Signal<S>
where
    S: Signal<Item = BoundsF64> + Unpin
{
    type Item = BoundsF64;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value_signal {
            None => {
                if self.const_has_fired {
                    Poll::Ready(None)
                } else {
                    self.const_has_fired = true;
                    Poll::Ready(self.default.take())
                }
            }
            Some(value) => {
                Pin::new(value).poll_change(cx)
            }
        }
    }
}
