use crate::prelude::*;



// front hitbox up smash
#[acmd_script( agent = "mario", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn mario_attackhi4_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        JostleModule::set_status(fighter.module_accessor, false);
    }    
    
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 14.0, 83, 94, 0, 32, 5.0, 2.5, 1.1, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 14.0, 83, 94, 0, 32, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);

        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        }

        sv_animcmd::wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL),0);
        }
        
        sv_animcmd::frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, true);
        }
    }
    

pub fn install() {
    smashline::install_acmd_scripts!(
        mario_attackhi4_smash_script
    );
}
