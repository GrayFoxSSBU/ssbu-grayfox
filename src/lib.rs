#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
//#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(unused_assignments)]

pub mod globals {
  pub const UNK1: 				 i32 = 0x0; //void
  pub const UNK2:                  i32 = 0x1; //i32
  pub const FIGHTER_KIND:          i32 = 0x2; //i32
  pub const OBJECT_ID:             i32 = 0x3; //i32
  pub const UNK3:                  i32 = 0x4; //ptr, very close in value to UNK6 and the last 5 digits of both values don't change on reboot, does NOT change by player number
  pub const UNK4:                  i32 = 0x5; //ptr
  pub const UNK5:                  i32 = 0x6; //void
  pub const INIT_STATUS_FUNC:      i32 = 0x7; //ptr
  pub const IN_HITLAG:             i32 = 0x8; //bool
  pub const STATUS_KIND_INTERRUPT: i32 = 0x9; //i32
  pub const PREV_STATUS_KIND:      i32 = 0xA; //i32
  pub const STATUS_KIND:           i32 = 0xB; //i32
  pub const UNK6:                  i32 = 0xC; //i32, varies by fighter_kind but idk what the pattern is. Prints 480 for Falcon, 479 for Giga Bowser, 502 for Ryu
  pub const UNK7:                  i32 = 0xD; //bool
  pub const MOTION_FRAME:          i32 = 0xE; //f32
  pub const MOTION_FRAME_NO_INTERP:i32 = 0xF; //f32
  pub const UNK8:                  i32 = 0x10; //ptr, pointer to where the status_kind's function is located?
  pub const UNK9:                  i32 = 0x11; //ptr, equal to UNK10, does NOT change by player number
  pub const UNK10:                 i32 = 0x12; //ptr
  pub const UNK11:                 i32 = 0x13; //this bitch changes types (i32/ptr)
  pub const PREV_SUB_STATUS:       i32 = 0x14; //i32
  pub const SUB_STATUS:            i32 = 0x15; //i32
  pub const SITUATION_KIND:        i32 = 0x16; //i32
  pub const PREV_SITUATION_KIND:   i32 = 0x17; //i32
  pub const UNK12:				         i32 = 0x18; //f32, status kind related, occasionally matches StatusModule::status_kind_next
  pub const UNK13:                 i32 = 0x19; //i32
  pub const STICK_X:               i32 = 0x1A; //f32
  pub const STICK_Y:               i32 = 0x1B; //f32
  pub const FLICK_X:               i32 = 0x1C; //i32
  pub const FLICK_Y:               i32 = 0x1D; //i32
  pub const FLICK_Y_DIR:           i32 = 0x1E; //f32
  pub const PAD_FLAG:              i32 = 0x1F; //u64
  pub const CMD_CAT1:              i32 = 0x20; //u64
  pub const CMD_CAT2:              i32 = 0x21; //u64
  pub const CMD_CAT3:              i32 = 0x22; //u64
  pub const CMD_CAT4:              i32 = 0x23; //u64
  // 0x24
  // 0x25
  // 0x26
  // 0x27
  // 0x28 some substatus
  pub const DASH_CALLBACK:         i32 = 0x29;
  // 0x2A
  pub const CUSTOM_ROUTINE:        i32 = 0x2B;
  // 0x2C
  // 0x2D
  // 0x2E
  // 0x2F
  // 0x30
  // 0x31
  // 0x32 some substatus
  pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
  pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
  pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
  pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;  
}



pub mod prelude {
  pub use {
    smash::{
        lua2cpp::{self, *},
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::{sv_module_access::*, lua_bind::*, sv_math, smashball, *},
        lib::{lua_const::{self, *}, L2CValueType::*, L2CValueType, L2CAgent, L2CValue, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner},
        app::sv_animcmd::ATK_HIT_ABS,
        app::sv_battle_object::module_accessor,
        app::*,
        app::{self, lua_bind::*, sv_animcmd::{frame, wait}, *},
        app::BattleObjectModuleAccessor,
        phx::Vector2f,
        phx::Vector3f
    },
    smashline::*,
    smash_script::*,
    smash_script::{macros::*, *},
    crate::FIGHTER_CUTIN_MANAGER,
    skyline::{install_hooks},
    skyline::hooks::{getRegionAddress, Region}
  };

}

mod mario;
mod donkey;
mod link;
mod samus;
mod samusd;
mod yoshi;
mod kirby;
mod fox;
mod pikachu;
mod luigi;
mod ness;
mod captain;
mod purin;
mod koopa;
mod sheik;
mod zelda;
mod pichu;
mod falco;
mod marth;
mod younglink;
mod ganon;
mod mewtwo;
mod metaknight;
mod pit;
mod pitb;
mod ike;
mod pzenigame;
mod pfushigisou;
mod plizardon;
mod lucas;
mod sonic;
mod dedede;
//mod pikmin;
mod lucario;
mod robot;
mod toonlink;
mod wolf;
mod murabito;
mod rockman;
mod wiifit;
mod rosetta;
mod littlemac;
mod gekkouga;
mod shulk;
mod reflet;
mod koopajr;
mod duckhunt;
mod kamui;
mod custom;
mod cloud;
mod bayonetta;
mod inkling;
mod ridley;
mod simon;
mod richter;
mod krool;
mod shizue;
mod gaogaen;
mod packun;
mod buddy;
mod master;
mod chrom;
mod lucina;

extern "C"{
  #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
  pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
}

#[skyline::main(name = "ssbu_grayfox")]
pub fn main() {

    mario::install();
    donkey::install();
    link::install();
    samus::install();
    samusd::install();
    yoshi::install();
    kirby::install();
    fox::install();
    pikachu::install();
    luigi::install();
    ness::install();
    captain::install();
    purin::install();
    koopa::install();
    sheik::install();
    zelda::install();
    pichu::install();
    falco::install();
    marth::install();
    younglink::install();
    ganon::install();
    mewtwo::install();
    metaknight::install();
    pit::install();
    pitb::install();
    ike::install();
    pzenigame::install();
    pfushigisou::install();
    plizardon::install();
    lucas::install();
    sonic::install(); 
    dedede::install();
    //pikmin::install();
    lucario::install();
    robot::install();
    toonlink::install();
    wolf::install(); 
    murabito::install();
    rockman::install();
    wiifit::install();
    rosetta::install();
    littlemac::install();
    gekkouga::install();
    shulk::install();
    reflet::install();
    koopajr::install();
    duckhunt::install();
    custom::install();
    kamui::install();
    cloud::install();
    bayonetta::install();
    inkling::install();
    ridley::install();
    simon::install();
    richter::install();
    krool::install();
    shizue::install();
    gaogaen::install();
    packun::install();
    buddy::install();
    master::install();
    chrom::install();
    lucina::install();
}