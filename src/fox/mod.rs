use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterBase,L2CFighterCommon};
use acmd::{acmd, acmd_func};


#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON,
battle_object_kind = WEAPON_KIND_FOX_BLASTER_BULLET,
animation = "fly",
animcmd = "game_fly")]
pub fn fox_laser(fighter : &mut L2CFighterBase) {
    acmd!({
        if(is_excute){
         ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=50.0, Angle=50, KBG=140, FKB=0, BKB=140, Size=5.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            AttackModule::enable_safe_pos()
        }
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_FOX,
animation = "attack_air_lw",
animcmd = "game_attackairlw")]
pub fn fox_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=5)
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    for(6 Iterations){
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.4, Angle=325, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=8.2, Z=-0.5, X2=0.0, Y2=9.0, Z2=2.0, Hitlag=0.66, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("toel"), Damage=3.4, Angle=110, KBG=30, FKB=0, BKB=15, Size=4.2, X=-0.5, Y=-0.5, Z=0.0, X2=-0.5, Y2=-0.5, Z2=0.0, Hitlag=0.66, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=1)
    }
    if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=270, KBG=140, FKB=0, BKB=50, Size=5.0, X=0.0, Y=9.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=270, KBG=140, FKB=0, BKB=50, Size=7.0, X=0.0, Y=2.8, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_KICK)
    }
    wait(Frames=1)
    if(is_excute){
        AttackModule::clear_all()
    }
    frame(Frame=28)
    if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
    });

}
pub fn install() {
    acmd::add_hooks!(
        //fox_laser,
        fox_dair);
}
