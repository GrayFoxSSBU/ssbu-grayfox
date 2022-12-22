use crate::prelude::*;
use crate::custom::get_player_number;


// Faster Jab1 for jab locks
#[acmd_script( agent = "duckhunt", script = "game_attack11" , category = ACMD_GAME )]
unsafe fn duckhunt_attack11_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 30, 0, 20, 1.8, 0.0, 4.0, 6.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 361, 30, 0, 20, 1.8, 0.0, 4.0, 8.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 180, 20, 0, 20, 2.0, 0.0, 4.0, 11.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 361, 20, 0, 20, 2.0, 0.0, 4.0, 11.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {

        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        macros::FT_MOTION_RATE(fighter, 0.6);
    }
}

//up tilt wider so it hits grounded opponents
#[acmd_script( agent = "duckhunt", script = "game_attackhi3" , category = ACMD_GAME )]
unsafe fn duckhunt_attackhi3_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 93, 40, 0, 95, 5.0, 0.0, 15.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 93, 40, 0, 95, 2.0, 0.0, 2.5, -4.0, Some(0.0), Some(2.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }
}


//down tilt more safe on shield
#[acmd_script( agent = "duckhunt", script = "game_attacklw3" , category = ACMD_GAME )]
unsafe fn duckhunt_attacklw3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 25, 60, 0, 50, 3.5, 0.0, 2.0, 13.0, Some(0.0), Some(2.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);

        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0,1.5);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.74);
    }
}

// Fair no freefall after up B
#[acmd_script( agent = "duckhunt", script = "game_attackairf" , category = ACMD_GAME )]
unsafe fn duckhunt_attackairf_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.5, 48, 80, 0, 40, 4.0, 0.0, 8.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 1.0);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 48, 80, 0, 30, 3.9, 0.0, 8.0, 16.0, Some(0.0), Some(8.0), Some(19.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {

        AttackModule::clear(fighter.module_accessor,1,false);
        macros::ATK_POWER(fighter,0, 7.5);
    }

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }



    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    
}

// Bair no freefall
#[acmd_script( agent = "duckhunt", script = "game_attackairb" , category = ACMD_GAME )]
unsafe fn duckhunt_attackairb_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 90, 0, 40, 4.0, 0.0, 6.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 361, 90, 0, 30, 3.9, 0.0, 6.0, -13.0, Some(0.0), Some(6.0), Some(-10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {

        AttackModule::clear(fighter.module_accessor,0,false);
        macros::ATK_POWER(fighter,0, 10.5);
    }

    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

//up air dragdown hitboxes
#[acmd_script( agent = "duckhunt", script = "game_attackairhi" , category = ACMD_GAME )]
unsafe fn duckhunt_attackairhi_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, 6.0, 0.0, 18.5, 5.0, Some(0.0), Some(18.5), Some(-2.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 100, 50, 0, 6.0, 0.0, 21.5, 4.0, Some(0.0), Some(21.5), Some(-5.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 91, 120, 0, 50, 8.5, 0.0, 22.5, -3.0, Some(0.0), Some(22.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

// Bigger Nair
#[acmd_script( agent = "duckhunt", script = "game_attackairn" , category = ACMD_GAME )]
unsafe fn duckhunt_attackairn_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 86, 0, 30, 9.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 5.0, 361, 90, 0, 22, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.0, 361, 90, 0, 22, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 5.0, 361, 90, 0, 22, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 3, 0, Hash40::new("arml"), 5.0, 361, 90, 0, 22, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }

}

// Stronger Dair
#[acmd_script( agent = "duckhunt", script = "game_attackairlw" , category = ACMD_GAME )]
unsafe fn duckhunt_attackairlw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 130, 20, 0, 20, 4.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 255, 20, 0, 20, 5.5, 0.0, -2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 270, 80, 0, 30, 6.5, 0.0, -8.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 70, 90, 0, 10, 6.5, 0.0, -8.0, 0.0, Some(0.0), Some(-5.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_HEAD);
    }
    
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }

}

// enable fall on Airdodges
// enable fall on side special
// enable fall on neutral special
// enable fall on down special
// enable fall on up special

// up throw kill throw
#[acmd_script( agent = "duckhunt", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn duckhunt_throwhi_smash_script(fighter: &mut L2CAgentBase) {
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 88, 140, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {    
        fighter.clear_lua_stack();
        lua_args!(fighter, 6, 1);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
        fighter.clear_lua_stack();      
        macros::CHECK_FINISH_CAMERA(fighter, 3, 17);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.7);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 0.0, y: 5.0, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// Bigger clay hitbox
#[acmd_script( agent = "duckhunt_clay", script = "game_fly" , category = ACMD_GAME )]
unsafe fn duckhunt_clay_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 75, 50, 0, 20, 2.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {

        WorkModule::on_flag(fighter.module_accessor,*WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_IS_ADD_ACCEL_Y);
    }
}

#[acmd_script( agent = "duckhunt_gunmanbullet", script = "game_move" , category = ACMD_GAME )]
unsafe fn duckhunt_gunmanbullet_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 65, 71, 0, 30, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 65, 112, 0, 30, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 50, 105, 0, 30, 1.2, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 11.0, 70, 105, 0, 30, 4.0, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 10.0, 65, 105, 0, 30, 4.0, 0.0, 0.0, 0.0, Some(1.0), Some(1.0), Some(1.0), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);

        ControlModule::set_rumble(fighter.module_accessor,Hash40::new("rbkind_explosion"), 0, false,0);
    }

}

// One Up B per airtime
pub static mut UPB_CHECK : [bool; 8] = [false; 8];


#[fighter_frame( agent = FIGHTER_KIND_DUCKHUNT)]
fn upb_duckhunt(fighter: &mut L2CFighterCommon) {
    unsafe{
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let is_status = status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if [*FIGHTER_STATUS_KIND_DAMAGE, 
            *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
            *FIGHTER_STATUS_KIND_DAMAGE_FALL, 
            *FIGHTER_STATUS_KIND_DAMAGE_SONG, 
            *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
            ].contains(&status_kind) 
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
        || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF
        && UPB_CHECK[get_player_number(boma)] {
            UPB_CHECK[get_player_number(boma)] = false;
        }
        if is_status {
            UPB_CHECK[get_player_number(boma)] = true;
        }
    }
}



pub fn install() {

    smashline::install_acmd_scripts!(
        duckhunt_attack11_smash_script,
        duckhunt_attackhi3_smash_script,
        duckhunt_attacklw3_smash_script,
        duckhunt_attackairn_smash_script,
        duckhunt_attackairf_smash_script,
        duckhunt_attackairb_smash_script,
        duckhunt_attackairhi_smash_script,
        duckhunt_attackairlw_smash_script,
        duckhunt_throwhi_smash_script,
        duckhunt_gunmanbullet_smash_script,
        duckhunt_clay_smash_script
    );

    smashline::install_agent_frames!(
        upb_duckhunt
    );
  
}
