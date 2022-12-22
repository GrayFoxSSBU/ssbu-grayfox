use crate::prelude::*;


// Stronger up throw
#[acmd_script( agent = "cloud", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn cloud_throwhi_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 80, 132, 0, 78, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    
        
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 2.0, 50, 150, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 50, 150, 0, 80, 4.0, 0.0, 21.5, 7.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    
        macros::CHECK_FINISH_CAMERA(fighter, 9, 18);

        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.3);
    }

    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);

        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }

}

pub fn install() {

    smashline::install_acmd_scripts!(
        cloud_throwhi_smash_script
    );
}
