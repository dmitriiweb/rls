pub mod list;
pub mod table;

pub use list::List;
pub use table::Table;

pub trait Render {
    fn render(&self);
}
