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
/* TIPS
 * a Signal must always return Poll::Ready(Some(...)) the first time it is called
 * after that it can return either Poll::Ready(Some(...)), Poll::Pending, or Poll::Ready(None)
 * and if it returns Poll::Ready(None), then from that point forward it must always return Poll::Ready(None)
*/


//The general idea for a few of these is getting around the fact that 
//always() can't be flatmapped with a different type
//

//Takes a name, a default (for when it's none), and a MutableSignalCloned for a changing value
pub struct StyleSignal <S> 
where
    S: Signal<Item = String>
{
    default: Option<String>,
    value: Option<S>,
    const_has_fired: bool
}

impl <S> StyleSignal <S> 
where
    S: Signal<Item = String>
{
    pub fn new(default:String, value: Option<S>) -> Self {
        Self {
            default: Some(default),
            value,
            const_has_fired: false,
        }
    }
}

impl <S> Signal for StyleSignal<S>
where
    S: Signal<Item = String> + Unpin
{
    type Item = String;
    
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match &mut self.value {
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
