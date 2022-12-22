use crate::prelude::*;

// Better Ftilt + earlier
#[acmd_script( agent = "packun", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn packun_attacks3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 366, 20, 0, 20, 6.3, 0.0, 7.5, 12.0, Some(0.0), Some(10.5), Some(13.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);

        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);

        macros::HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }    
    
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

// Poison Bite + safer on shield
#[acmd_script( agent = "packun", script = "game_attacks32", category = ACMD_GAME )]
unsafe fn packun_attacks32_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 117, 0, 40, 6.5, 0.0, 7.5, 15.0, Some(0.0), Some(10.5), Some(15.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);

        AttackModule::set_poison_param(fighter.module_accessor,0, 361, 60, 0.5, false);

        macros::HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
}

// Dtilt less endlag
#[acmd_script( agent = "packun", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn packun_attacklw3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 73, 85, 0, 55, 3.0, 0.0, 4.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 73, 85, 0, 55, 3.0, 0.0, 3.4, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 78, 85, 0, 55, 3.0, 0.0, 2.8, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 78, 85, 0, 55, 3.0, 0.0, 2.0, 17.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
}

// Autocancel Nair + bigger hitbox + less endlag
#[acmd_script( agent = "packun", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn packun_attackairn_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 2.0, 367, 100, 0, 10, 5.0, 1.8, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 2.0, 367, 100, 0, 10, 5.0, 1.8, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }

    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 3.0, 361, 140, 0, 45, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 361, 140, 0, 45, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    } 
    
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
}

// Earlier hitbox for scooping + less endlag
#[acmd_script( agent = "packun", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn packun_attackairf_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);

        macros::ATTACK(fighter, 0, 0, Hash40::new("potc"), 9.0, 361, 60, 0, 50, 4.5, 3.0, 0.5, 0.0, Some(3.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 1, 0, Hash40::new("potc"), 11.0, 361, 60, 0, 55, 6.5, -3.0, 0.5, 0.0, Some(-3.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }

    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 0.6);
    }
}

// Faster + Stronger Bair +  Shield Damage + less endlag
#[acmd_script( agent = "packun", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn packun_attackairb_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.5, 361, 105, 0, 25, 8.0, 0.0, 4.0, -10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
}

// Longer UpAir

#[acmd_script( agent = "packun", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn packun_attackairhi_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("mouth"), 10.0, 85, 104, 0, 40, 6.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);

        macros::HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_XLU);

        macros::HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_XLU);

        macros::HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_XLU);

        macros::HIT_NODE(fighter, Hash40::new("neck6"), *HIT_STATUS_XLU);

        macros::HIT_NODE(fighter, Hash40::new("neck8"), *HIT_STATUS_XLU);
    }

    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
        macros::HIT_NODE(fighter, Hash40::new("mouth"), *HIT_STATUS_NORMAL);

        macros::HIT_NODE(fighter, Hash40::new("lipu3"), *HIT_STATUS_NORMAL);

        macros::HIT_NODE(fighter, Hash40::new("lipd3"), *HIT_STATUS_NORMAL);

        macros::HIT_NODE(fighter, Hash40::new("neck6"), *HIT_STATUS_NORMAL);

        macros::HIT_NODE(fighter, Hash40::new("neck8"), *HIT_STATUS_NORMAL);
    }

    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// More consistent Dair Spike

#[acmd_script( agent = "packun", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn packun_attackairlw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 5.0, 3.0, 8.0, 1.0);
    }
    
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("potc"), 11.0, 361, 92, 0, 15, 4.5, -0.3, 0.0, 0.0, Some(-0.3), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 0, 0, Hash40::new("potc"), 11.0, 270, 90, 0, 25, 4.5, -7.0, -0.5, 0.5, Some(-7.0), Some(-0.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("potc"), 9.0, 361, 92, 0, 15, 4.5, -0.3, 0.0, 0.0, Some(-0.3), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        macros::ATTACK(fighter, 1, 0, Hash40::new("potc"), 9.0, 361, 92, 0, 15, 4.5, -7.0, -0.5, 0.5, Some(-7.0), Some(-0.5), Some(0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }
    

    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Stronger up throw
#[acmd_script( agent = "packun", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn packun_throwhi_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 90, 88, 0, 64, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    
        
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("mouth"), 5.0, 90, 100, 0, 82, 7.0, 2.0, 0.0, 0.0, Some(3.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_THROW);
        
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
        fighter.clear_lua_stack();      
        macros::CHECK_FINISH_CAMERA(fighter, 0, 27);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.7);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 0.0, y: 14.0, z: 0.0});
    }

    frame(fighter.lua_state_agent, 20.0);
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
        packun_attacks3_smash_script,
        packun_attacks32_smash_script,
        packun_attacklw3_smash_script,
        packun_attackairn_smash_script,
        packun_attackairf_smash_script,
        packun_attackairb_smash_script,
        packun_attackairhi_smash_script,
        packun_attackairlw_smash_script,
        packun_throwhi_smash_script
    );
}
