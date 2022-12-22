use crate::prelude::*;


// Up Air endlag
#[acmd_script( agent = "krool", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn krool_attackairhi_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        SET_SPEED_EX(fighter,0, 2.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 85, 60, 0, 84, 8.6, 0.0, 22.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);

    }
    
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {

        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL),0);
    
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 85, 52, 0, 85, 6.2, 0.0, 22.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
    }
    
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
    }

    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
        
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//faster Dair
#[acmd_script( agent = "krool", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn krool_attackairlw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_ON);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 90, 0, 10, 7.0, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 270, 90, 0, 10, 8.5, 0.0, -6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

    }
    
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 100, 0, 40, 6.5, 0.0, -1.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 20, 7.0, 0.0, -6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL),0);
    }

    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KROOL_INSTANCE_WORK_ID_FLAG_REQUEST_WAIST_SHIELD_OFF);
    }

    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

}

//Faster and bigger Fsmash
#[acmd_script( agent = "krool", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn krool_attacks4_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }

    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 19.0, 361, 96, 0, 40, 6.3, 6.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 16.5, 361, 96, 0, 40, 4.5, -2.0, 0.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor,0, 5.2);
    }

    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Fsmash hi
#[acmd_script( agent = "krool", script = "game_attacks4hi", category = ACMD_GAME )]
unsafe fn krool_attacks4hi_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 19.95, 361, 96, 0, 40, 6.3, 6.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 17.325, 361, 96, 0, 40, 4.5, -2.0, 0.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor,0, 5.2);
    }

    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Fsmash low
#[acmd_script( agent = "krool", script = "game_attacks4lw", category = ACMD_GAME )]
unsafe fn krool_attacks4lw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 19.57, 361, 96, 0, 40, 6.3, 6.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 16.995, 361, 96, 0, 40, 4.5, -2.0, 0.0, 0.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor,0, 5.2);
    }

    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        krool_attackairhi_smash_script,
        krool_attackairlw_smash_script,
        krool_attacks4_smash_script,
        krool_attacks4hi_smash_script,
        krool_attacks4lw_smash_script
    );
}
