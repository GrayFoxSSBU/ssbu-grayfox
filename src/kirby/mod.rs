use crate::prelude::*;

// Faster inhale
// Faster Startup
#[acmd_script( agent = "kirby", scripts = ["game_specialnstart","game_specialairnstart"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialnstart_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.33);
    }

    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);

        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_INHALE);

        macros::CATCH(fighter,0, Hash40::new("top"), 6.0, 0.0, 7.0, 5.0,  None, None, None,*FIGHTER_STATUS_KIND_SWALLOWED,*COLLISION_SITUATION_MASK_GA);

        macros::SEARCH(fighter,0, 0, Hash40::new("top"), 5.0, 0.0, 7.0, 8.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
}

// Bigger inhale
#[acmd_script( agent = "kirby", scripts = ["game_specialnloop","game_specialairnloop"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialnloop_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KIRBY_STATUS_SPECIAL_N_FLAG_INHALE);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 160, 100, 30, 0, 8.5, 0.0, 7.0, 15.5, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 160, 100, 25, 0, 7.8, 0.0, 7.0, 9.7, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);

        macros::CATCH(fighter,0, Hash40::new("top"), 5.8, 0.0, 6.4, 14.2,  None, None, None,*FIGHTER_STATUS_KIND_SWALLOWED,*COLLISION_SITUATION_MASK_GA);

        macros::CATCH(fighter,1, Hash40::new("top"), 5.8, 0.0, 7.0, 6.2,  None, None, None,*FIGHTER_STATUS_KIND_SWALLOWED,*COLLISION_SITUATION_MASK_GA);

        macros::SEARCH(fighter,0, 0, Hash40::new("top"), 3.0, 0.0, 7.0, 8.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);

        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 00, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
}

// Faster Aerial Stone
#[acmd_script( agent = "kirby", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialairlw_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_air_lw"),false,-1.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_BLINK_ONOFF);
    }

    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 25);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, Hash40::new("special_lw_to_air"),false,-1.0);
        AttackModule::clear(fighter.module_accessor,1,false);

        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 70, 76, 0, 69, 6.5, 0.0, 2.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);

        AttackModule::init_attack_pos(fighter.module_accessor,0);
    }
}

// Up Special buffs
// Faster Startup
#[acmd_script( agent = "kirby", scripts = ["game_specialhi","game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialhi_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, -1.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, false, 0);
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi"), false, -1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

//Better ledge grab and Spike, earlier hitbox
#[acmd_script( agent = "kirby", scripts = ["game_specialhi2","game_specialairhi2"], category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialhi2_smash_script(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        ArticleModule::change_motion(fighter.module_accessor,*FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, Hash40::new("special_hi2"), false, -1.0);
        // grab ledge before hitbox
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 117, 0, 3.5, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 93, 100, 117, 0, 3.5, 0.0, 3.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 100, 102, 0, 3.5, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);

        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 93, 100, 102, 0, 3.5, 0.0, 13.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 85, 100, 60, 0, 3.5, 0.0, 3.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 93, 100, 60, 0, 3.5, 0.0, 3.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 90, 100, 50, 0, 3.5, 0.0, 13.5, 7.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 93, 100, 50, 0, 3.5, 0.0, 13.5, 18.0, None, None, None, 1.0, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        //macros::FT_MOTION_RATE(fighter, 0.5);
    }
    
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }

    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        // buff damage, size, and KB
        
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 4.0, 275, 100, 130, 0, 6.5, 0.0, 8.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 20, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            
    }
}

// Faster Nair
#[acmd_script( agent = "kirby", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn kirby_attackairn_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }

    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 46, 117, 0, 35, 9.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 46, 90, 0, 30, 8.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 46, 90, 0, 30, 7.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 4.0, 46, 90, 0, 30, 6.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }

    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Stronger Bair
#[acmd_script( agent = "kirby", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn kirby_attackairb_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }

    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 361, 112, 0, 15, 4.5, 0.0, 4.5, -13.0, Some(0.0), Some(4.5), Some(-6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 112, 0, 0, 3.5, 0.0, 4.5, -11.0, Some(0.0), Some(4.5), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Stronger Up air
#[acmd_script( agent = "kirby", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn kirby_attackairhi_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
    }

    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    

    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 12.0, 70, 105, 0, 20, 4.0, 0.0, -5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);

        macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 12.0, 70, 105, 0, 20, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Faster Dair + Stronger
#[acmd_script( agent = "kirby", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn kirby_attackairlw_smash_script(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }

    for _ in 0..5 {
        if is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 367, 100, 0, 20, 6.0, 0.0, -2.4, 3.0, Some(0.0), Some(2.3), Some(0.5), 0.8, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
            wait(fighter.lua_state_agent, 2.0);
            if is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
            }
            wait(fighter.lua_state_agent, 1.0);
        }

    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 270, 110, 0, 30, 5.6, 0.0, -2.5, 2.4, Some(0.0), Some(0.0), Some(1.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

//Buff Kirby up throw
#[acmd_script( agent = "kirby", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn kirby_throwhi_smash_script(fighter: &mut L2CAgentBase) {

        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {             
            macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5); 
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 85, 90, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
        }
        frame(fighter.lua_state_agent, 45.0);
        if is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_STATUS_THROW_FLAG_STOP);
            macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 7.0, 65, 95, 0, 85, 9.5, 0.0, 6.5, 2.0, None, None,None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"),*ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        frame(fighter.lua_state_agent, 47.0);
        if is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::set_float(fighter.module_accessor,5.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FINISH_CAMERA_THROW_RAY_LENGTH);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_RAY_CHECK_FINISH_CAMERA_THROW);
            macros::CHECK_FINISH_CAMERA(fighter, 15, 7);

            lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.8);
            lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, smash::phx::Vector3f{x: 5.0, y: 0.0, z: 0.0});
        }
        frame(fighter.lua_state_agent, 51.0);
        if is_excute(fighter) {
            let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        }
}

#[acmd_script( agent = "kirby", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn kirby_specialairsstart_smash_script(fighter: &mut L2CAgentBase) {

        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {             
            
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, false, -1);
            macros::FT_MOTION_RATE(fighter, 0.5);
        }
}





pub fn install() {
    smashline::install_acmd_scripts!(
        kirby_attackairn_smash_script,
        kirby_attackairb_smash_script,
        kirby_attackairhi_smash_script,
        kirby_attackairlw_smash_script,
        kirby_specialnstart_smash_script,
        kirby_specialnloop_smash_script,
        kirby_specialairlw_smash_script,
        kirby_specialhi_smash_script,
        kirby_specialhi2_smash_script,
        kirby_throwhi_smash_script,
        kirby_specialairsstart_smash_script
    );
}