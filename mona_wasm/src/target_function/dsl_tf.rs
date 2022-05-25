use std::cell::RefCell;
use mona::artifacts::Artifact;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::SimpleAttributeGraph2;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use mona::target_functions::TargetFunction;
use mona::team::TeamQuantization;
use mona::weapon::Weapon;
use mona_dsl::common::UnsafeDamageContext;
use mona_dsl::compile_source_to_code_object;
use mona_dsl::vm::env::MonaEnv;

pub struct TargetFunctionDSL {
    pub vm: RefCell<MonaEnv>
}

impl TargetFunction for TargetFunctionDSL {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        todo!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context_unsafe = UnsafeDamageContext {
            character_common_data: &character.common_data,
            enemy: &*enemy,
            attribute: &*attribute
        };

        {
            let mut borrow_mut = self.vm.borrow_mut();

            borrow_mut.add_damage_context(context_unsafe);
            borrow_mut.init_damage();
            borrow_mut.init_prop();
            borrow_mut.execute();
        }

        // println!("{:?}", self.vm.borrow().namespace.map.keys());
        let result = self.vm.borrow().namespace.map.get("result").unwrap().borrow().get_number();
        self.vm.borrow_mut().clear_local_state();

        result
    }
}

impl TargetFunctionDSL {
    pub fn new(source: &str) -> Self {
        let code_obj = compile_source_to_code_object(source).unwrap();
        let env = MonaEnv::new(code_obj);

        TargetFunctionDSL {
            vm: RefCell::new(env)
        }
    }
}