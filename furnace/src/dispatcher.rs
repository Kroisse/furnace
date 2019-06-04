use std::fmt::Debug;
use std::sync::Arc;

use arc_swap::ArcSwapOption;
use crossbeam_channel as ch;

use crate::{
    component::{Component, RenderContext, RenderQueue},
    error::Fallible,
    model::Update,
};

pub struct Dispatcher<M>
where
    M: Update,
{
    model: M,
}

impl<M> Dispatcher<M>
where
    M: Update + Default,
{
    pub fn new() -> Fallible<Self> {
        let model = M::default();
        Ok(Dispatcher { model })
    }
}

impl<M> Dispatcher<M>
where
    M: Update,
    M::Action: Debug,
{
    pub fn main<C>(mut self, mut ui: <C as Component<M>>::Renderer) -> Fallible<()>
    where
        C: Component<M>,
    {
        let (h, rx) = Handle::new(ui.queue());
        let mut toplevel = <C as Component<M>>::view(&ui, h.clone(), &self.model);
        log::info!("Toplevel component is ready");

        let root = toplevel.root();
        ui.show(root);
        ui.main_loop(|ui| match rx.try_recv() {
            Ok(act) => {
                log::info!("Action received: {:?}", act);
                self.model.update(&act);
                toplevel.update(&ui, &self.model);
            }
            Err(ch::TryRecvError::Empty) => {}
            Err(ch::TryRecvError::Disconnected) => {
                log::error!("Critical error: dispatcher loop has broken.");
                ui.quit();
            }
        });

        Ok(())
    }
}

pub struct Handle<A> {
    inner: ArcSwapOption<HandleInner<A>>,
    queue: Arc<dyn RenderQueue>,
}

struct HandleInner<A> {
    tx: ch::Sender<A>,
}

impl<A> Handle<A> {
    fn new(queue: impl RenderQueue) -> (Handle<A>, ch::Receiver<A>) {
        let (tx, rx) = ch::unbounded();
        let inner = HandleInner { tx };
        (
            Handle {
                inner: ArcSwapOption::from_pointee(inner),
                queue: Arc::new(queue),
            },
            rx,
        )
    }
}

impl<A> Handle<A>
where
    A: Debug + Send + 'static,
{
    pub fn dispatch(&self, action: A) {
        let lease = match self.inner.lease().into_option() {
            Some(v) => v,
            None => return,
        };
        log::info!("Dispatching: {:#?}", action);
        let inner: &HandleInner<A> = &*lease;
        let tx = inner.tx.clone();
        self.queue.execute(Box::new(move || {
            let _ = tx.try_send(action);
        }));
    }
}

impl<A> Clone for Handle<A> {
    fn clone(&self) -> Self {
        Handle {
            inner: self.inner.clone(),
            queue: Arc::clone(&self.queue),
        }
    }
}
