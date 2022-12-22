use crate::prelude::*;


// Lloid Trap Hitbox comes out sooner
#[acmd_script( agent = "shizue_clayrocket", script = "game_fly", category = ACMD_GAME )]
unsafe fn shizue_clayrocket_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 96, 100, 70, 0, 6.0, 0.0, 0.5, 0.0, Some(0.0), Some(-5.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 8, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }

    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.9, 89, 10, 0, 70, 6.0, 0.0, 0.5, 0.0, None, None, None, 0.3, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 9, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

// Increase power of forward tilt
#[acmd_script( agent = "shizue", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn shizue_attacks3_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,false,-1);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 9.0, 361, 100, 0, 56, 4.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 9.0, 361, 100, 0, 56, 2.5, -5.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 9.0, 361, 100, 0, 56, 2.5, 5.0, 7.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 9.0, 361, 100, 0, 56, 2.0, 0.0, 4.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }

    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor,*FIGHTER_SHIZUE_GENERATE_ARTICLE_UMBRELLA,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// Slightly increase range of down-smash
#[acmd_script( agent = "shizue", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn shizue_attacklw4_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET,false,-1);
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET, Hash40::new("attack"),false,-1.0);
    }

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 28, 104, 0, 35, 5.0, 0.0, 3.6, 11.5, Some(0.0),Some(3.6),Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 28, 104, 0, 35, 3.3, 0.0, 3.6, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 28, 105, 0, 40, 5.0, 0.0, 3.6, -11.5, Some(0.0),Some(3.6),Some(-12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 28, 105, 0, 40, 3.3, 0.0, 3.6, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        ArticleModule::remove(fighter.module_accessor,*FIGHTER_SHIZUE_GENERATE_ARTICLE_BUCKET,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// Significantly increase size of up-smash’s hitbox
#[acmd_script( agent = "shizue_trafficsign", script = "game_attack", category = ACMD_GAME )]
unsafe fn shizue_trafficsign_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::FT_MOTION_RATE(fighter, 0.458);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.85);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 88, 116, 0, 32, 3.3, 0.0, 3.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);

        macros::ATTACK(fighter, 1, 0, Hash40::new("trafficsign"), 12.0, 88, 116, 0, 32, 4.8, 0.0, 3.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }

    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Faster Grab
#[acmd_script( agent = "shizue", script = "game_catch", category = ACMD_GAME )]
unsafe fn shizue_catch_smash_script(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor,true);
    }

    frame(fighter.lua_state_agent, 14.0);
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

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::UNABLE_AREA(fighter,*FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }

    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor,*FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET,ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }

}

pub fn install() {

    smashline::install_acmd_scripts!(
        shizue_catch_smash_script,
        shizue_clayrocket_smash_script,
        shizue_attacks3_smash_script,
        shizue_attacklw4_smash_script,
        shizue_trafficsign_smash_script
    );
}
