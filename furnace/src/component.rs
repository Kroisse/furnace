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

pub trait Component<M>
where
    M: Update,
{
    type Renderer: RenderContext;

    fn root(&self) -> <Self::Renderer as RenderContext>::Widget;
    fn view(ui: &Self::Renderer, handle: Handle<M::Action>, model: &M) -> Self;
    #[allow(unused_variables)]
    fn update(&mut self, ui: &Self::Renderer, model: &M) {}
}
