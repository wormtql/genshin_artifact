use super::weapon_common_data::WeaponCommonData;

pub trait WeaponEffect<T> {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T);
}