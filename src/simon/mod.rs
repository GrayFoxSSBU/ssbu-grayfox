use crate::prelude::*;

// Faster Axe throw
#[acmd_script( agent = "simon", scripts = ["game_specialn","game_specialairn"], category = ACMD_GAME )]
unsafe fn simon_specialn_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_SIMON_GENERATE_ARTICLE_AXE, false,0);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor,*FIGHTER_SIMON_GENERATE_ARTICLE_AXE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
}

// More Range + Faster Grab
#[acmd_script( agent = "simon", script = "game_catch", category = ACMD_GAME )]
unsafe fn simon_catch_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 6.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 6.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor,true);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        
        macros::CATCH(fighter,0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);

        macros::CATCH(fighter,1, Hash40::new("top"), 1.65, 6.6, 9.0, 2.35, Some(0.0), Some(6.6), Some(12.35), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);

        macros::game_CaptureCutCommon(fighter);
    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);

        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);

        GrabModule::set_rebound(fighter.module_accessor,false);
    }
}

// Up throw less endlag
#[acmd_script( agent = "simon", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn simon_throwhi_smash_script(fighter: &mut L2CAgentBase) {

        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {             
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 90, 82, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }

        frame(fighter.lua_state_agent, 35.0);
        if is_excute(fighter) {

            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 89, 100, 0, 55, 2.0, 0.0, 23.0, 0.0, Some(0.0), Some(50.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            macros::CHECK_FINISH_CAMERA(fighter, 3, 34);
            lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 2.0);
            lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 0.0, y: 12.0, z: 0.0});
        }
        frame(fighter.lua_state_agent, 36.0);
        if is_excute(fighter) {
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);

            macros::FT_MOTION_RATE(fighter, 0.8);
        }
}

// Down Throw Less KB, Less endlag
#[acmd_script( agent = "simon", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn simon_throwlw_smash_script(fighter: &mut L2CAgentBase) {

        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {             
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 57, 65, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }

        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {

            fighter.clear_lua_stack();
            lua_args!(fighter, 10, 1);
            sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
            fighter.clear_lua_stack();      
            macros::CHECK_FINISH_CAMERA(fighter, 5, 0);
        }

        frame(fighter.lua_state_agent, 24.0);
        if is_excute(fighter) {
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);

            macros::FT_MOTION_RATE(fighter, 0.8);
        }
}

// Bigger/more active tippers

// Fair
#[acmd_script( agent = "simon", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn simon_attackairf_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 55, 0, 80, 2.8, 0.0, 8.0, 40.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 55, 0, 80, 2.3, 0.0, 8.0, 7.0, Some(0.0), Some(8.0), Some(40.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 8.0, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairfhi", category = ACMD_GAME )]
unsafe fn simon_attackairfhi_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 55, 0, 80, 2.8, 0.0, 23.0, 38.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 55, 0, 80, 2.3, 0.0, 10.0, 7.0, Some(0.0), Some(23.0), Some(38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 10.0, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairflw", category = ACMD_GAME )]
unsafe fn simon_attackairflw_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 55, 0, 80, 2.8, 0.0, -7.0, 38.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 55, 0, 80, 2.3, 0.0, 6.0, 7.0, Some(0.0), Some(-7.0), Some(38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 55, 0, 80, 5.5, 0.0, 6.0, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Bair
#[acmd_script( agent = "simon", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn simon_attackairb_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 70, 0, 70, 2.8, 0.0, 8.0, -40.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 70, 0, 70, 2.3, 0.0, 8.0, -7.0, Some(0.0), Some(8.0), Some(-40.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 70, 0, 70, 5.5, 0.0, 8.0, -5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairbhi", category = ACMD_GAME )]
unsafe fn simon_attackairbhi_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 70, 0, 70, 2.8, 0.0, 23.0, -38.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 70, 0, 70, 2.3, 0.0, 10.0, -7.0, Some(0.0), Some(23.0), Some(-38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 70, 0, 70, 5.5, 0.0, 10.0, -5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairblw", category = ACMD_GAME )]
unsafe fn simon_attackairblw_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 70, 0, 70, 2.8, 0.0, -7.0, -38.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 361, 70, 0, 70, 2.3, 0.0, 6.0, -7.0, Some(0.0), Some(-7.0), Some(-38.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 361, 70, 0, 70, 5.5, 0.0, 6.0, -5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "simon", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn simon_attackairhi_smash_script(fighter: &mut L2CAgentBase) {
    
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_CLIFF_RAY_CHECK);
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LASSO_FLAG_CHECK);
   
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 90, 75, 0, 60, 3.0, 0.0, 53.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 90, 75, 0, 60, 2.5, 0.0, 17.0, 2.5, Some(0.0), Some(53.0), Some(2.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SIMON_WHIP, *ATTACK_REGION_WHIP);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 90, 75, 0, 60, 5.5, 0.0, 19.0, 2.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }

    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        simon_specialn_smash_script,
        simon_catch_smash_script,
        simon_throwhi_smash_script,
        simon_throwlw_smash_script,
        simon_attackairf_smash_script,
        simon_attackairfhi_smash_script,
        simon_attackairflw_smash_script,
        simon_attackairb_smash_script,
        simon_attackairbhi_smash_script,
        simon_attackairblw_smash_script,
        simon_attackairhi_smash_script
    );
}
