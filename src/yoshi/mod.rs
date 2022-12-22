use crate::prelude::*;



// F Throw tech chase
#[acmd_script( agent = "yoshi", script = "game_throwf", category = ACMD_GAME )]
unsafe fn yoshi_throwf_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0, 9.0, 15, 35, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0, 9.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        
        macros::CHECK_FINISH_CAMERA(fighter, 13, 14);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.1);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 6.0, y: 4.5, z: 0.0});    

    }

    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }

}

// Strong Egg
#[acmd_script( agent = "yoshi", script = "game_specialsloop", category = ACMD_GAME )]
unsafe fn yoshi_specialsloop_smash_script(fighter: &mut L2CAgentBase) {
    
    
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
    
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 60, 0, 70, 2.8, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 45, 60, 0, 70, 4.0, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        
    JostleModule::set_status(fighter.module_accessor,false);       

}

// Strong Air Egg
#[acmd_script( agent = "yoshi", script = "game_specialairsloop", category = ACMD_GAME )]
unsafe fn yoshi_specialairsloop_smash_script(fighter: &mut L2CAgentBase) {
    
    damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
    
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 45, 60, 0, 70, 2.8, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 45, 60, 0, 70, 4.0, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        
    JostleModule::set_status(fighter.module_accessor,false);       

}
pub fn install() {

    smashline::install_acmd_scripts!(
        yoshi_throwf_smash_script,
        yoshi_specialsloop_smash_script,
        yoshi_specialairsloop_smash_script
    );
}
