use crate::enemies::Enemy;

pub trait Buff<T> {
    fn change_enemy(&self, enemy: &mut Enemy) {}

    fn change_attribute(&self, attribute: &mut T) {}
}