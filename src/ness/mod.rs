use crate::prelude::*;



// Bigger magnet
#[acmd_script( agent = "ness", script = "game_speciallwhold", category = ACMD_GAME )]
unsafe fn ness_speciallwhold_smash_script(fighter: &mut L2CAgentBase) {

    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter,0, 0, Hash40::new("top"), 4.0, 361, 100, 40, 0, 9.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        fighter.clear_lua_stack();
        sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
      }
}


pub fn install() {

    smashline::install_acmd_scripts!(
        ness_speciallwhold_smash_script
    );
}
