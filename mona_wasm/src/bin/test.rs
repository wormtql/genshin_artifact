use mona::character::{Character, CharacterConfig, CharacterName};
use mona::target_functions::target_functions::{HuTaoDefaultTargetFunction, NilouDefaultTargetFunction};
use mona::target_functions::TargetFunction;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use mona_wasm::applications::artifact_best_set::artifact_best_set::calc_artifact_best_set;

fn main() {
    let c = Character::new(
        CharacterName::HuTao,
        90,
        false,
        0,
        9, 9, 9,
        &CharacterConfig::Nilou { golden_rate: 1.0 },
    );
    let w = Weapon::new(
        WeaponName::StaffOfHoma,
        90,
        false,
        1,
        &WeaponConfig::NoConfig,
        &c
    );
    let tf: Box<dyn TargetFunction> = Box::new(HuTaoDefaultTargetFunction {
        vaporize_rate: 0.5,
        melt_rate: 0.0
    });

    let result = calc_artifact_best_set(
        &c, &w, &tf, None, &[], &Default::default()
    );
    println!("{:?}", result);
}
