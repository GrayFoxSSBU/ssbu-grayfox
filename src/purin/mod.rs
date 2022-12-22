use crate::prelude::*;



// Hitbox rollout charge
#[acmd_script( agent = "purin", script = "game_specialnhold", category = ACMD_GAME )]
unsafe fn purin_specialnhold_smash_script(fighter: &mut L2CAgentBase) {
    macros::ATTACK(fighter,0, 0, Hash40::new("top"), 0.5, 366, 50, 0, 60, 6.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
}
// Hitbox rollout charge max
#[acmd_script( agent = "purin", script = "game_specialnholdmax", category = ACMD_GAME )]
unsafe fn purin_specialnholdmax_smash_script(fighter: &mut L2CAgentBase) {
    macros::ATTACK(fighter,0, 0, Hash40::new("top"), 0.5, 366, 50, 0, 60, 6.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);    
}

// Rollout Bigger hitbox
#[acmd_script( agent = "purin", script = "game_specialn", category = ACMD_GAME )]
unsafe fn purin_specialn_smash_script(fighter: &mut L2CAgentBase) {
 
    macros::ATTACK(fighter,0, 0, Hash40::new("top"), 7.0, 30, 60, 0, 60, 5.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

    AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
}
// Rollout Turn hitbox
#[acmd_script( agent = "purin", script = "game_specialnturn", category = ACMD_GAME )]
unsafe fn purin_specialnturn_smash_script(fighter: &mut L2CAgentBase) {

    JostleModule::set_status(fighter.module_accessor, true);
    
    macros::ATTACK(fighter,0, 0, Hash40::new("top"), 7.0, 30, 60, 0, 60, 5.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
  
}

// Air Rollout Bigger hitbox
#[acmd_script( agent = "purin", script = "game_specialairn", category = ACMD_GAME )]
unsafe fn purin_specialairn_smash_script(fighter: &mut L2CAgentBase) {
 
    macros::ATTACK(fighter,0, 0, Hash40::new("top"), 10.0, 30, 60, 0, 60, 5.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

    AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
}

// Air Rollout Turn hitbox
#[acmd_script( agent = "purin", script = "game_specialairnturn", category = ACMD_GAME )]
unsafe fn purin_specialairnturn_smash_script(fighter: &mut L2CAgentBase) {

    JostleModule::set_status(fighter.module_accessor, true);
    
    macros::ATTACK(fighter,0, 0, Hash40::new("top"), 10.0, 30, 60, 0, 60, 5.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
  
}

// Bair BKB buff
#[acmd_script( agent = "purin", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn purin_attackairb_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 4.0);    
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 10.0);    
    if is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);    
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 42, 130, 0, 30, 4.5, 0.0, 4.0, -8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 42, 130, 0, 0, 4.0, 0.0, 4.0, -13.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

    }

    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Fair -1 endlag
#[acmd_script( agent = "purin", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn purin_attackairf_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 4.0);    
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 8.0);    
    if is_excute(fighter) {
       
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 98, 0, 30, 4.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 98, 0, 30, 3.5, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 98, 0, 30, 4.5, 0.0, 4.0, 8.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 98, 0, 30, 3.5, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }        

    frame(fighter.lua_state_agent, 21.0);    
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 27.0);    
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.9);
    }

    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
// Nair 1 frame faster
#[acmd_script( agent = "purin", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn purin_attackairn_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);    
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 3.0);    
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(fighter.lua_state_agent, 4.0);    
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 6.0);    
    if is_excute(fighter) {
       
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 100, 0, 30, 3.5, 0.0, 3.0, 2.0,Some(0.0), Some(3.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 361, 100, 0, 30, 6.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

    }

    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 70, 0, 50, 3.5, 0.0, 3.0, 2.0,Some(0.0), Some(3.5), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        

        AttackModule::clear(fighter.module_accessor,1,false);
    }        

    frame(fighter.lua_state_agent, 31.0);    
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 40.0);    
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
// Dair less SDI
#[acmd_script( agent = "purin", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn purin_attackairlw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 7.0);
    for _ in 0..8 {
        if is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 145, 100, 0, 10, 3.6, 0.0, -2.3, 1.0, Some(0.0), Some(-1.3), Some(3.5), 0.8, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 280, 100, 30, 0, 4.0, 0.0, 3.5, 1.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }


    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 45, 150, 0, 30, 5.0, 0.0, -2.4, 3.5, None, None, None, 2.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 45, 150, 0, 30, 4.0, 0.0, 3.5, 1.0, None, None, None, 2.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}
// Dtilt faster
#[acmd_script( agent = "purin", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn purin_attacklw3_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);    
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);

        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 10.0, 20, 68, 0, 50, 3.0, 0.0, -8.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 10.0, 20, 68, 0, 50, 3.0, 0.0, -4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 10.0, 20, 68, 0, 50, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        AttackModule::set_attack_height_all(fighter.module_accessor,AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::FT_MOTION_RATE(fighter, 0.8);
    }
}

// Up throw kill more
#[acmd_script( agent = "purin", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn purin_throwhi_smash_script(fighter: &mut L2CAgentBase) {
    
    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,0, 8.0, 90, 53, 0, 130, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0,  true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);

    macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_S,*COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);

    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {

        macros::CHECK_FINISH_CAMERA(fighter, 1, 32);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 0.0, y: 2.0, z: 0.0});    
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);

        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

// Rest stronger
#[acmd_script( agent = "purin", scripts = ["game_speciallwl","game_speciallwr","game_specialairlwl","game_specialairlwr"], category = ACMD_GAME )]
unsafe fn purin_speciallw_smash_script(fighter: &mut L2CAgentBase) {

    JostleModule::set_status(fighter.module_accessor, false);

    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 20.0, 90, 95, 0, 100, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_BODY);
    }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            JostleModule::set_status(fighter.module_accessor, true);
    }
}

// Sing better hitbox
#[acmd_script( agent = "purin", scripts = ["game_specialhil","game_specialhir","game_specialairhil","game_specialairhir"], category = ACMD_GAME )]
unsafe fn purin_specialhi_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }

    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }

    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }

    wait(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


// Shield cancel rollout
#[fighter_frame( agent = FIGHTER_KIND_PURIN )]
fn purin_frame(fighter: &mut L2CFighterCommon) {
    // shield cancel
    unsafe{
        let status = StatusModule::status_kind(fighter.module_accessor);
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if [
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, 
            *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD_MAX
        ].contains(&status) {
            println!("NSpecial check Passed");
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                println!("Ground check Passed");
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    println!("Press Shield check Passed");
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                    println!("Change to Shield state");
                }
            }
        }
    }
}

pub fn install() {

    smashline::install_acmd_scripts!(
        purin_specialnhold_smash_script,
        purin_specialnholdmax_smash_script,
        purin_specialn_smash_script,
        purin_specialnturn_smash_script,
        purin_specialairn_smash_script,
        purin_specialairnturn_smash_script,
        purin_speciallw_smash_script,
        purin_specialhi_smash_script,
        purin_attackairb_smash_script,
        purin_attackairf_smash_script,
        purin_attackairn_smash_script,
        purin_attackairlw_smash_script,
        purin_attacklw3_smash_script,
        purin_throwhi_smash_script

    );
    smashline::install_agent_frames!(
        purin_frame
        
    );
}
