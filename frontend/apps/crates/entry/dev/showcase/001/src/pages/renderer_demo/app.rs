use crate::prelude::*;
use renderer::prelude::*;

pub struct App {
    state: Rc<State>,
}

impl Drop for App {
    fn drop(&mut self) {
        log::info!("app dropped!");
    }
}
impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let state = Rc::new(State::new(canvas));
        state.renderer.render();

        state.image_loader.load(clone!(state => async move {
            let texture_id = state.renderer.load_texture(utils::path::ui("mock/thumbnail/sticker.jpg")).await.unwrap_throw();
            state.renderer.add_sprite(texture_id, None);

            state.renderer.render();
        }));


        Self { 
            state
        }
    }
}

struct State {
    renderer: Rc<Renderer>,
    image_loader: AsyncLoader,
}

impl State {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let renderer = Rc::new(Renderer::new(canvas, None));
        Self {
            renderer,
            image_loader: AsyncLoader::new()
        }
    }
}
