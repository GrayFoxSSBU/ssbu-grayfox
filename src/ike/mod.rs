use crate::prelude::*;
pub const SITUATION_KIND: i32 = 0x16;

// Faster Up B
#[acmd_script( agent = "ike", script = "game_specialhi1", category = ACMD_GAME )]
unsafe fn ike_specialhi1_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::FT_MOTION_RATE(fighter, 0.7);
    
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
}

// Up air extra frame
#[acmd_script( agent = "ike", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn ike_attackairhi_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 12.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 8.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 4.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 11.0, 80, 94, 0, 55, 5.0, 0.0, 0.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);
    }


    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Fair extra frame
#[acmd_script( agent = "ike", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn ike_attackairf_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 13.0, 30, 60, 0, 55, 5.0, 0.0, 9.4, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 13.0, 30, 60, 0, 55, 5.0, 0.0, 6.2, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 13.0, 30, 60, 0, 55, 5.0, 0.0, 3.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

    }


    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Safer Downtilt
#[acmd_script( agent = "ike", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn ike_attacklw3_smash_script(fighter: &mut L2CAgentBase) {

    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.4, -1.0, 1.0, -4.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.4, -0.3, 4.0, -2.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.4, 0.7, 7.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 3, 0, Hash40::new("sword"), 8.0, 80, 55, 0, 70, 3.0, 1.5, 10.0, 2.5, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);

        macros::FT_MOTION_RATE(fighter, 0.9);
    }
}

// New Ground Down B
#[acmd_script( agent = "ike", scripts = ["game_speciallw","game_specialairlw"], category = ACMD_GAME )]
unsafe fn ike_speciallw_smash_script(fighter: &mut L2CAgentBase) {
        
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
    }

    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25);
    }

    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 50);
    }
    
}

// SFX
#[acmd_script( agent = "ike", script = "sound_speciallw", category = ACMD_SOUND )]
unsafe fn ike_sound_speciallw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ike_jump02"));
    }
    
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ike_attackair_f01"));
    }
}

// VFX
#[acmd_script( agent = "ike", script = "effect_speciallw", category = ACMD_EFFECT )]
unsafe fn ike_effect_speciallw_smash_script(fighter: &mut L2CAgentBase) {


    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("sword"), 0, 12, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }

}

// New Ground Down B Hit
#[acmd_script( agent = "ike", scripts = ["game_speciallwhit","game_specialairlwhit"], category = ACMD_GAME )]
unsafe fn ike_speciallwhit_smash_script(fighter: &mut L2CAgentBase) {
        
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("sword"), 35.0, 30, 60, 0, 55, 5.0, 0.0, 9.4, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("sword"), 35.0, 30, 60, 0, 55, 5.0, 0.0, 6.2, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("sword"), 35.0, 30, 60, 0, 55, 5.0, 0.0, 3.0, 0.0, None,None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_IKE, *ATTACK_REGION_SWORD);

    }

    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// SFX 
/*
#[acmd_script( agent = "ike", script = "sound_speciallwhit", category = ACMD_SOUND )]
unsafe fn ike_sound_speciallwhit_smash_script(fighter: &mut L2CAgentBase) {

}
*/

// VFX
#[acmd_script( agent = "ike", script = "effect_speciallwhit", category = ACMD_EFFECT )]
unsafe fn ike_effect_speciallwhit_smash_script(fighter: &mut L2CAgentBase) {

}

// Ike counter transition
pub unsafe extern "C" fn special_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    let mot;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = Hash40::new("special_air_lw_hit");
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = Hash40::new("special_lw_hit");
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    }
    GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(correct));
    if !cont {
        MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, mot, -1.0, 1.0, 0.0);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_IKE )]
fn ike_jumpquickdraw(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let is_status = status_kind == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH && MotionModule::frame(boma) >= 5.0;
        let motion_kind = MotionModule::motion_kind(boma);
        let frame = MotionModule::frame(boma);
        if is_status
        && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
        }
        //if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
        //&& frame > 114.0 {
        //    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        //    special_lw_mot_helper(fighter);
        //}
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        ike_specialhi1_smash_script,
        ike_attackairhi_smash_script,
        ike_attackairf_smash_script,
        ike_attacklw3_smash_script,
        ike_speciallw_smash_script,
        ike_sound_speciallw_smash_script,
        ike_effect_speciallw_smash_script,
        ike_speciallwhit_smash_script,
        //ike_sound_speciallwhit_smash_script,
        ike_effect_speciallwhit_smash_script


    );
    smashline::install_agent_frames!(
        ike_jumpquickdraw
    );
}
