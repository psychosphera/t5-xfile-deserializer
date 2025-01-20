use alloc::{boxed::Box, vec::Vec};

use crate::{
    FatPointer, FatPointerCountFirstU32, Ptr32, Result, ScriptStringRaw, T5XFileDeserialize,
    T5XFileSerialize, XFileDeserializeInto, XFileSerialize, XString, XStringRaw, assert_size,
    fx::{FxEffectDef, FxEffectDefRaw},
    xmodel::{PhysConstraints, PhysConstraintsRaw, PhysPreset, PhysPresetRaw, XModel, XModelRaw},
};

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct DestructibleDefRaw<'a> {
    pub name: XStringRaw<'a>,
    pub model: Ptr32<'a, XModelRaw<'a>>,
    pub pristine_model: Ptr32<'a, XModelRaw<'a>>,
    pub pieces: FatPointerCountFirstU32<'a, DestructiblePieceRaw<'a>>,
    pub client_only: i32,
}
assert_size!(DestructibleDefRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DestructibleDef {
    pub name: XString,
    pub model: Option<Box<XModel>>,
    pub pristine_model: Option<Box<XModel>>,
    pub pieces: Vec<DestructiblePiece>,
    pub client_only: bool,
}

impl<'a> XFileDeserializeInto<DestructibleDef, ()> for DestructibleDefRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DestructibleDef> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let model = self.model.xfile_deserialize_into(de, ())?;
        let pristine_model = self.pristine_model.xfile_deserialize_into(de, ())?;
        let pieces = self.pieces.xfile_deserialize_into(de, ())?;

        Ok(DestructibleDef {
            name,
            model,
            pristine_model,
            pieces,
            client_only: self.client_only != 0,
        })
    }
}

impl XFileSerialize<()> for DestructibleDef {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let name = XStringRaw::from_str(self.name.get());
        let model = Ptr32::from_box(&self.model);
        let pristine_model = Ptr32::from_box(&self.pristine_model);
        let pieces = FatPointerCountFirstU32::from_slice(&self.pieces);

        let destructible_def = DestructibleDefRaw {
            name,
            model,
            pristine_model,
            pieces,
            client_only: self.client_only as _,
        };

        ser.store_into_xfile(destructible_def)?;
        self.name.xfile_serialize(ser, ())?;
        self.model.xfile_serialize(ser, ())?;
        self.pristine_model.xfile_serialize(ser, ())?;
        self.pieces.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct DestructiblePieceRaw<'a> {
    pub stages: [DestructibleStageRaw<'a>; 5],
    pub parent_piece: u8,
    #[allow(dead_code)]
    unused: [u8; 3],
    pub parent_damage_percent: f32,
    pub bullet_damage_scale: f32,
    pub explosive_damage_scale: f32,
    pub melee_damage_scale: f32,
    pub impact_damage_scael: f32,
    pub entity_damage_transfer: f32,
    pub phys_constraints: Ptr32<'a, PhysConstraintsRaw<'a>>,
    pub health: i32,
    pub damage_sound: XStringRaw<'a>,
    pub burn_effect: Ptr32<'a, FxEffectDefRaw<'a>>,
    pub burn_sound: XStringRaw<'a>,
    pub enable_label: u16,
    #[allow(dead_code)]
    unused_2: [u8; 2],
    pub hide_bones: [i32; 5],
}
assert_size!(DestructiblePieceRaw, 312);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DestructiblePiece {
    pub stages: [DestructibleStage; 5],
    pub parent_piece: u8,
    pub parent_damage_percent: f32,
    pub bullet_damage_scale: f32,
    pub explosive_damage_scale: f32,
    pub melee_damage_scale: f32,
    pub impact_damage_scael: f32,
    pub entity_damage_transfer: f32,
    pub phys_constraints: Option<Box<PhysConstraints>>,
    pub health: i32,
    pub damage_sound: XString,
    pub burn_effect: Option<Box<FxEffectDef>>,
    pub burn_sound: XString,
    pub enable_label: u16,
    pub hide_bones: [i32; 5],
}

impl<'a> XFileDeserializeInto<DestructiblePiece, ()> for DestructiblePieceRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DestructiblePiece> {
        let stages = [
            self.stages[0].xfile_deserialize_into(de, ())?,
            self.stages[1].xfile_deserialize_into(de, ())?,
            self.stages[2].xfile_deserialize_into(de, ())?,
            self.stages[3].xfile_deserialize_into(de, ())?,
            self.stages[4].xfile_deserialize_into(de, ())?,
        ];
        let phys_constraints = self.phys_constraints.xfile_deserialize_into(de, ())?;
        let damage_sound = self.damage_sound.xfile_deserialize_into(de, ())?;
        let burn_effect = self.burn_effect.xfile_deserialize_into(de, ())?;
        let burn_sound = self.burn_sound.xfile_deserialize_into(de, ())?;

