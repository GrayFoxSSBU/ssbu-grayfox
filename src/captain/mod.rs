use crate::prelude::*;


// Knee SFX
#[acmd_script( agent = "captain", script = "sound_attackairf", category = ACMD_SOUND )]
unsafe fn captain_sound_attackairf_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_captain_swing_1"));

        // Dtaunt on hit for "YES!"
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            macros::PLAY_SE(fighter,Hash40::new("vc_captain_appeal03"));
        }
    }
}


// More active knee, less endlag
#[acmd_script( agent = "captain", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn captain_attackairf_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }    

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CAPTAIN_STATUS_ATTACK_AIR_WORK_INT_CRITICAL_ATTACK_ID);
        
        macros::ATTACK(fighter,0, 0, Hash40::new("legl"), 22.0, 32, 81, 0, 30, 3.75, 4.4, -0.2, -1.0, Some(4.4), Some(0.4), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 22.0, 32, 81, 0, 30, 3.0, 0.0, 6.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
    
    }

    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_ATTACK_AIR_WORK_FLAG_CRITICAL);

        macros::ATTACK(fighter,0, 0, Hash40::new("legl"), 3.0, 361, 80, 0, 35, 4.7, 4.4, -0.2, -1.0, Some(4.4), Some(0.3), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 3.0, 361, 80, 0, 35, 3.0, 0.0, 6.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
    }

    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.6);
    }

    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// uptilt fire effect
#[acmd_script( agent = "captain", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn captain_attackhi3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.812);
    }    

    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {

        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);

        macros::ATTACK(fighter,0, 0, Hash40::new("kneer"), 11.0, 285, 100, 0, 35, 6.0, 7.0, -1.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 11.0, 285, 90, 0, 22, 3.5, 7.0, -1.0, -0.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,2, 0, Hash40::new("legr"), 11.0, 285, 100, 0, 35, 4.0, 5.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,3, 0, Hash40::new("legr"), 11.0, 285, 100, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,4, 0, Hash40::new("legr"), 11.0, 285, 100, 0, 35, 4.0, 5.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,5, 0, Hash40::new("legr"), 11.0, 285, 100, 0, 35, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    
    }

    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL),0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Upsmash fire
#[acmd_script( agent = "captain", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn captain_attackhi4_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }    

    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {

        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);

        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 7.0, 110, 90, 150, 0, 5.7, 0.0, 7.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 7.0, 96, 90, 80, 0, 6.0, 0.0, 17.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,3, 0, Hash40::new("top"), 12.0, 80, 90, 10, 5, 4.8, 0.0, 21.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,2, 0, Hash40::new("top"), 12.0, 80, 90, 0, 0, 4.8, 0.0, 28.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 5.0, y: 29.0}, 8, false);
    
    }

    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor,1,false);
    }

    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL),0);
    }

    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);

        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 14.0, 83, 80, 0, 70, 6.0, 0.0, 29.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 14.0, 85, 81, 0, 70, 5.5, 0.0, 21.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,2, 0, Hash40::new("top"), 13.0, 70, 88, 0, 70, 5.0, 0.0, 16.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {        
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL),0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Downsmash fire
#[acmd_script( agent = "captain", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn captain_attacklw4_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }    

    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter,0, 0, Hash40::new("kneer"), 14.0, 28, 90, 0, 30, 4.5, 4.9, -0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 14.0, 28, 90, 0, 30, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,3, 0, Hash40::new("legr"), 14.0, 28, 90, 0, 30, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
    }

    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter,0, 0, Hash40::new("kneer"), 18.0, 28, 84, 0, 20, 4.5, 4.9, -0.9, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 18.0, 28, 84, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter,3, 0, Hash40::new("legr"), 18.0, 28, 84, 0, 20, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
    }

    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// jab3 elec
#[acmd_script( agent = "captain", script = "game_attack13", category = ACMD_GAME )]
unsafe fn captain_attack13_smash_script(fighter: &mut L2CAgentBase) {
    


    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 70, 5.5, 0.0, 11.0, 12.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 5.0, 361, 100, 0, 70, 3.8, 0.0, 10.0, 6.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KNEE);
    }

    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// dair elec
#[acmd_script( agent = "captain", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn captain_attackairlw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
    
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        JostleModule::set_status(fighter.module_accessor,false);
    }    

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 14.0, 270, 100, 0, 10, 5.9, 0.0, -5.2, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("top"), 14.0, 361, 100, 0, 40, 6.0, 0.0, 1.0, 0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
    }

    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        captain_attackairf_smash_script,
        captain_sound_attackairf_smash_script,
        captain_attackhi3_smash_script,
        captain_attackhi4_smash_script,
        captain_attacklw4_smash_script,
        captain_attack13_smash_script,
        captain_attackairlw_smash_script
    );
}
