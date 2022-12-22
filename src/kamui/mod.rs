use crate::prelude::*;
pub const SITUATION_KIND: i32 = 0x16;
pub const STATUS_KIND:  i32 = 0xB;


// Stronger landing dair
#[acmd_script( agent = "kamui", script = "game_landingairlw", category = ACMD_GAME )]
unsafe fn kamui_landingairlw_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 110, 0, 65, 7.0, 0.0, 4.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);

        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 110, 0, 65, 7.0, 0.0, 4.0, -4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_NONE);
    }
    
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {

        AttackModule::clear_all(fighter.module_accessor);
    }

}

// Dragon Surge ACMD
#[acmd_script( agent = "kamui", scripts = ["game_speciallwhit", "game_specialairlwhit"] , category = ACMD_GAME , low_priority)]
unsafe fn kamui_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 30.0/26.0);
        ArticleModule::generate_article(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(boma, false);
        ArticleModule::set_visibility_whole(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 37.0/(66.0 - 31.0));
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(boma, true);
    }
}

#[acmd_script( agent = "kamui", scripts = ["effect_speciallwhit", "effect_specialairlwhit"] , category = ACMD_EFFECT , low_priority)]
unsafe fn kamui_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 30.0/26.0);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_guard_mark"), true, true);
        EFFECT(fighter, Hash40::new("kamui_counter_success"), Hash40::new("top"), 0, 14, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("kamui_water_hamon"), Hash40::new("top"), 0, 0.5, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    if WorkModule::is_flag(boma, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 14.8, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
    EFFECT(fighter, Hash40::new("kamui_water_splash"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
    WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), -7, 0, 15, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), -7, 0, 9, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
        }
        else {
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 7, 0, 15, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("kamui_counter_splash"), Hash40::new("top"), 7, 0, 9, 0, 0, 0, 0.45, 0, 0, 0, 0, 0, 0, true);
        }
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            EFFECT(fighter, Hash40::new("kamui_counter_ripple"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 37.0/(66.0 - 31.0));
    }
    frame(lua_state, 47.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
        WorkModule::on_flag(boma, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
}

#[acmd_script( agent = "kamui_waterdragon", scripts = ["game_speciallwhit", "game_specialairlwhit"] , category = ACMD_GAME , low_priority)]
unsafe fn kamui_waterdragon_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    FT_MOTION_RATE(fighter, 30.0/26.0);
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 17.0, 80, 60, 0, 90, 8.0, 0.0, 8.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 80, 60, 0, 90, 6.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
    }
    if WorkModule::is_flag(boma, *WEAPON_KAMUI_WATERDRAGON_INSTANCE_WORK_ID_FLAG_SET_CRITICAL_HIT_SE) {
        if is_excute(fighter) {
            AttackModule::set_optional_hit_sound(boma, 0, Hash40::new("se_kamui_criticalhit"));
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 5.0, 0.0, 21.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 6.0, 0.0, 6.0, -3.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 1, true, false);
        AttackModule::set_force_reaction(boma, 2, true, false);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 80, 60, 0, 90, 7.0, 0.0, 10.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 80, 60, 0, 90, 5.0, 0.0, 22.0, 15.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(boma, 0, true, false);
        AttackModule::set_force_reaction(boma, 1, true, false);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 37.0/(66.0 - 31.0));
        AttackModule::clear_all(boma);
    }
}  

// Dragon Surge Status
#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    special_lw_mot_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_mot_helper(fighter: &mut L2CFighterCommon) {
    let cont = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
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
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !cont {
        MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, mot, -1.0, 1.0, 0.0);
    }
    // <HDR>
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, mot, cont, -1.0);
    }
    // </HDR>
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_lw_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    0.into()
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
            ArticleModule::remove(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_whole(fighter.module_accessor, true);
        }
    }
    0.into()
}


pub fn install() {

    smashline::install_acmd_scripts!(
        kamui_landingairlw_smash_script,
        kamui_special_lw_hit_game,
        kamui_special_lw_hit_effect,
        kamui_waterdragon_special_lw_hit_game
    );
    smashline::install_status_scripts!(
        special_lw_main, special_lw_end
    );
}