        Ok(DestructiblePiece {
            stages,
            parent_piece: self.parent_piece,
            parent_damage_percent: self.parent_damage_percent,
            bullet_damage_scale: self.bullet_damage_scale,
            explosive_damage_scale: self.explosive_damage_scale,
            melee_damage_scale: self.melee_damage_scale,
            impact_damage_scael: self.impact_damage_scael,
            entity_damage_transfer: self.entity_damage_transfer,
            phys_constraints,
            health: self.health,
            damage_sound,
            burn_effect,
            burn_sound,
            enable_label: self.enable_label,
            hide_bones: self.hide_bones,
        })
    }
}

impl XFileSerialize<()> for DestructiblePiece {
    fn xfile_serialize(&self, ser: &mut impl T5XFileSerialize, _data: ()) -> Result<()> {
        let stages = self
            .stages
            .iter()
            .map(|s| {
                let show_bone = ser.get_or_insert_script_string(s.show_bone.get())?;
                let break_effect = Ptr32::from_box(&s.break_effect);
                let break_sound = XStringRaw::from_str(s.break_sound.get());
                let break_notify = XStringRaw::from_str(s.break_notify.get());
                let loop_sound = XStringRaw::from_str(s.loop_sound.get());
                let spawn_model = s
                    .spawn_model
                    .iter()
                    .into_iter()
                    .map(|m| Ptr32::from_box(m))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let phys_preset = Ptr32::from_box(&s.phys_preset);

                Ok(DestructibleStageRaw {
                    show_bone,
                    break_effect,
                    break_health: s.break_health,
                    max_time: s.max_time,
                    flags: s.flags,
                    break_sound,
                    break_notify,
                    loop_sound,
                    spawn_model,
                    phys_preset,
                })
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .unwrap();
        let phys_constraints = Ptr32::from_box(&self.phys_constraints);
        let damage_sound = XStringRaw::from_str(self.damage_sound.get());
        let burn_effect = Ptr32::from_box(&self.burn_effect);
        let burn_sound = XStringRaw::from_str(self.burn_sound.get());

        let destructible_piece = DestructiblePieceRaw {
            stages,
            parent_piece: self.parent_piece,
            unused: [0u8; 3],
            parent_damage_percent: self.parent_damage_percent,
            bullet_damage_scale: self.bullet_damage_scale,
            explosive_damage_scale: self.explosive_damage_scale,
            melee_damage_scale: self.melee_damage_scale,
            impact_damage_scael: self.impact_damage_scael,
            entity_damage_transfer: self.entity_damage_transfer,
            phys_constraints,
            health: self.health,
            damage_sound,
            burn_effect,
            burn_sound,
            enable_label: self.enable_label,
            unused_2: [0u8; 2],
            hide_bones: self.hide_bones,
        };

        ser.store_into_xfile(destructible_piece)?;
        self.phys_constraints.xfile_serialize(ser, ())?;
        self.damage_sound.xfile_serialize(ser, ())?;
        self.burn_effect.xfile_serialize(ser, ())?;
        self.burn_sound.xfile_serialize(ser, ())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct DestructibleStageRaw<'a> {
    pub show_bone: ScriptStringRaw,
    pub break_health: f32,
    pub max_time: f32,
    pub flags: u32,
    pub break_effect: Ptr32<'a, FxEffectDefRaw<'a>>,
    pub break_sound: XStringRaw<'a>,
    pub break_notify: XStringRaw<'a>,
    pub loop_sound: XStringRaw<'a>,
    pub spawn_model: [Ptr32<'a, XModelRaw<'a>>; 3],
    pub phys_preset: Ptr32<'a, PhysPresetRaw<'a>>,
}
assert_size!(DestructibleStageRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DestructibleStage {
    pub show_bone: XString,
    pub break_health: f32,
    pub max_time: f32,
    pub flags: u32,
    pub break_effect: Option<Box<FxEffectDef>>,
    pub break_sound: XString,
    pub break_notify: XString,
    pub loop_sound: XString,
    pub spawn_model: [Option<Box<XModel>>; 3],
    pub phys_preset: Option<Box<PhysPreset>>,
}

impl<'a> XFileDeserializeInto<DestructibleStage, ()> for DestructibleStageRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<DestructibleStage> {
        Ok(DestructibleStage {
            show_bone: XString(self.show_bone.to_string(de).unwrap_or_default()),
            break_health: self.break_health,
            max_time: self.max_time,
            flags: self.flags,
            break_effect: self.break_effect.xfile_deserialize_into(de, ())?,
            break_sound: self.break_sound.xfile_deserialize_into(de, ())?,
            break_notify: self.break_notify.xfile_deserialize_into(de, ())?,
            loop_sound: self.loop_sound.xfile_deserialize_into(de, ())?,
            spawn_model: [
                self.spawn_model[0].xfile_deserialize_into(de, ())?,
                self.spawn_model[1].xfile_deserialize_into(de, ())?,
                self.spawn_model[2].xfile_deserialize_into(de, ())?,
            ],
            phys_preset: self.phys_preset.xfile_deserialize_into(de, ())?,
        })
    }
}
