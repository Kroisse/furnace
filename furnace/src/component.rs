use crate::{dispatcher::Handle, model::Update};

pub trait RenderContext {
    type Widget;
    type Queue: RenderQueue;

    fn show(&mut self, widget: Self::Widget);
    fn main_loop<'a>(&'a mut self, f: impl FnMut(&Self) -> () + 'a);
    fn queue(&mut self) -> Self::Queue;
    fn quit(&self);
}

pub trait RenderQueue: Send + Sync + 'static {
    fn execute(&self, f: Box<dyn FnOnce()>);
}

/// Building block of user interface.
pub trait Component {
    type Renderer: RenderContext;
    type Model: Update;

    fn root(&self) -> <Self::Renderer as RenderContext>::Widget;
    fn view(ui: &Self::Renderer, handle: Handle<<Self::Model as Update>::Action>, model: &Self::Model) -> Self;
    #[allow(unused_variables)]
    fn update(&mut self, ui: &Self::Renderer, model: &Self::Model) {}
}
