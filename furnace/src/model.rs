use std::fmt::Debug;

pub trait Action: Debug + Send + Sync + 'static { }

pub trait Update {
    type Action: Action;

    fn update(&mut self, action: &Self::Action);
}
