// https://users.rust-lang.org/t/ownership-and-async-move-inside-a-loop/43250/5?u=dakom

#[macro_export]
macro_rules! async_clone_fn {
    ($($share:ident),*; |$($arg:ident),*| { $($tok:tt)* }) => {
        {
            $(let $share = $share.clone(););*
            move |$($arg),*| {
                $(let $share = $share.clone(););*
                async move {
                    $($tok)*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! async_clone_cb {
    ($($share:ident),*; || { $($tok:tt)* }) => {
        {
            $(let $share = $share.clone(););*
            move || {
                $(let $share = $share.clone(););*
                async move {
                    $($tok)*
                }
            }
        }
    };
}
