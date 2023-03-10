use crate::prelude::*;


// Faster hookshot
#[acmd_script( agent = "younglink", script = "game_aircatch", category = ACMD_GAME )]
unsafe fn younglink_aircatch_smash_script(fighter: &mut L2CAgentBase) {
        
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {

        macros::FT_MOTION_RATE(fighter, 0.5);

        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, false,-1);
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND,false,-1);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 4.0, 60, 20, 0, 70, 2.7, 0.0, 0.0, -0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

        AttackModule::set_add_reaction_frame(fighter.module_accessor,0, 4.0, false);
        ArticleModule::change_status(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

   
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND,Hash40::new("shoot"),true,0.0);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        macros::FT_MOTION_RATE(fighter, 0.66);

        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND,Hash40::new("back"),false,0.0);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }

    sv_animcmd::frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
    sv_animcmd::frame(fighter.lua_state_agent, 72.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_YOUNGLINK_GENERATE_ARTICLE_HOOKSHOT_HAND,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
        
}

// bigger ledge attack
#[acmd_script( agent = "younglink", script = "game_cliffattack", category = ACMD_GAME )]
unsafe fn younglink_cliffattack_smash_script(fighter: &mut L2CAgentBase) {

    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, 14.5, Some(0.0), Some(5.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }

    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        younglink_aircatch_smash_script,
        younglink_cliffattack_smash_script

    );
}
