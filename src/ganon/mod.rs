use crate::prelude::*;

// Reflect Jab
#[acmd_script( agent = "ganon", script = "game_attack11", category = ACMD_GAME )]
unsafe fn ganon_attack11_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {

        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {

            macros::FT_MOTION_RATE(fighter, 1.0);

            //Reflect Hitbox
            shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 5.0, 0.0, 12.0, 19.0, 0.0, 0.0, 19.0, 1.1, 1.4, 50, false, 1.0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 4.4, 0.0, 12.0, 11.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);

            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 5.0, 0.0, 12.0, 19.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);

            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 5.0, 0.0, 12.0, 7.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }

        else {

            macros::FT_MOTION_RATE(fighter, 1.0);
            
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 4.4, 0.0, 12.0, 11.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);

            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 5.0, 0.0, 12.0, 19.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);

            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 41, 5.0, 0.0, 12.0, 7.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);

        }
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.9);
    }

    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        smash_script::shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

// Early Up Air
#[acmd_script( agent = "ganon", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn ganon_attackairhi_smash_script(fighter: &mut L2CAgentBase) {

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    

    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 13.0, 361, 100, 0, 35, 4.8, 3.2, 0.0, 0.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 12.0, 361, 100, 0, 35, 5.8, 6.0, 0.0, 0.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 12.0, 30, 80, 0, 30, 4.8, 3.2, 0.0, 0.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 30, 80, 0, 30, 5.8, 6.0, 0.0, 0.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);        
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 8.0, 70, 100, 0, 20, 4.8, 3.2, 0.0, 0.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 70, 100, 0, 20, 5.8, 6.0, 0.0, 0.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);        
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

// Upsmash extra frame
#[acmd_script( agent = "ganon", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn ganon_attackhi4_smash_script(fighter: &mut L2CAgentBase) {

    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
    
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }

    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 24.0, 85, 71, 0, 40, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 24.0, 78, 71, 0, 40, 4.5, 0.0, 14.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 21.0, 75, 75, 0, 40, 4.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }

    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Better Grab
#[acmd_script( agent = "ganon", script = "game_catch", category = ACMD_GAME )]
unsafe fn ganon_catch_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor,true);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::CATCH(fighter,0, Hash40::new("top"), 4.0, 0.0, 9.0, 14.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);

        macros::CATCH(fighter,1, Hash40::new("top"), 2.0, 0.0, 9.0, 12.0, Some(0.0), Some(9.0), Some(10.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);

        macros::game_CaptureCutCommon(fighter);
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);

        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);

        GrabModule::set_rebound(fighter.module_accessor,false);
    }

}

//Grab effect
#[acmd_script( agent = "ganon", script = "effect_catch", category = ACMD_EFFECT )]
unsafe fn ganon_effect_catch_smash_script(fighter: &mut L2CAgentBase) {


    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::EFFECT(fighter,Hash40::new("ganon_engokua_flash"), Hash40::new("top"), 0, 9, 12, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

    }
}

// Combo Down Throw
#[acmd_script( agent = "ganon", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn ganon_throwlw_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 65, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 11, 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// Up B Better Grab
#[acmd_script( agent = "ganon", script = "game_specialhi", category = ACMD_GAME )]
unsafe fn ganon_specialhi_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.2);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }

    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.9);
    }
    
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        macros::CATCH(fighter,0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);

        macros::CATCH(fighter,1, Hash40::new("top"), 6.5, 0.0, 8.8, 13.7, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);

        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }

    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {

        macros::CATCH(fighter,0, Hash40::new("top"), 4.4, 0.0, 16.0, 6.5, Some(0.0), Some(18.0), Some(3.0), *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);

        macros::CATCH(fighter,1, Hash40::new("top"), 4.4, 0.0, 8.8, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_GANON, *COLLISION_SITUATION_MASK_GA);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }

    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }

    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 9.0, 6.0, 0.0, 0.0,None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);

        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 70, 6.0, -1.0, 0.0, 0.0,None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }

    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }

    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE);
    }
}
// Down B restores Double Jump

pub static mut DJ_RESTORED : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_GANON)]
fn jump_kick(fighter: &mut L2CFighterCommon) {
    unsafe{
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let is_status = status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW;
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if is_status 
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR 
        && jump_count == 2 
        && !DJ_RESTORED[entry_id] {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            DJ_RESTORED[entry_id] = true;
        }

        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            DJ_RESTORED[entry_id] = false;
        }
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        ganon_attack11_smash_script,
        ganon_attackairhi_smash_script,
        ganon_attackhi4_smash_script,
        ganon_catch_smash_script,
        ganon_effect_catch_smash_script,
        ganon_throwlw_smash_script,
        ganon_specialhi_smash_script

    );
    smashline::install_agent_frames!(
        jump_kick
    );
}
 




