/* TIPS
 * a Signal must always return Poll::Ready(Some(...)) the first time it is called
 * after that it can return either Poll::Ready(Some(...)), Poll::Pending, or Poll::Ready(None)
 * and if it returns Poll::Ready(None), then from that point forward it must always return Poll::Ready(None)
*/


//The general idea for a few of these is getting around the fact that 
//always() can't be flatmapped with a different type
//

mod default;
mod option;
pub use default::*;
pub use option::*;
