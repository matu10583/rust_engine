use crate::core::app::App;

pub trait Plugin {
    fn build(&self, app: &mut App);
}
