#![deny(rust_2018_idioms)]

use furnace::{component::RenderQueue, prelude::*};
use iui::{controls::Control, prelude::*};

#[derive(Clone)]
pub struct UiContext {
    ui: UI,
}

impl UiContext {
    pub fn init() -> Result<Self, iui::UIError> {
        let ui = UI::init()?;
        Ok(UiContext { ui })
    }
}

impl RenderContext for UiContext {
    type Widget = Control;
    type Queue = UiQueue;

    fn show(&mut self, widget: Self::Widget) {
        self.ui.set_shown(widget, true)
    }

    fn main_loop<'a>(&'a mut self, mut f: impl FnMut(&Self) -> () + 'a) {
        let mut event_loop = self.ui.event_loop();
        event_loop.on_tick(&self.ui, || f(&*self));
        event_loop.run_delay(&self.ui, 1000);
    }

    fn queue(&mut self) -> Self::Queue {
        UiQueue { _opaque: () }
    }

    fn quit(&self) {
        self.ui.quit()
    }
}

impl std::ops::Deref for UiContext {
    type Target = UI;

    fn deref(&self) -> &Self::Target {
        &self.ui
    }
}

pub struct UiQueue {
    _opaque: (),
}

impl RenderQueue for UiQueue {
    fn execute(&self, f: Box<dyn FnOnce()>) {
        // let mut f = Some(f);
        // self.ui.queue_main(move || {
        //     let f = f
        //         .take()
        //         .expect("render queue tried to execute the callback more than once");
        //     f()
        // });
        f();
    }
}
