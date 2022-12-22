use crate::prelude::*;



// Shield cancel Cannonball
#[fighter_frame( agent = FIGHTER_KIND_KOOPAJR )]
fn koopajr_cannonball(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
        let is_status = status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD;
        if is_status {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            fighter.sub_transition_group_check_ground_guard();
            fighter.sub_transition_group_check_air_escape();
        }
    }
}

// ftilt reduce endlag
#[acmd_script( agent = "koopajr", scripts = ["game_attacks3","game_attacks3hi"], category = ACMD_GAME )]
unsafe fn koopajr_attacks3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 6.0, 361, 70, 0, 72, 2.3, 0.0, 0.0, 0.0, Some(4.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 8.0, 361, 70, 0, 72, 2.8, 8.5, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

    }
    
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.65);
    }
    
}
//angled down ftilt
#[acmd_script( agent = "koopajr", script = "game_attacks3lw", category = ACMD_GAME )]
unsafe fn koopajr_attacks3lw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 6.0, 361, 70, 0, 72, 2.3, 0.0, 0.0, 0.0, Some(4.5), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 8.0, 361, 70, 0, 72, 2.8, 8.5, 0.0, 0.0, Some(12.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);

    }
    
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.65);
    }
    
}

//-fair animation speed changed so frame 10 is the start of the upward swing
#[acmd_script( agent = "koopajr", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn koopajr_attackairf_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 2.0);
    }

    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }


    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 11.0, 45, 93, 0, 33, 6.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 9.0, 45, 93, 0, 33, 3.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }   

    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 9.0, 45, 93, 0, 33, 5.5, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 7.0, 45, 93, 0, 33, 3.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }

    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 7.0, 45, 93, 0, 33, 5.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 5.0, 45, 93, 0, 33, 2.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 33.0);
        if is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            macros::FT_MOTION_RATE(fighter, 0.9);
    }
}

// larger hitbox on landing fair +1 frame
#[acmd_script( agent = "koopajr", script = "game_landingairf", category = ACMD_GAME )]
unsafe fn koopajr_landingairf_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_LANDING_ATTACK_AIR_SQUAT);
    }

    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("clownarml1"), 2.0, 60, 100, 60, 0, 4.0, 2.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("clownarml1"), 2.0, 60, 100, 60, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }   

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

}


//-increase dair ac window by one so it can be autocanceled like smash4
#[acmd_script( agent = "koopajr", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn koopajr_attackairlw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {

        JostleModule::set_status(fighter.module_accessor,false);
    }   

    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 270, 10, 0, 30, 3.6, 0.0, 4.0, -2.0, Some(0.0), Some(4.0), Some(2.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 96, 10, 0, 30, 4.0, 0.0, -2.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }

    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);

        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 2.5, 64, 124, 0, 70, 4.0, 0.0, 2.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 2.5, 64, 124, 0, 70, 4.5, 0.0, -3.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);

        JostleModule::set_status(fighter.module_accessor,true);

        macros::FT_MOTION_RATE(fighter, 0.7);
    }

    frame(fighter.lua_state_agent, 60.0);
        if is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//dair landing less knockback growth
#[acmd_script( agent = "koopajr", script = "game_landingairlw", category = ACMD_GAME )]
unsafe fn koopajr_landingairlw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_LANDING_ATTACK_AIR_SQUAT);

        JostleModule::set_status(fighter.module_accessor,true);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 50, 140, 0, 40, 5.0, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }   

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

}

//-reduce endlag to mech 
#[acmd_script( agent = "koopajr", scripts = ["game_speciallw","game_specialairlw"], category = ACMD_GAME )]
unsafe fn koopajr_specialw_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor,*FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, -1);
    }   

}


// Frame 9 standing grab
#[acmd_script( agent = "koopajr", script = "game_catch", category = ACMD_GAME )]
unsafe fn koopajr_catch_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor,true);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter,0, Hash40::new("top"), 4.0, 0.0, 6.6, 6.5, Some(0.0), Some(6.6), Some(14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);

        macros::CATCH(fighter,1, Hash40::new("top"), 2.0, 0.0, 6.6, 4.5, Some(0.0), Some(6.6), Some(16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);

        macros::game_CaptureCutCommon(fighter);
    }

    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {

        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);

        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);

        GrabModule::set_rebound(fighter.module_accessor,false);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        koopajr_cannonball
    );

    smashline::install_acmd_scripts!(
        koopajr_attacks3lw_smash_script,
        koopajr_attacks3_smash_script,
        koopajr_attackairf_smash_script,
        koopajr_landingairf_smash_script,
        koopajr_attackairlw_smash_script,
        koopajr_landingairlw_smash_script,
        koopajr_specialw_smash_script,
        koopajr_catch_smash_script
    );
}
