use crate::prelude::*;

// Stronger Bair
#[acmd_script( agent = "fox", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn fox_attackairb_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 1.0);
    
        macros::ATTACK(fighter,0, 0, Hash40::new("kneer"), 13.0, 30, 93, 0, 20, 4.6, 5.9, -0.9, 1.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    
        macros::ATTACK(fighter,1, 0, Hash40::new("kneer"), 13.0, 30, 93, 0, 20, 4.6, 0.9, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {

        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
// D Throw combo
#[acmd_script( agent = "fox", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn fox_throwlw_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0, 1.0, 60, 105, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0, 1.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_FOX_GENERATE_ARTICLE_BLASTER);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("open"), false, -1.0);
        }
    }

    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET,false,-1);
    }

    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET,false,-1);
    }

    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET,false,-1);
    }

    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        
        macros::CHECK_FINISH_CAMERA(fighter, 1, 1);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.6);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 0.0, y: 6.0, z: 0.0});   
    }

    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {

        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        
    }

    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_FOX_GENERATE_ARTICLE_BLASTER);
        if sv_animcmd::IS_EXIST_ARTICLE(fighter.lua_state_agent) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40::new("close"), false, -1.0);
        }
    }

    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 0.7);
    }

}

// F Throw tech chase
#[acmd_script( agent = "fox", script = "game_throwf", category = ACMD_GAME )]
unsafe fn fox_throwf_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0, 3.0, 35, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter,0, 0, Hash40::new("top"), 4.0, 48, 100, 140, 10, 5.5, 0.0, 6.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ELBOW);
        
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 23, 4);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.3);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {

        
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        AttackModule::clear_all(fighter.module_accessor);
        
    }

    frame(fighter.lua_state_agent,21.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter,0.7)

    }
}

// Jump cancel Shine
#[fighter_frame( agent = FIGHTER_KIND_FOX )]
fn fox_jumpshine(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status = StatusModule::status_kind(fighter.module_accessor);
        if [
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END
        ].contains(&status) 
        || (status == *FIGHTER_STATUS_KIND_SPECIAL_LW && MotionModule::frame(boma) >= 3.0) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            fighter.sub_transition_group_check_ground_jump_mini_attack();
            fighter.sub_transition_group_check_ground_jump();
            fighter.sub_transition_group_check_air_jump_aerial();
        }
    }
}
    
pub fn install() {

    smashline::install_acmd_scripts!(
        fox_attackairb_smash_script,
        fox_throwlw_smash_script,
        fox_throwf_smash_script
    );

    smashline::install_agent_frames!(
        fox_jumpshine
    );
}
