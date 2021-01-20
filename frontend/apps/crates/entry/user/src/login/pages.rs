use dominator::{Dom, html, events};
use std::rc::Rc;

pub struct Login {
}

impl Login {
    pub fn render() -> Dom {
        let _self = Rc::new(Self {
        });

        html!("page-login-landing", {
            .children(vec![
                html!("button-google", {
                    .property("slot", "google")
                    .event(|evt:events::Click| {
                        log::info!("clicked google!");
                    })
                })
            ])
        })

    }
}
/*
        <page-login-landing>

            <button-google slot="google"></button-google>
            
            <input-text slot="username" mode="text" label="${STR_USERLABEL}" }></input-text>
            <input-text slot="password" mode="passwordHidden" label="${STR_PASSWORD}" ></input-text>
            <button-text color="blue" slot="password-forgot">${STR_FORGOTTEN}</button-text>
            <button-rect slot="submit" color="red" size="medium">${STR_SUBMIT}</button-rect> 
            <button-text color="blue" slot="register">${STR_REGISTER}</button-text>
        </page-login-landing>
        */
