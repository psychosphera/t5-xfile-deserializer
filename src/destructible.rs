use crate::{*, xmodel::PhysConstraintsRaw, fx::FxEffectDefRaw};

#[derive(Clone, Debug, Deserialize)]
pub struct DestructibleDefRaw<'a> {
    pub name: XString<'a>,
    pub model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub pristine_model: Ptr32<'a, xmodel::XModelRaw<'a>>,
    pub pieces: FatPointerCountFirstU32<'a, DestructiblePieceRaw<'a>>,
    pub client_only: i32,
}
assert_size!(DestructibleDefRaw, 24);

pub struct DestructibleDef {
    pub name: String,
    pub model: Option<Box<xmodel::XModel>>,
    pub pristine_model: Option<Box<xmodel::XModel>>,
    pub pieces: Vec<DestructiblePiece>,
    pub client_only: bool,
}

impl<'a> XFileInto<DestructibleDef> for DestructibleDefRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> DestructibleDef {
        DestructibleDef { 
            name: self.name.xfile_into(&mut xfile), 
            model: self.model.xfile_into(&mut xfile), 
            pristine_model: self.pristine_model.xfile_into(&mut xfile), 
            pieces: self.pieces.xfile_into(xfile), 
            client_only: self.client_only != 0 
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DestructiblePieceRaw<'a> {
    pub stages: [DestructibleStageRaw<'a>; 5],
    pub parent_piece: u8,
    unused: [u8; 3],
    pub parent_damage_percent: f32,
    pub bullet_damage_scale: f32,
    pub explosive_damage_scale: f32,
    pub melee_damage_scale: f32,
    pub impact_damage_scael: f32,
    pub entity_damage_transfer: f32,
    pub phys_constraints: Ptr32<'a, PhysConstraintsRaw<'a>>,
    pub health: i32,
    pub damage_sound: XString<'a>,
    pub burn_effect: Ptr32<'a, FxEffectDefRaw<'a>>,
    pub burn_sound: XString<'a>,
    pub enable_label: u16,
    unused_2: [u8; 2],
    pub hide_bones: [i32; 5],
}
assert_size!(DestructiblePieceRaw, 312);

pub struct DestructiblePiece {
    pub stages: [DestructibleStage; 5],
    pub parent_piece: u8,
    pub parent_damage_percent: f32,
    pub bullet_damage_scale: f32,
    pub explosive_damage_scale: f32,
    pub melee_damage_scale: f32,
    pub impact_damage_scael: f32,
    pub entity_damage_transfer: f32,
    pub phys_constraints: Option<Box<xmodel::PhysConstraints>>,
    pub health: i32,
    pub damage_sound: String,
    pub burn_effect: Option<Box<fx::FxEffectDef>>,
    pub burn_sound: String,
    pub enable_label: u16,
    pub hide_bones: [i32; 5],
}

impl<'a> XFileInto<DestructiblePiece> for DestructiblePieceRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> DestructiblePiece {
        DestructiblePiece { 
            stages: [self.stages[0].xfile_into(&mut xfile), self.stages[1].xfile_into(&mut xfile), self.stages[2].xfile_into(&mut xfile), self.stages[3].xfile_into(&mut xfile), self.stages[4].xfile_into(&mut xfile) ], 
            parent_piece: self.parent_piece, 
            parent_damage_percent: self.parent_damage_percent, 
            bullet_damage_scale: self.bullet_damage_scale, 
            explosive_damage_scale: self.explosive_damage_scale,
            melee_damage_scale: self.melee_damage_scale, 
            impact_damage_scael: self.impact_damage_scael, 
            entity_damage_transfer: self.entity_damage_transfer, 
            phys_constraints: self.phys_constraints.xfile_into(&mut xfile), 
            health: self.health, 
            damage_sound: self.damage_sound.xfile_into(&mut xfile), 
            burn_effect: self.burn_effect.xfile_into(&mut xfile), 
            burn_sound: self.burn_sound.xfile_into(&mut xfile), 
            enable_label: self.enable_label, 
            hide_bones: self.hide_bones 
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DestructibleStageRaw<'a> {
    pub show_bone: ScriptString,
    pub break_health: f32,
    pub max_time: f32,
    pub flags: u32,
    pub break_effect: Ptr32<'a, fx::FxEffectDefRaw<'a>>,
    pub break_sound: XString<'a>,
    pub break_notify: XString<'a>,
    pub loop_sound: XString<'a>,
    pub spawn_model: [Ptr32<'a, xmodel::XModelRaw<'a>>; 3],
    pub phys_preset: Ptr32<'a, xmodel::PhysPresetRaw<'a>>

}
assert_size!(DestructibleStageRaw, 48);

pub struct DestructibleStage {
    pub show_bone: String,
    pub break_health: f32,
    pub max_time: f32,
    pub flags: u32,
    pub break_effect: Option<Box<fx::FxEffectDef>>,
    pub break_sound: String,
    pub break_notify: String,
    pub loop_sound: String,
    pub spawn_model: [Option<Box<xmodel::XModel>>; 3],
    pub phys_preset: Option<Box<xmodel::PhysPreset>>
}

impl<'a> XFileInto<DestructibleStage> for DestructibleStageRaw<'a> {
    fn xfile_into(&self, mut xfile: impl Read + Seek) -> DestructibleStage {
        DestructibleStage { 
            show_bone: self.show_bone.to_string(), 
            break_health: self.break_health, 
            max_time: self.max_time, 
            flags: self.flags, 
            break_effect: self.break_effect.xfile_into(&mut xfile), 
            break_sound: self.break_sound.xfile_into(&mut xfile),
            break_notify: self.break_notify.xfile_into(&mut xfile),
            loop_sound: self.loop_sound.xfile_into(&mut xfile),
            spawn_model: [self.spawn_model[0].xfile_into(&mut xfile), self.spawn_model[1].xfile_into(&mut xfile), self.spawn_model[2].xfile_into(&mut xfile)],
            phys_preset: self.phys_preset.xfile_into(xfile)
        }
    }
}