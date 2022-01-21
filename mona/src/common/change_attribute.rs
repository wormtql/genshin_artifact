pub trait ChangeAttribute<T> {
    fn change_attribute(&self, attribute: &mut T);
}