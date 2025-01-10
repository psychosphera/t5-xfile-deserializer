use core::fmt::Display;
#[cfg(feature = "d3d9")]
use core::ptr::addr_of_mut;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    FatPointer, FatPointerCountFirstU32, FatPointerCountLastU8, FatPointerCountLastU32, Ptr32,
    Result, T5XFileDeserialize, XFileDeserializeInto, XString, assert_size,
    common::{GfxVertexBuffer, Mat3, Mat4, Vec2, Vec3, Vec4},
    light::{GfxLightDef, GfxLightDefRaw},
    techset::{
        GfxDrawSurf, GfxImage, GfxImageRaw, GfxTexture, GfxTextureRaw, Material, MaterialRaw,
    },
    xmodel::{CPlane, CPlaneRaw, GfxColor, XModel, XModelDrawInfo, XModelRaw},
};

use serde::{Deserialize, Serialize};

#[cfg(feature = "d3d9")]
use windows::Win32::Graphics::Direct3D9::{D3DPOOL_DEFAULT, IDirect3DVertexBuffer9};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldRaw<'a, const MAX_LOCAL_CLIENTS: usize> {
    pub name: XString<'a>,
    pub base_name: XString<'a>,
    pub plane_count: i32,
    pub node_count: i32,
    pub surface_count: i32,
    pub stream_info: GfxWorldStreamInfoRaw<'a>,
    pub sky_start_surfs: FatPointerCountFirstU32<'a, i32>,
    pub sky_image: Ptr32<'a, GfxImageRaw<'a>>,
    pub sky_sampler_state: u8,
    #[allow(dead_code)]
    pad: [u8; 3],
    pub sky_box_model: XString<'a>,
    pub sun_parse: SunLightParseParamsRaw<MAX_LOCAL_CLIENTS>,
    pub sun_light: Ptr32<'a, GfxLightRaw<'a>>,
    pub sun_color_from_bsp: [f32; 3],
    pub sun_primary_light_index: u32,
    pub primary_light_count: u32,
    pub cull_group_count: i32,
    pub coronas: FatPointerCountFirstU32<'a, GfxLightCoronaRaw>,
    pub shadow_map_volumes: FatPointerCountFirstU32<'a, GfxShadowMapVolumeRaw>,
    pub shadow_map_volume_planes: FatPointerCountFirstU32<'a, GfxVolumePlaneRaw>,
    pub exposure_volumes: FatPointerCountFirstU32<'a, GfxExposureVolume>,
    pub exposure_volume_planes: FatPointerCountFirstU32<'a, GfxVolumePlaneRaw>,
    pub sky_dyn_intensity: GfxSkyDynamicIntensity,
    pub dpvs_planes: GfxWorldDpvsPlanesRaw<'a>,
    pub cell_bits_count: i32,
    pub cells: Ptr32<'a, GfxCellRaw<'a>>,
    pub draw: GfxWorldDrawRaw<'a>,
    pub light_grid: GfxLightGridRaw<'a>,
    pub models: FatPointerCountFirstU32<'a, GfxBrushModelRaw>,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub checksum: u32,
    pub material_memory: FatPointerCountFirstU32<'a, MaterialMemoryRaw<'a>>,
    pub sun: SunflareRaw<'a>,
    pub outdoor_lookup_matrix: [[f32; 4]; 4],
    pub outdoor_image: Ptr32<'a, GfxImageRaw<'a>>,
    pub cell_caster_bits: Ptr32<'a, u32>,
    pub scene_dyn_model: Ptr32<'a, GfxSceneDynModel>,
    pub scene_dyn_brush: Ptr32<'a, GfxSceneDynBrush>,
    pub primary_light_entity_shadow_vis: Ptr32<'a, u32>,
    pub primary_light_dyn_ent_shadow_vis: [Ptr32<'a, u32>; 2],
    pub non_sun_primary_light_for_model_dyn_ent: Ptr32<'a, u8>,
    pub shadow_geom: Ptr32<'a, GfxShadowGeometryRaw<'a>>,
    pub light_region: Ptr32<'a, GfxLightRegionRaw<'a>>,
    pub dpvs: GfxWorldDpvsStaticRaw<'a>,
    pub dpvs_dyn: GfxWorldDpvsDynamicRaw<'a>,
    pub world_lod_chains: FatPointerCountFirstU32<'a, GfxWorldLodChainRaw>,
    pub world_lod_infos: FatPointerCountFirstU32<'a, GfxWorldLodInfo>,
    pub world_lod_surfaces: FatPointerCountFirstU32<'a, u32>,
    pub water_direction: f32,
    pub water_buffers: [GfxWaterBufferRaw<'a>; 2],
    pub water_material: Ptr32<'a, MaterialRaw<'a>>,
    pub corona_material: Ptr32<'a, MaterialRaw<'a>>,
    pub rope_material: Ptr32<'a, MaterialRaw<'a>>,
    pub occluders: FatPointerCountFirstU32<'a, OccluderRaw>,
    pub outdoor_bounds: FatPointerCountFirstU32<'a, GfxOutdoorBoundsRaw>,
    pub hero_light_count: u32,
    pub hero_light_tree_count: u32,
    pub hero_lights: Ptr32<'a, GfxHeroLightRaw>,
    pub hero_light_tree: Ptr32<'a, GfxHeroLightTreeRaw>,
}
assert_size!(GfxWorldRaw<1>, 1084);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorld<const MAX_LOCAL_CLIENTS: usize> {
    pub name: String,
    pub base_name: String,
    pub plane_count: i32,
    pub node_count: i32,
    pub surface_count: i32,
    pub stream_info: GfxWorldStreamInfo,
    pub sky_start_surfs: Vec<i32>,
    pub sky_image: Option<Box<GfxImage>>,
    pub sky_sampler_state: u8,
    pub sky_box_model: String,
    pub sun_parse: SunLightParseParams<MAX_LOCAL_CLIENTS>,
    pub sun_light: Option<Box<GfxLight>>,
    pub sun_color_from_bsp: Vec3,
    pub sun_primary_light_index: usize,
    pub primary_light_count: u32,
    pub cull_group_count: i32,
    pub coronas: Vec<GfxLightCorona>,
    pub shadow_map_volumes: Vec<GfxShadowMapVolume>,
    pub shadow_map_volume_planes: Vec<GfxVolumePlane>,
    pub exposure_volumes: Vec<GfxExposureVolume>,
    pub exposure_volume_planes: Vec<GfxVolumePlane>,
    pub sky_dyn_intensity: GfxSkyDynamicIntensity,
    pub dpvs_planes: GfxWorldDpvsPlanes,
    pub cell_bits_count: i32,
    pub cells: Vec<GfxCell>,
    pub draw: GfxWorldDraw,
    pub light_grid: GfxLightGrid,
    pub models: Vec<GfxBrushModel>,
    pub mins: Vec3,
    pub maxs: Vec3,
    pub checksum: u32,
    pub material_memory: Vec<MaterialMemory>,
    pub sun: Sunflare,
    pub outdoor_lookup_matrix: Mat4,
    pub outdoor_image: Option<Box<GfxImage>>,
    pub cell_caster_bits: Vec<u32>,
    pub scene_dyn_model: Vec<GfxSceneDynModel>,
    pub scene_dyn_brush: Vec<GfxSceneDynBrush>,
    pub primary_light_entity_shadow_vis: Vec<u32>,
    pub primary_light_dyn_ent_shadow_vis: [Vec<u32>; 2],
    pub non_sun_primary_light_for_model_dyn_ent: Vec<u8>,
    pub shadow_geom: Vec<GfxShadowGeometry>,
    pub light_region: Vec<GfxLightRegion>,
    pub dpvs: GfxWorldDpvsStatic,
    pub dpvs_dyn: GfxWorldDpvsDynamic,
    pub world_lod_chains: Vec<GfxWorldLodChain>,
    pub world_lod_infos: Vec<GfxWorldLodInfo>,
    pub world_lod_surfaces: Vec<u32>,
    pub water_direction: f32,
    pub water_buffers: [GfxWaterBuffer; 2],
    pub water_material: Option<Box<Material>>,
    pub corona_material: Option<Box<Material>>,
    pub rope_material: Option<Box<Material>>,
    pub occluders: Vec<Occluder>,
    pub outdoor_bounds: Vec<GfxOutdoorBounds>,
    pub hero_lights: Vec<GfxHeroLight>,
    pub hero_light_tree: Vec<GfxHeroLightTree>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileDeserializeInto<GfxWorld<MAX_LOCAL_CLIENTS>, ()>
    for GfxWorldRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxWorld<MAX_LOCAL_CLIENTS>> {
        let name = self.name.xfile_deserialize_into(de, ())?;
        let base_name = self.base_name.xfile_deserialize_into(de, ())?;
        let stream_info = self.stream_info.xfile_deserialize_into(de, ())?;
        let sky_start_surfs = self.sky_start_surfs.to_vec(de)?;
        let sky_image = self.sky_image.xfile_deserialize_into(de, ())?;
        let sky_box_model = self.sky_box_model.xfile_deserialize_into(de, ())?;
        let sun_parse = self.sun_parse.into();
        let sun_light = self.sun_light.xfile_deserialize_into(de, ())?;
        let sun_color_from_bsp = self.sun_color_from_bsp.into();
        let coronas = self.coronas.to_vec_into(de)?;
        let shadow_map_volumes = self.shadow_map_volumes.to_vec_into(de)?;
        let shadow_map_volume_planes = self.shadow_map_volume_planes.to_vec_into(de)?;
        let exposure_volumes = self.exposure_volumes.to_vec(de)?;
        let exposure_volume_planes = self.exposure_volume_planes.to_vec_into(de)?;
        let dpvs_planes = self
            .dpvs_planes
            .xfile_deserialize_into(de, (self.node_count, self.plane_count))?;
        let cells = self
            .cells
            .to_array(self.dpvs_planes.cell_count as _)
            .xfile_deserialize_into(de, ())?;
        let draw = self.draw.xfile_deserialize_into(de, ())?;
        let light_grid = self.light_grid.xfile_deserialize_into(de, ())?;
        let models = self.models.to_vec_into(de)?;
        let mins = self.mins.into();
        let maxs = self.maxs.into();
        let material_memory = self.material_memory.xfile_deserialize_into(de, ())?;
        let sun = self.sun.xfile_deserialize_into(de, ())?;
        let outdoor_lookup_matrix = self.outdoor_lookup_matrix.into();
        let outdoor_image = self.outdoor_image.xfile_deserialize_into(de, ())?;
        let cell_caster_bits = self
            .cell_caster_bits
            .to_array(
                ((self.dpvs_planes.cell_count as usize + 31) >> 5)
                    * self.dpvs_planes.cell_count as usize,
            )
            .to_vec(de)?;
        let scene_dyn_model = self
            .scene_dyn_model
            .to_array(self.dpvs_dyn.dyn_ent_client_count[0] as _)
            .to_vec(de)?;
        let scene_dyn_brush = self
            .scene_dyn_brush
            .to_array(self.dpvs_dyn.dyn_ent_client_count[1] as _)
            .to_vec(de)?;
        let primary_light_entity_shadow_vis = self
            .primary_light_entity_shadow_vis
            .to_array(
                (self.primary_light_count as usize - self.sun_primary_light_index as usize + 1)
                    * 8192,
            )
            .to_vec(de)?;
        let primary_light_dyn_ent_shadow_vis = [
            self.primary_light_dyn_ent_shadow_vis[0]
                .to_array(
                    (self.primary_light_count as usize - self.sun_primary_light_index as usize + 1)
                        * self.dpvs_dyn.dyn_ent_client_count[0] as usize,
                )
                .to_vec(de)?,
            self.primary_light_dyn_ent_shadow_vis[1]
                .to_array(
                    (self.primary_light_count as usize - self.sun_primary_light_index as usize + 1)
                        * self.dpvs_dyn.dyn_ent_client_count[1] as usize,
                )
                .to_vec(de)?,
        ];
        let non_sun_primary_light_for_model_dyn_ent = self
            .non_sun_primary_light_for_model_dyn_ent
            .to_array(self.dpvs_dyn.dyn_ent_client_count[0] as _)
            .to_vec(de)?;
        let shadow_geom = self
            .shadow_geom
            .to_array(self.primary_light_count as _)
            .xfile_deserialize_into(de, ())?;
        let light_region = self
            .light_region
            .to_array(self.primary_light_count as _)
            .xfile_deserialize_into(de, ())?;
        let dpvs = self
            .dpvs
            .xfile_deserialize_into(de, (self.surface_count, self.cull_group_count))?;
        let dpvs_dyn = self
            .dpvs_dyn
            .xfile_deserialize_into(de, self.dpvs_planes.cell_count as _)?;
        let world_lod_chains = self.world_lod_chains.to_vec_into(de)?;
        let world_lod_infos = self.world_lod_infos.to_vec(de)?;
        let world_lod_surfaces = self.world_lod_surfaces.to_vec(de)?;
        let water_buffers = [
            self.water_buffers[0].xfile_deserialize_into(de, ())?,
            self.water_buffers[0].xfile_deserialize_into(de, ())?,
        ];
        let water_material = self.water_material.xfile_deserialize_into(de, ())?;
        let corona_material = self.corona_material.xfile_deserialize_into(de, ())?;
        let rope_material = self.rope_material.xfile_deserialize_into(de, ())?;
        let occluders = self.occluders.to_vec_into(de)?;
        let outdoor_bounds = self.outdoor_bounds.to_vec_into(de)?;
        let hero_lights = self
            .hero_lights
            .to_array(self.hero_light_count as _)
            .to_vec_into(de)?;
        let hero_light_tree = self
            .hero_light_tree
            .to_array(self.hero_light_tree_count as _)
            .to_vec_into(de)?;

        Ok(GfxWorld {
            name,
            base_name,
            plane_count: self.plane_count,
            node_count: self.node_count,
            surface_count: self.surface_count,
            stream_info,
            sky_start_surfs,
            sky_image,
            sky_sampler_state: self.sky_sampler_state,
            sky_box_model,
            sun_parse,
            sun_light,
            sun_color_from_bsp,
            sun_primary_light_index: self.sun_primary_light_index as _,
            primary_light_count: self.primary_light_count,
            cull_group_count: self.cull_group_count,
            coronas,
            shadow_map_volumes,
            shadow_map_volume_planes,
            exposure_volumes,
            exposure_volume_planes,
            sky_dyn_intensity: self.sky_dyn_intensity,
            dpvs_planes,
            cell_bits_count: self.cell_bits_count,
            cells,
            draw,
            light_grid,
            models,
            mins,
            maxs,
            checksum: self.checksum,
            material_memory,
            sun,
            outdoor_lookup_matrix,
            outdoor_image,
            cell_caster_bits,
            scene_dyn_model,
            scene_dyn_brush,
            primary_light_entity_shadow_vis,
            primary_light_dyn_ent_shadow_vis,
            non_sun_primary_light_for_model_dyn_ent,
            shadow_geom,
            light_region,
            dpvs,
            dpvs_dyn,
            world_lod_chains,
            world_lod_infos,
            world_lod_surfaces,
            water_direction: self.water_direction,
            water_buffers,
            water_material,
            corona_material,
            rope_material,
            occluders,
            outdoor_bounds,
            hero_lights,
            hero_light_tree,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldStreamInfoRaw<'a> {
    pub aabb_trees: FatPointerCountFirstU32<'a, GfxStreamingAabbTreeRaw>,
    pub leaf_refs: FatPointerCountFirstU32<'a, i32>,
}
assert_size!(GfxWorldStreamInfoRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldStreamInfo {
    pub aabb_trees: Vec<GfxStreamingAabbTree>,
    pub leaf_refs: Vec<i32>,
}

impl<'a> XFileDeserializeInto<GfxWorldStreamInfo, ()> for GfxWorldStreamInfoRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxWorldStreamInfo> {
        let aabb_trees = self.aabb_trees.to_vec_into(de)?;
        let leaf_refs = self.leaf_refs.to_vec(de)?;

        Ok(GfxWorldStreamInfo {
            aabb_trees,
            leaf_refs,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxStreamingAabbTreeRaw {
    pub first_item: u16,
    pub item_count: u16,
    pub first_child: u16,
    pub child_count: u16,
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
}
assert_size!(GfxStreamingAabbTreeRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxStreamingAabbTree {
    pub first_item: u16,
    pub item_count: u16,
    pub first_child: u16,
    pub child_count: u16,
    pub mins: Vec3,
    pub maxs: Vec3,
}

impl From<GfxStreamingAabbTreeRaw> for GfxStreamingAabbTree {
    fn from(value: GfxStreamingAabbTreeRaw) -> Self {
        Self {
            first_item: value.first_item,
            item_count: value.item_count,
            first_child: value.first_child,
            child_count: value.child_count,
            mins: value.mins.into(),
            maxs: value.maxs.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxName64(#[serde(with = "serde_arrays")] [u8; 64]);

impl Display for GfxName64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let len = self
            .0
            .into_iter()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
            .unwrap_or(64);
        let s = self.0[..len].iter().map(|c| *c as char).collect::<String>();
        write!(f, "{}", s)
    }
}

impl Default for GfxName64 {
    fn default() -> Self {
        Self([0u8; 64])
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub(crate) struct GfxName16([u8; 16]);

impl Display for GfxName16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let len = self
            .0
            .into_iter()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
            .unwrap_or(64);
        let s = self.0[..len].iter().map(|c| *c as char).collect::<String>();
        write!(f, "{}", s)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SunLightParseParamsRaw<const MAX_LOCAL_CLIENTS: usize> {
    pub name: GfxName64,
    pub tree_scatter_intensity: f32,
    pub tree_scatter_amount: f32,
    #[serde(with = "serde_arrays")]
    pub sun_settings: [GfxWorldSunColorRaw; MAX_LOCAL_CLIENTS],
}
assert_size!(SunLightParseParamsRaw<1>, 180);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SunLightParseParams<const MAX_LOCAL_CLIENTS: usize> {
    pub name: String,
    pub tree_scatter_intensity: f32,
    pub tree_scatter_amount: f32,
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub sun_settings: [GfxWorldSunColor; MAX_LOCAL_CLIENTS],
}

impl<const MAX_LOCAL_CLIENTS: usize> From<SunLightParseParamsRaw<MAX_LOCAL_CLIENTS>>
    for SunLightParseParams<MAX_LOCAL_CLIENTS>
{
    fn from(value: SunLightParseParamsRaw<MAX_LOCAL_CLIENTS>) -> Self {
        Self {
            name: value.name.to_string(),
            tree_scatter_intensity: value.tree_scatter_intensity,
            tree_scatter_amount: value.tree_scatter_amount,
            sun_settings: value
                .sun_settings
                .into_iter()
                .map(Into::<GfxWorldSunColor>::into)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldSunColorRaw {
    pub control: u32,
    pub angles: [f32; 3],
    pub ambient_color: [f32; 4],
    pub sun_diffuse_color: [f32; 4],
    pub sun_specular_color: [f32; 4],
    pub sky_color: [f32; 4],
    pub ground_color: [f32; 4],
    pub exposure: f32,
    pub sun_shadow_sample_size_near: f32,
    pub skybox_hdr_scale: f32,
}
assert_size!(GfxWorldSunColorRaw, 108);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldSunColor {
    pub control: u32,
    pub angles: Vec3,
    pub ambient_color: Vec4,
    pub sun_diffuse_color: Vec4,
    pub sun_specular_color: Vec4,
    pub sky_color: Vec4,
    pub ground_color: Vec4,
    pub exposure: f32,
    pub sun_shadow_sample_size_near: f32,
    pub skybox_hdr_scale: f32,
}

impl From<GfxWorldSunColorRaw> for GfxWorldSunColor {
    fn from(value: GfxWorldSunColorRaw) -> Self {
        Self {
            control: value.control,
            angles: value.angles.into(),
            ambient_color: value.ambient_color.into(),
            sun_diffuse_color: value.sun_diffuse_color.into(),
            sun_specular_color: value.sun_specular_color.into(),
            sky_color: value.sky_color.into(),
            ground_color: value.ground_color.into(),
            exposure: value.exposure,
            sun_shadow_sample_size_near: value.sun_shadow_sample_size_near,
            skybox_hdr_scale: value.skybox_hdr_scale,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightRaw<'a> {
    pub type_: u8,
    pub can_use_shadow_map: u8,
    pub cull_dist: i16,
    pub color: [f32; 3],
    pub dir: [f32; 3],
    pub origin: [f32; 3],
    pub radius: f32,
    pub cos_half_fov_outer: f32,
    pub cos_half_fov_inner: f32,
    pub exponent: i32,
    pub spot_shadows_index: u32,
    pub angles: [f32; 3],
    pub spot_shadow_hi_distance: f32,
    pub diffuse_color: [f32; 4],
    pub specular_color: [f32; 4],
    pub shadow_color: [f32; 4],
    pub falloff: [f32; 4],
    pub attenuation: [f32; 4],
    pub aabb: [f32; 4],
    pub cookie_control_0: [f32; 4],
    pub cookie_control_1: [f32; 4],
    pub cookie_control_2: [f32; 4],
    #[allow(dead_code)]
    pad: [u8; 4],
    pub view_matrix: [[f32; 4]; 4],
    pub proj_matrix: [[f32; 4]; 4],
    pub def: Ptr32<'a, GfxLightDefRaw<'a>>,
    #[allow(dead_code)]
    pad2: [u8; 12],
}
assert_size!(GfxLightRaw, 368);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLight {
    pub type_: u8,
    pub can_use_shadow_map: u8,
    pub cull_dist: i16,
    pub color: Vec3,
    pub dir: Vec3,
    pub origin: Vec3,
    pub radius: f32,
    pub cos_half_fov_outer: f32,
    pub cos_half_fov_inner: f32,
    pub exponent: i32,
    pub spot_shadows_index: usize,
    pub angles: Vec3,
    pub spot_shadow_hi_distance: f32,
    pub diffuse_color: Vec4,
    pub specular_color: Vec4,
    pub shadow_color: Vec4,
    pub falloff: Vec4,
    pub attenuation: Vec4,
    pub aabb: Vec4,
    pub cookie_control_0: Vec4,
    pub cookie_control_1: Vec4,
    pub cookie_control_2: Vec4,
    pub view_matrix: Mat4,
    pub proj_matrix: Mat4,
    pub def: Option<Box<GfxLightDef>>,
}

impl<'a> XFileDeserializeInto<GfxLight, ()> for GfxLightRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLight> {
        let color = self.color.into();
        let dir = self.dir.into();
        let origin = self.origin.into();
        let angles = self.angles.into();
        let diffuse_color = self.diffuse_color.into();
        let specular_color = self.specular_color.into();
        let shadow_color = self.shadow_color.into();
        let falloff = self.falloff.into();
        let attenuation = self.attenuation.into();
        let aabb = self.aabb.into();
        let cookie_control_0 = self.cookie_control_0.into();
        let cookie_control_1 = self.cookie_control_1.into();
        let cookie_control_2 = self.cookie_control_2.into();
        let view_matrix = self.view_matrix.into();
        let proj_matrix = self.proj_matrix.into();
        let def = self.def.xfile_deserialize_into(de, ())?;

        Ok(GfxLight {
            type_: self.type_,
            can_use_shadow_map: self.can_use_shadow_map,
            cull_dist: self.cull_dist,
            color,
            dir,
            origin,
            radius: self.radius,
            cos_half_fov_outer: self.cos_half_fov_outer,
            cos_half_fov_inner: self.cos_half_fov_inner,
            exponent: self.exponent,
            spot_shadows_index: self.spot_shadows_index as _,
            angles,
            spot_shadow_hi_distance: self.spot_shadow_hi_distance,
            diffuse_color,
            specular_color,
            shadow_color,
            falloff,
            attenuation,
            aabb,
            cookie_control_0,
            cookie_control_1,
            cookie_control_2,
            view_matrix,
            proj_matrix,
            def,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightCoronaRaw {
    pub origin: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub intensity: f32,
}
assert_size!(GfxLightCoronaRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightCorona {
    pub origin: Vec3,
    pub radius: f32,
    pub color: Vec3,
    pub intensity: f32,
}

impl From<GfxLightCoronaRaw> for GfxLightCorona {
    fn from(value: GfxLightCoronaRaw) -> Self {
        Self {
            origin: value.origin.into(),
            radius: value.radius,
            color: value.color.into(),
            intensity: value.intensity,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxShadowMapVolumeRaw {
    pub control: u32,
    #[allow(dead_code)]
    pad: [u8; 12],
}
assert_size!(GfxShadowMapVolumeRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxShadowMapVolume {
    pub control: u32,
}

impl From<GfxShadowMapVolumeRaw> for GfxShadowMapVolume {
    fn from(value: GfxShadowMapVolumeRaw) -> Self {
        Self {
            control: value.control,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxVolumePlaneRaw {
    pub plane: [f32; 4],
}
assert_size!(GfxVolumePlaneRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxVolumePlane {
    pub plane: Vec4,
}

impl From<GfxVolumePlaneRaw> for GfxVolumePlane {
    fn from(value: GfxVolumePlaneRaw) -> Self {
        Self {
            plane: value.plane.into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxExposureVolume {
    pub control: u32,
    pub exposure: f32,
    pub luminance_increase_scale: f32,
    pub luminance_decrease_scale: f32,
    pub feather_range: f32,
    pub feather_adjust: f32,
}
assert_size!(GfxExposureVolume, 24);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxSkyDynamicIntensity {
    pub angle_0: f32,
    pub angle_1: f32,
    pub factor_0: f32,
    pub factor_1: f32,
}
assert_size!(GfxSkyDynamicIntensity, 16);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldDpvsPlanesRaw<'a> {
    pub cell_count: i32,
    pub planes: Ptr32<'a, CPlaneRaw>,
    pub nodes: Ptr32<'a, u16>,
    pub scene_ent_cell_bits: Ptr32<'a, u32>,
}
assert_size!(GfxWorldDpvsPlanesRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldDpvsPlanes {
    pub planes: Vec<CPlane>,
    pub nodes: Vec<u16>,
    pub scene_ent_cell_bits: Vec<u32>,
}

impl<'a> XFileDeserializeInto<GfxWorldDpvsPlanes, (i32, i32)> for GfxWorldDpvsPlanesRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (plane_count, node_count): (i32, i32),
    ) -> Result<GfxWorldDpvsPlanes> {
        let planes = self.planes.to_array(plane_count as _).to_vec_into(de)?;
        let nodes = self.nodes.to_array(node_count as _).to_vec(de)?;
        let scene_ent_cell_bits = self
            .scene_ent_cell_bits
            .to_array(self.cell_count as usize * 512)
            .to_vec(de)?;

        Ok(GfxWorldDpvsPlanes {
            planes,
            nodes,
            scene_ent_cell_bits,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxCellRaw<'a> {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub aabb_tree: FatPointerCountFirstU32<'a, GfxAabbTreeRaw<'a>>,
    pub portals: FatPointerCountFirstU32<'a, GfxPortalRaw<'a>>,
    pub cull_groups: FatPointerCountFirstU32<'a, i32>,
    pub reflection_probes: FatPointerCountFirstU32<'a, u8>,
}
assert_size!(GfxCellRaw, 56);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxCell {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub aabb_tree: Vec<GfxAabbTree>,
    pub portals: Vec<GfxPortal>,
    pub cull_groups: Vec<i32>,
    pub reflection_probes: Vec<u8>,
}

impl<'a> XFileDeserializeInto<GfxCell, ()> for GfxCellRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxCell> {
        let mins = self.mins.into();
        let maxs = self.maxs.into();
        let aabb_tree = self.aabb_tree.xfile_deserialize_into(de, ())?;
        let portals = self.portals.xfile_deserialize_into(de, ())?;
        let cull_groups = self.cull_groups.to_vec(de)?;
        let reflection_probes = self.reflection_probes.to_vec(de)?;

        Ok(GfxCell {
            mins,
            maxs,
            aabb_tree,
            portals,
            cull_groups,
            reflection_probes,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxAabbTreeRaw<'a> {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub child_count: u16,
    pub surface_count: u16,
    pub start_surf_index: u16,
    pub smodel_index_count: u16,
    pub smodel_indexes: Ptr32<'a, u16>,
    pub children_offset: i32,
}
assert_size!(GfxAabbTreeRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxAabbTree {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub child_count: usize,
    pub surface_count: usize,
    pub start_surf_index: usize,
    pub smodel_indexes: Vec<u16>,
    pub children_offset: i32,
}

impl<'a> XFileDeserializeInto<GfxAabbTree, ()> for GfxAabbTreeRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxAabbTree> {
        let mins = self.mins.into();
        let maxs = self.maxs.into();
        let smodel_indexes = self
            .smodel_indexes
            .to_array(self.smodel_index_count as _)
            .to_vec(de)?;

        Ok(GfxAabbTree {
            mins,
            maxs,
            child_count: self.child_count as _,
            surface_count: self.surface_count as _,
            start_surf_index: self.start_surf_index as _,
            smodel_indexes,
            children_offset: self.children_offset,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxPortalRaw<'a> {
    #[allow(dead_code)]
    pub writable: GfxPortalWritableRaw<'a>,
    pub plane: DpvsPlaneRaw,
    pub cell: Ptr32<'a, GfxCellRaw<'a>>,
    pub vertices: FatPointerCountLastU8<'a, [f32; 3]>,
    pub hull_axis: [[f32; 3]; 2],
}
assert_size!(GfxPortalRaw, 68);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxPortal {
    pub plane: DpvsPlane,
    pub cell: Option<Box<GfxCell>>,
    pub vertices: Vec<Vec3>,
    pub hull_axis: [Vec3; 2],
}

impl<'a> XFileDeserializeInto<GfxPortal, ()> for GfxPortalRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxPortal> {
        let plane = self.plane.into();
        let cell = self.cell.xfile_deserialize_into(de, ())?;
        let vertices = self.vertices.to_vec_into(de)?;
        let hull_axis = [self.hull_axis[0].into(), self.hull_axis[1].into()];

        Ok(GfxPortal {
            plane,
            cell,
            vertices,
            hull_axis,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxPortalWritableRaw<'a> {
    pub is_queued: bool,
    pub is_ancestor: bool,
    pub recursion_depth: u8,
    pub hull_point_count: u8,
    pub hull_points: Ptr32<'a, [f32; 2]>,
    #[allow(dead_code)]
    pub queued_parent: Ptr32<'a, GfxPortalRaw<'a>>,
}
assert_size!(GfxPortalWritableRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxPortalWritable {
    pub is_queued: bool,
    pub is_ancestor: bool,
    pub recursion_depth: u8,
    pub hull_points: Vec<Vec2>,
    pub queued_parent: Option<Box<GfxPortal>>,
}

impl<'a> XFileDeserializeInto<GfxPortalWritable, ()> for GfxPortalWritableRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxPortalWritable> {
        let hull_points = self
            .hull_points
            .to_array(self.hull_point_count as _)
            .to_vec_into(de)?;
        let queued_parent = None; // FIXME

        Ok(GfxPortalWritable {
            is_queued: self.is_queued,
            is_ancestor: self.is_ancestor,
            recursion_depth: self.recursion_depth,
            hull_points,
            queued_parent,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DpvsPlaneRaw {
    pub coeffs: [f32; 4],
    pub side: [u8; 3],
    #[allow(dead_code)]
    pad: u8,
}
assert_size!(DpvsPlaneRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DpvsPlane {
    pub coeffs: Vec4,
    pub side: [u8; 3],
}

impl From<DpvsPlaneRaw> for DpvsPlane {
    fn from(value: DpvsPlaneRaw) -> Self {
        Self {
            coeffs: value.coeffs.into(),
            side: value.side,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldDrawRaw<'a> {
    pub reflection_probes: FatPointerCountFirstU32<'a, GfxReflectionProbeRaw<'a>>,
    pub reflection_probe_textures: Ptr32<'a, GfxTextureRaw<'a>>,
    pub lightmaps: FatPointerCountFirstU32<'a, GfxLightmapArrayRaw<'a>>,
    pub lightmap_primary_textures: Ptr32<'a, GfxTextureRaw<'a>>,
    pub lightmap_secondary_textures: Ptr32<'a, GfxTextureRaw<'a>>,
    pub lightmap_secondary_textures_b: Ptr32<'a, GfxTextureRaw<'a>>,
    pub terrain_scorch_images: [Ptr32<'a, GfxImageRaw<'a>>; 31],
    pub vertex_count: u32,
    pub vd: GfxWorldVertexDataRaw<'a>,
    pub vertex_layer_data_size: u32,
    pub vld: GfxWorldVertexLayerDataRaw<'a>,
    pub vertex_stream_2_data_size: u32,
    pub indices: FatPointerCountFirstU32<'a, u16>,
}
assert_size!(GfxWorldDrawRaw, 192);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldDraw {
    pub reflection_probes: Vec<GfxReflectionProbe>,
    pub reflection_probe_textures: Vec<GfxTexture>,
    pub lightmaps: Vec<GfxLightmapArray>,
    pub lightmap_primary_textures: Vec<GfxTexture>,
    pub lightmap_secondary_textures: Vec<GfxTexture>,
    pub lightmap_secondary_textures_b: Vec<GfxTexture>,
    pub terrain_scorch_images: [GfxImage; 31],
    pub vertex_count: u32,
    pub vd: GfxWorldVertexData,
    pub vertex_layer_data_size: u32,
    pub vld: GfxWorldVertexLayerData,
    pub vertex_stream_2_data_size: u32,
    pub indices: Vec<u16>,
}

impl<'a> XFileDeserializeInto<GfxWorldDraw, ()> for GfxWorldDrawRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxWorldDraw> {
        let reflection_probes = self.reflection_probes.xfile_deserialize_into(de, ())?;
        let reflection_probe_textures = self
            .reflection_probe_textures
            .to_array(self.reflection_probes.size())
            .xfile_deserialize_into(de, ())?;
        let lightmaps = self.lightmaps.xfile_deserialize_into(de, ())?;
        let lightmap_primary_textures = self
            .lightmap_primary_textures
            .to_array(self.lightmaps.size())
            .xfile_deserialize_into(de, ())?;
        let lightmap_secondary_textures = self
            .lightmap_secondary_textures
            .to_array(self.lightmaps.size())
            .xfile_deserialize_into(de, ())?;
        let lightmap_secondary_textures_b = self
            .lightmap_secondary_textures_b
            .to_array(self.lightmaps.size())
            .xfile_deserialize_into(de, ())?;
        let terrain_scorch_images = self
            .terrain_scorch_images
            .into_iter()
            .map(|i| i.xfile_deserialize_into(de, ()).map(|r| r.map(|p| *p)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let vd = self.vd.xfile_deserialize_into(de, self.vertex_count)?;
        let vld = self
            .vld
            .xfile_deserialize_into(de, self.vertex_layer_data_size)?;
        let indices = self.indices.to_vec(de)?;

        Ok(GfxWorldDraw {
            reflection_probes,
            reflection_probe_textures,
            lightmaps,
            lightmap_primary_textures,
            lightmap_secondary_textures,
            lightmap_secondary_textures_b,
            terrain_scorch_images,
            vertex_count: self.vertex_count,
            vd,
            vertex_layer_data_size: self.vertex_layer_data_size,
            vld,
            vertex_stream_2_data_size: self.vertex_stream_2_data_size,
            indices,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxReflectionProbeRaw<'a> {
    pub origin: [f32; 3],
    pub image: Ptr32<'a, GfxImageRaw<'a>>,
    pub probe_volumes: FatPointerCountLastU32<'a, GfxReflectionProbeVolumeDataRaw>,
}
assert_size!(GfxReflectionProbeRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxReflectionProbe {
    pub origin: Vec3,
    pub image: Option<Box<GfxImage>>,
    pub probe_volumes: Vec<GfxReflectionProbeVolumeData>,
}

impl<'a> XFileDeserializeInto<GfxReflectionProbe, ()> for GfxReflectionProbeRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxReflectionProbe> {
        let origin = self.origin.into();
        let image = self.image.xfile_deserialize_into(de, ())?;
        let probe_volumes = self.probe_volumes.to_vec_into(de)?;

        Ok(GfxReflectionProbe {
            origin,
            image,
            probe_volumes,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxReflectionProbeVolumeDataRaw {
    pub volume_planes: [[f32; 4]; 6],
}
assert_size!(GfxReflectionProbeVolumeDataRaw, 96);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxReflectionProbeVolumeData {
    pub volume_planes: [Vec4; 6],
}

impl From<GfxReflectionProbeVolumeDataRaw> for GfxReflectionProbeVolumeData {
    fn from(value: GfxReflectionProbeVolumeDataRaw) -> Self {
        Self {
            volume_planes: [
                value.volume_planes[0].into(),
                value.volume_planes[1].into(),
                value.volume_planes[2].into(),
                value.volume_planes[3].into(),
                value.volume_planes[4].into(),
                value.volume_planes[5].into(),
            ],
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightmapArrayRaw<'a> {
    pub primary: Ptr32<'a, GfxImageRaw<'a>>,
    pub secondary: Ptr32<'a, GfxImageRaw<'a>>,
    pub secondary_b: Ptr32<'a, GfxImageRaw<'a>>,
}
assert_size!(GfxLightmapArrayRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightmapArray {
    pub primary: Option<Box<GfxImage>>,
    pub secondary: Option<Box<GfxImage>>,
    pub secondary_b: Option<Box<GfxImage>>,
}

impl<'a> XFileDeserializeInto<GfxLightmapArray, ()> for GfxLightmapArrayRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLightmapArray> {
        let primary = self.primary.xfile_deserialize_into(de, ())?;
        let secondary = self.secondary.xfile_deserialize_into(de, ())?;
        let secondary_b = self.secondary_b.xfile_deserialize_into(de, ())?;

        Ok(GfxLightmapArray {
            primary,
            secondary,
            secondary_b,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldVertexDataRaw<'a> {
    pub vertices: Ptr32<'a, GfxWorldVertexRaw>,
    #[cfg_attr(not(feature = "d3d9"), allow(dead_code))]
    pub world_vb: Ptr32<'a, ()>,
}
assert_size!(GfxWorldVertexDataRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldVertexData {
    pub vertices: Vec<GfxWorldVertex>,
    pub world_vb: Option<Box<GfxVertexBuffer>>,
}

impl<'a> XFileDeserializeInto<GfxWorldVertexData, u32> for GfxWorldVertexDataRaw<'a> {
    #[cfg(feature = "d3d9")]
    fn xfile_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        vertex_count: u32,
    ) -> Result<GfxWorldVertexData> {
        let vertices = self
            .vertices
            .to_array(vertex_count as _)
            .to_vec_into::<GfxWorldVertex>(de)?;
        let world_vb = if de.create_d3d9() {
            let count = vertex_count as usize * sizeof!(GfxWorldVertex);
            let mut vb = Option::<IDirect3DVertexBuffer9>::None;
            unsafe {
                de.d3d9_state().unwrap().device.CreateVertexBuffer(
                    count as u32,
                    8,
                    0,
                    D3DPOOL_DEFAULT,
                    addr_of_mut!(vb),
                    core::ptr::null_mut(),
                )
            }?;
            let vb = vb.unwrap();
            let mut ppbdata = core::ptr::null_mut();
            unsafe { vb.Lock(0, 0, addr_of_mut!(ppbdata), 0) }?;
            unsafe { std::ptr::copy(vertices.as_ptr().cast::<u8>(), ppbdata.cast::<u8>(), count) };
            unsafe { vb.Unlock() }?;
            Some(Box::new(GfxVertexBuffer(vb)))
        } else {
            None
        };

        Ok(GfxWorldVertexData { vertices, world_vb })
    }

    #[cfg(not(feature = "d3d9"))]
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        vertex_count: u32,
    ) -> Result<GfxWorldVertexData> {
        let vertices = self
            .vertices
            .to_array(vertex_count as _)
            .to_vec_into::<GfxWorldVertex>(de)?;

        Ok(GfxWorldVertexData {
            vertices,
            world_vb: None,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldVertexRaw {
    pub xyz: [f32; 3],
    pub binormal_sign: f32,
    pub color: GfxColor,
    pub tex_coord: [f32; 2],
    pub lmap_coord: [f32; 2],
    pub normal: [u8; 4],
    pub tangent: [u8; 4],
}
assert_size!(GfxWorldVertexRaw, 44);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldVertex {
    pub xyz: Vec3,
    pub binormal_sign: f32,
    pub color: GfxColor,
    pub tex_coord: Vec2,
    pub lmap_coord: Vec2,
    pub normal: [u8; 4],
    pub tangent: [u8; 4],
}

impl From<GfxWorldVertexRaw> for GfxWorldVertex {
    fn from(value: GfxWorldVertexRaw) -> Self {
        let xyz = value.xyz.into();
        let tex_coord = value.tex_coord.into();
        let lmap_coord = value.lmap_coord.into();

        Self {
            xyz,
            binormal_sign: value.binormal_sign,
            color: value.color,
            tex_coord,
            lmap_coord,
            normal: value.normal,
            tangent: value.tangent,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldVertexLayerDataRaw<'a> {
    pub data: Ptr32<'a, u8>,
    #[cfg_attr(not(feature = "d3d9"), allow(dead_code))]
    pub layer_vb: Ptr32<'a, ()>,
}
assert_size!(GfxWorldVertexLayerDataRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldVertexLayerData {
    pub data: Vec<u8>,
    pub layer_vb: Option<Box<GfxVertexBuffer>>,
}

impl<'a> XFileDeserializeInto<GfxWorldVertexLayerData, u32> for GfxWorldVertexLayerDataRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        vertex_layer_data_size: u32,
    ) -> Result<GfxWorldVertexLayerData> {
        let data = self.data.to_array(vertex_layer_data_size as _).to_vec(de)?;

        Ok(GfxWorldVertexLayerData {
            data,
            layer_vb: None,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightGridRaw<'a> {
    pub has_light_regions: bool,
    #[allow(dead_code)]
    pad: [u8; 3],
    pub sun_primary_light_index: u32,
    pub mins: [u16; 3],
    pub maxs: [u16; 3],
    pub row_axis: u32,
    pub col_axis: u32,
    pub row_data_start: Ptr32<'a, u16>,
    pub raw_row_data: FatPointerCountFirstU32<'a, u16>,
    pub entries: FatPointerCountFirstU32<'a, GfxLightGridEntry>,
    pub colors: FatPointerCountFirstU32<'a, GfxCompressedLightGridColors>,
}
assert_size!(GfxLightGridRaw, 56);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightGrid {
    pub has_light_regions: bool,
    pub sun_primary_light_index: usize,
    pub mins: [u16; 3],
    pub maxs: [u16; 3],
    pub row_axis: u32,
    pub col_axis: u32,
    pub row_data_start: Vec<u16>,
    pub raw_row_data: Vec<u16>,
    pub entries: Vec<GfxLightGridEntry>,
    pub colors: Vec<GfxCompressedLightGridColors>,
}

impl<'a> XFileDeserializeInto<GfxLightGrid, ()> for GfxLightGridRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLightGrid> {
        let row_data_start = self
            .row_data_start
            .to_array(
                self.maxs[self.row_axis as usize] as usize
                    - self.mins[self.row_axis as usize] as usize
                    + 1,
            )
            .to_vec(de)?;
        let raw_row_data = self.raw_row_data.to_vec(de)?;
        let entries = self.entries.to_vec(de)?;
        let colors = self.colors.to_vec(de)?;

        Ok(GfxLightGrid {
            has_light_regions: self.has_light_regions,
            sun_primary_light_index: self.sun_primary_light_index as _,
            mins: self.mins,
            maxs: self.maxs,
            row_axis: self.row_axis,
            col_axis: self.col_axis,
            row_data_start,
            raw_row_data,
            entries,
            colors,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxLightGridEntry {
    pub colors_index: u16,
    pub primary_light_index: u8,
    pub needs_trace: u8,
}
assert_size!(GfxLightGridEntry, 4);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxCompressedLightGridColors {
    #[serde(with = "serde_arrays")]
    pub rgb: [[u8; 3]; 56],
}
assert_size!(GfxCompressedLightGridColors, 168);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxBrushModelRaw {
    pub writable: GfxBrushModelWritableRaw,
    pub bounds: [[f32; 3]; 2],
    pub surface_count: u32,
    pub start_surf_index: u32,
}
assert_size!(GfxBrushModelRaw, 60);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxBrushModel {
    pub writable: GfxBrushModelWritable,
    pub bounds: [Vec3; 2],
    pub surface_count: usize,
    pub start_surf_index: usize,
}

impl From<GfxBrushModelRaw> for GfxBrushModel {
    fn from(value: GfxBrushModelRaw) -> Self {
        let writable = value.writable.into();
        let bounds = [value.bounds[0].into(), value.bounds[1].into()];

        Self {
            writable,
            bounds,
            surface_count: value.surface_count as _,
            start_surf_index: value.start_surf_index as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxBrushModelWritableRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub mip_1_radius_sq: f32,
}
assert_size!(GfxBrushModelWritableRaw, 28);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxBrushModelWritable {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub mip_1_radius_sq: f32,
}

impl From<GfxBrushModelWritableRaw> for GfxBrushModelWritable {
    fn from(value: GfxBrushModelWritableRaw) -> Self {
        Self {
            mins: value.mins.into(),
            maxs: value.maxs.into(),
            mip_1_radius_sq: value.mip_1_radius_sq,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MaterialMemoryRaw<'a> {
    pub material: Ptr32<'a, MaterialRaw<'a>>,
    pub memory: i32,
}
assert_size!(MaterialMemoryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialMemory {
    pub material: Option<Box<Material>>,
    pub memory: usize,
}

impl<'a> XFileDeserializeInto<MaterialMemory, ()> for MaterialMemoryRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<MaterialMemory> {
        let material = self.material.xfile_deserialize_into(de, ())?;

        Ok(MaterialMemory {
            material,
            memory: self.memory as _,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SunflareRaw<'a> {
    pub has_valid_data: bool,
    #[allow(dead_code)]
    pad: [u8; 3],
    pub sprite_material: Ptr32<'a, MaterialRaw<'a>>,
    pub flare_material: Ptr32<'a, MaterialRaw<'a>>,
    pub sprite_size: f32,
    pub flare_min_size: f32,
    pub flare_min_dot: f32,
    pub flare_max_size: f32,
    pub flare_max_dot: f32,
    pub flare_max_alpha: f32,
    pub flare_fade_in_time: i32,
    pub flare_fade_out_time: i32,
    pub blind_min_dot: f32,
    pub blind_max_dot: f32,
    pub blind_max_darken: f32,
    pub blind_fade_in_time: i32,
    pub blind_fade_out_time: i32,
    pub glare_min_dot: f32,
    pub glare_max_dot: f32,
    pub glare_max_lighten: f32,
    pub glare_fade_in_time: i32,
    pub glare_fade_out_time: i32,
    pub sun_fx_position: [f32; 3],
}
assert_size!(SunflareRaw, 96);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Sunflare {
    pub has_valid_data: bool,
    pub sprite_material: Option<Box<Material>>,
    pub flare_material: Option<Box<Material>>,
    pub sprite_size: f32,
    pub flare_min_size: f32,
    pub flare_min_dot: f32,
    pub flare_max_size: f32,
    pub flare_max_dot: f32,
    pub flare_max_alpha: f32,
    pub flare_fade_in_time: i32,
    pub flare_fade_out_time: i32,
    pub blind_min_dot: f32,
    pub blind_max_dot: f32,
    pub blind_max_darken: f32,
    pub blind_fade_in_time: i32,
    pub blind_fade_out_time: i32,
    pub glare_min_dot: f32,
    pub glare_max_dot: f32,
    pub glare_max_lighten: f32,
    pub glare_fade_in_time: i32,
    pub glare_fade_out_time: i32,
    pub sun_fx_position: Vec3,
}

impl<'a> XFileDeserializeInto<Sunflare, ()> for SunflareRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<Sunflare> {
        let sprite_material = self.sprite_material.xfile_deserialize_into(de, ())?;
        let flare_material = self.flare_material.xfile_deserialize_into(de, ())?;
        let sun_fx_position = self.sun_fx_position.into();

        Ok(Sunflare {
            has_valid_data: self.has_valid_data,
            sprite_material,
            flare_material,
            sprite_size: self.sprite_size,
            flare_min_size: self.flare_min_size,
            flare_min_dot: self.flare_min_dot,
            flare_max_size: self.flare_max_size,
            flare_max_dot: self.flare_max_dot,
            flare_max_alpha: self.flare_max_alpha,
            flare_fade_in_time: self.flare_fade_in_time,
            flare_fade_out_time: self.flare_fade_out_time,
            blind_min_dot: self.blind_min_dot,
            blind_max_dot: self.blind_max_dot,
            blind_max_darken: self.blind_max_darken,
            blind_fade_in_time: self.blind_fade_in_time,
            blind_fade_out_time: self.blind_fade_out_time,
            glare_min_dot: self.glare_min_dot,
            glare_max_dot: self.glare_max_dot,
            glare_max_lighten: self.glare_max_lighten,
            glare_fade_in_time: self.glare_fade_in_time,
            glare_fade_out_time: self.glare_fade_out_time,
            sun_fx_position,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxSceneDynModel {
    pub info: XModelDrawInfo,
    pub dyn_ent_id: u16,
}
assert_size!(GfxSceneDynModel, 6);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxSceneDynBrush {
    pub info: BModelDrawInfo,
    pub dyn_ent_id: u16,
}
assert_size!(GfxSceneDynModel, 6);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct BModelDrawInfo {
    pub surf_id: u16,
}
assert_size!(BModelDrawInfo, 2);

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxShadowGeometryRaw<'a> {
    pub surface_count: u16,
    pub smodel_count: u16,
    pub sorted_surf_index: Ptr32<'a, u16>,
    pub smodel_index: Ptr32<'a, u16>,
}
assert_size!(GfxShadowGeometryRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxShadowGeometry {
    pub sorted_surf_index: Vec<u16>,
    pub smodel_index: Vec<u16>,
}

impl<'a> XFileDeserializeInto<GfxShadowGeometry, ()> for GfxShadowGeometryRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxShadowGeometry> {
        let sorted_surf_index = self
            .sorted_surf_index
            .to_array(self.surface_count as _)
            .to_vec(de)?;
        let smodel_index = self
            .smodel_index
            .to_array(self.smodel_count as _)
            .to_vec(de)?;

        Ok(GfxShadowGeometry {
            sorted_surf_index,
            smodel_index,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightRegionRaw<'a> {
    pub hulls: FatPointerCountFirstU32<'a, GfxLightRegionHullRaw<'a>>,
}
assert_size!(GfxLightRegionRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightRegion {
    pub hulls: Vec<GfxLightRegionHull>,
}

impl<'a> XFileDeserializeInto<GfxLightRegion, ()> for GfxLightRegionRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLightRegion> {
        let hulls = self.hulls.xfile_deserialize_into(de, ())?;

        Ok(GfxLightRegion { hulls })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightRegionHullRaw<'a> {
    pub kdop_mid_point: [[f32; 3]; 3],
    pub kdop_half_size: [[f32; 3]; 3],
    pub axis: FatPointerCountFirstU32<'a, GfxLightRegionAxisRaw>,
}
assert_size!(GfxLightRegionHullRaw, 80);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightRegionHull {
    pub kdop_mid_point: Mat3,
    pub kdop_half_size: Mat3,
    pub axis: Vec<GfxLightRegionAxis>,
}

impl<'a> XFileDeserializeInto<GfxLightRegionHull, ()> for GfxLightRegionHullRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxLightRegionHull> {
        let kdop_mid_point = self.kdop_mid_point.into();
        let kdop_half_size = self.kdop_half_size.into();
        let axis = self.axis.to_vec_into(de)?;

        Ok(GfxLightRegionHull {
            kdop_mid_point,
            kdop_half_size,
            axis,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightRegionAxisRaw {
    pub dir: [f32; 3],
    pub mid_point: f32,
    pub half_size: f32,
}
assert_size!(GfxLightRegionAxisRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightRegionAxis {
    pub dir: Vec3,
    pub mid_point: f32,
    pub half_size: f32,
}

impl From<GfxLightRegionAxisRaw> for GfxLightRegionAxis {
    fn from(value: GfxLightRegionAxisRaw) -> Self {
        Self {
            dir: value.dir.into(),
            mid_point: value.mid_point,
            half_size: value.half_size,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldDpvsStaticRaw<'a> {
    pub smodel_count: u32,
    pub dynamic_smodel_count: u32,
    pub static_surface_count: u32,
    pub lit_surfs_begin: u32,
    pub lit_surfs_end: u32,
    pub decal_surfs_begin: u32,
    pub decal_surfs_end: u32,
    pub emissive_surfs_begin: u32,
    pub emissive_surfs_end: u32,
    pub smodel_vis_data_count: u32,
    pub surface_vis_data_count: u32,
    pub smodel_vis_data: [Ptr32<'a, u8>; 3],
    pub surface_vis_data: [Ptr32<'a, u8>; 3],
    pub smodel_vis_data_camera_saved: Ptr32<'a, u8>,
    pub surface_vis_data_camera_saved: Ptr32<'a, u8>,
    pub lod_data: Ptr32<'a, u32>,
    pub sorted_surf_index: Ptr32<'a, u16>,
    pub smodel_insts: Ptr32<'a, GfxStaticModelInstRaw>,
    pub surfaces: Ptr32<'a, GfxSurfaceRaw<'a>>,
    pub cull_groups: Ptr32<'a, GfxCullGroupRaw>,
    pub smodel_draw_insts: Ptr32<'a, GfxStaticModelDrawInstRaw<'a>>,
    pub surface_materials: Ptr32<'a, GfxDrawSurf>,
    pub surface_casts_sun_shadow: Ptr32<'a, u32>,
    pub usage_count: i32,
}
assert_size!(GfxWorldDpvsStaticRaw, 112);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldDpvsStatic {
    pub smodel_count: usize,
    pub dynamic_smodel_count: usize,
    pub static_surface_count: usize,
    pub lit_surfs_begin: u32,
    pub lit_surfs_end: u32,
    pub decal_surfs_begin: u32,
    pub decal_surfs_end: u32,
    pub emissive_surfs_begin: u32,
    pub emissive_surfs_end: u32,
    pub smodel_vis_data_count: usize,
    pub surface_vis_data_count: usize,
    pub smodel_vis_data: [Vec<u8>; 3],
    pub surface_vis_data: [Vec<u8>; 3],
    pub smodel_vis_data_camera_saved: Vec<u8>,
    pub surface_vis_data_camera_saved: Vec<u8>,
    pub lod_data: Vec<u32>,
    pub sorted_surf_index: Vec<u16>,
    pub smodel_insts: Vec<GfxStaticModelInst>,
    pub surfaces: Vec<GfxSurface>,
    pub cull_groups: Vec<GfxCullGroup>,
    pub smodel_draw_insts: Vec<GfxStaticModelDrawInst>,
    pub surface_materials: Vec<GfxDrawSurf>,
    pub surface_casts_sun_shadow: Vec<u32>,
    pub usage_count: usize,
}

impl<'a> XFileDeserializeInto<GfxWorldDpvsStatic, (i32, i32)> for GfxWorldDpvsStaticRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        (surface_count, cull_groups_count): (i32, i32),
    ) -> Result<GfxWorldDpvsStatic> {
        let smodel_vis_data = [
            self.smodel_vis_data[0]
                .to_array(self.smodel_count as _)
                .to_vec(de)?,
            self.smodel_vis_data[1]
                .to_array(self.smodel_count as _)
                .to_vec(de)?,
            self.smodel_vis_data[2]
                .to_array(self.smodel_count as _)
                .to_vec(de)?,
        ];

        let surface_vis_data = [
            self.surface_vis_data[0]
                .to_array(self.static_surface_count as _)
                .to_vec(de)?,
            self.surface_vis_data[1]
                .to_array(self.static_surface_count as _)
                .to_vec(de)?,
            self.surface_vis_data[2]
                .to_array(self.static_surface_count as _)
                .to_vec(de)?,
        ];

        let smodel_vis_data_camera_saved = self
            .smodel_vis_data_camera_saved
            .to_array(self.smodel_count as _)
            .to_vec(de)?;
        let surface_vis_data_camera_saved = self
            .surface_vis_data_camera_saved
            .to_array(self.static_surface_count as _)
            .to_vec(de)?;
        let lod_data = self
            .lod_data
            .to_array(self.smodel_vis_data_count as usize * 2)
            .to_vec(de)?;
        let sorted_surf_index = self
            .sorted_surf_index
            .to_array(self.static_surface_count as _)
            .to_vec_into(de)?;
        let smodel_insts = self
            .smodel_insts
            .to_array(self.smodel_count as _)
            .to_vec_into(de)?;
        let surfaces = self
            .surfaces
            .to_array(surface_count as _)
            .xfile_deserialize_into(de, ())?;
        let cull_groups = self
            .cull_groups
            .to_array(cull_groups_count as _)
            .to_vec_into(de)?;
        let smodel_draw_insts = self
            .smodel_draw_insts
            .to_array(self.smodel_count as _)
            .xfile_deserialize_into(de, ())?;
        let surface_materials = self
            .surface_materials
            .to_array(self.static_surface_count as _)
            .to_vec(de)?;
        let surface_casts_sun_shadow = self
            .surface_casts_sun_shadow
            .to_array(self.surface_vis_data_count as _)
            .to_vec(de)?;

        Ok(GfxWorldDpvsStatic {
            smodel_count: self.smodel_count as _,
            dynamic_smodel_count: self.dynamic_smodel_count as _,
            static_surface_count: self.static_surface_count as _,
            lit_surfs_begin: self.lit_surfs_begin,
            lit_surfs_end: self.lit_surfs_end,
            decal_surfs_begin: self.decal_surfs_begin,
            decal_surfs_end: self.decal_surfs_end,
            emissive_surfs_begin: self.emissive_surfs_begin,
            emissive_surfs_end: self.emissive_surfs_end,
            smodel_vis_data_count: self.smodel_vis_data_count as _,
            surface_vis_data_count: self.surface_vis_data_count as _,
            smodel_vis_data,
            surface_vis_data,
            smodel_vis_data_camera_saved,
            surface_vis_data_camera_saved,
            lod_data,
            sorted_surf_index,
            smodel_insts,
            surfaces,
            cull_groups,
            smodel_draw_insts,
            surface_materials,
            surface_casts_sun_shadow,
            usage_count: self.usage_count as _,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxStaticModelInstRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub lighting_origin: [f32; 3],
    pub ground_lighting: GfxColor,
}
assert_size!(GfxStaticModelInstRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxStaticModelInst {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub lighting_origin: Vec3,
    pub ground_lighting: GfxColor,
}

impl From<GfxStaticModelInstRaw> for GfxStaticModelInst {
    fn from(value: GfxStaticModelInstRaw) -> Self {
        Self {
            mins: value.mins.into(),
            maxs: value.maxs.into(),
            lighting_origin: value.lighting_origin.into(),
            ground_lighting: value.ground_lighting,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxSurfaceRaw<'a> {
    pub tris: SrfTrianglesRaw,
    pub material: Ptr32<'a, MaterialRaw<'a>>,
    pub lightmap_index: u8,
    pub reflection_probe_index: u8,
    pub primary_light_index: u8,
    pub flags: u8,
    pub bounds: [[f32; 3]; 2],
}
assert_size!(GfxSurfaceRaw, 80);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxSurface {
    pub tris: SrfTriangles,
    pub material: Option<Box<Material>>,
    pub lightmap_index: usize,
    pub reflection_probe_index: usize,
    pub primary_light_index: usize,
    pub flags: u8,
    pub bounds: [Vec3; 2],
}

impl<'a> XFileDeserializeInto<GfxSurface, ()> for GfxSurfaceRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxSurface> {
        let tris = self.tris.into();
        let material = self.material.xfile_deserialize_into(de, ())?;
        let bounds = [self.bounds[0].into(), self.bounds[1].into()];

        Ok(GfxSurface {
            tris,
            material,
            lightmap_index: self.lightmap_index as _,
            reflection_probe_index: self.reflection_probe_index as _,
            primary_light_index: self.primary_light_index as _,
            flags: self.flags,
            bounds,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SrfTrianglesRaw {
    pub mins: [f32; 3],
    pub vertex_layer_data: i32,
    pub maxs: [f32; 3],
    pub first_vertex: i32,
    pub vertex_count: u16,
    pub tri_count: u16,
    pub base_index: i32,
    pub himip_radius_sq: f32,
    pub stream_2_byte_offset: i32,
}
assert_size!(SrfTrianglesRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct SrfTriangles {
    pub mins: Vec3,
    pub vertex_layer_data: i32,
    pub maxs: Vec3,
    pub first_vertex: i32,
    pub vertex_count: usize,
    pub tri_count: usize,
    pub base_index: usize,
    pub himip_radius_sq: f32,
    pub stream_2_byte_offset: i32,
}

impl From<SrfTrianglesRaw> for SrfTriangles {
    fn from(value: SrfTrianglesRaw) -> Self {
        Self {
            mins: value.mins.into(),
            vertex_layer_data: value.vertex_layer_data,
            maxs: value.maxs.into(),
            first_vertex: value.first_vertex,
            vertex_count: value.vertex_count as _,
            tri_count: value.tri_count as _,
            base_index: value.base_index as _,
            himip_radius_sq: value.himip_radius_sq,
            stream_2_byte_offset: value.stream_2_byte_offset,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxCullGroupRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub surface_count: i32,
    pub start_surf_index: i32,
}
assert_size!(GfxCullGroupRaw, 32);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxCullGroup {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub surface_count: usize,
    pub start_surf_index: usize,
}

impl From<GfxCullGroupRaw> for GfxCullGroup {
    fn from(value: GfxCullGroupRaw) -> Self {
        Self {
            mins: value.mins.into(),
            maxs: value.maxs.into(),
            surface_count: value.surface_count as _,
            start_surf_index: value.start_surf_index as _,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxStaticModelDrawInstRaw<'a> {
    pub cull_dist: f32,
    pub placement: GfxPackedPlacementRaw,
    pub model: Ptr32<'a, XModelRaw<'a>>,
    pub flags: i32,
    pub smodel_cache_index: [u16; 4],
    pub lighting_handle: u16,
    pub reflection_probe_index: u8,
    pub primary_light_index: u8,
}
assert_size!(GfxStaticModelDrawInstRaw, 76);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxStaticModelDrawInst {
    pub cull_dist: f32,
    pub placement: GfxPackedPlacement,
    pub model: Option<Box<XModel>>,
    pub flags: i32,
    pub smodel_cache_index: [u16; 4],
    pub lighting_handle: u16,
    pub reflection_probe_index: usize,
    pub primary_light_index: usize,
}

impl<'a> XFileDeserializeInto<GfxStaticModelDrawInst, ()> for GfxStaticModelDrawInstRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxStaticModelDrawInst> {
        let placement = self.placement.into();
        let model = self.model.xfile_deserialize_into(de, ())?;

        Ok(GfxStaticModelDrawInst {
            cull_dist: self.cull_dist,
            placement,
            model,
            flags: self.flags,
            smodel_cache_index: self.smodel_cache_index,
            lighting_handle: self.lighting_handle,
            reflection_probe_index: self.reflection_probe_index as _,
            primary_light_index: self.primary_light_index as _,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxPackedPlacementRaw {
    pub origin: [f32; 3],
    pub axis: [[f32; 3]; 3],
    pub scale: f32,
}
assert_size!(GfxPackedPlacementRaw, 52);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxPackedPlacement {
    pub origin: Vec3,
    pub axis: Mat3,
    pub scale: f32,
}

impl From<GfxPackedPlacementRaw> for GfxPackedPlacement {
    fn from(value: GfxPackedPlacementRaw) -> Self {
        Self {
            origin: value.origin.into(),
            axis: value.axis.into(),
            scale: value.scale,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldDpvsDynamicRaw<'a> {
    pub dyn_ent_client_word_count: [u32; 2],
    pub dyn_ent_client_count: [u32; 2],
    pub dyn_ent_cell_bits: [Ptr32<'a, u32>; 2],
    pub dyn_ent_vis_data: [[Ptr32<'a, u8>; 2]; 3],
}
assert_size!(GfxWorldDpvsDynamicRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldDpvsDynamic {
    pub dyn_ent_cell_bits: [Vec<u32>; 2],
    pub dyn_ent_vis_data: [[Vec<u8>; 2]; 3],
}

impl<'a> XFileDeserializeInto<GfxWorldDpvsDynamic, i32> for GfxWorldDpvsDynamicRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        cell_count: i32,
    ) -> Result<GfxWorldDpvsDynamic> {
        let dyn_ent_cell_bits = [
            self.dyn_ent_cell_bits[0]
                .to_array(self.dyn_ent_client_word_count[0] as usize * cell_count as usize)
                .to_vec(de)?,
            self.dyn_ent_cell_bits[1]
                .to_array(self.dyn_ent_client_word_count[1] as usize * cell_count as usize)
                .to_vec(de)?,
        ];

        let dyn_ent_vis_data = [
            [
                self.dyn_ent_vis_data[0][0]
                    .to_array(self.dyn_ent_client_word_count[0] as usize * 32)
                    .to_vec(de)?,
                self.dyn_ent_vis_data[0][1]
                    .to_array(self.dyn_ent_client_word_count[1] as usize * 32)
                    .to_vec(de)?,
            ],
            [
                self.dyn_ent_vis_data[1][0]
                    .to_array(self.dyn_ent_client_word_count[0] as usize * 32)
                    .to_vec(de)?,
                self.dyn_ent_vis_data[1][1]
                    .to_array(self.dyn_ent_client_word_count[1] as usize * 32)
                    .to_vec(de)?,
            ],
            [
                self.dyn_ent_vis_data[2][0]
                    .to_array(self.dyn_ent_client_word_count[0] as usize * 32)
                    .to_vec(de)?,
                self.dyn_ent_vis_data[2][1]
                    .to_array(self.dyn_ent_client_word_count[1] as usize * 32)
                    .to_vec(de)?,
            ],
        ];

        Ok(GfxWorldDpvsDynamic {
            dyn_ent_cell_bits,
            dyn_ent_vis_data,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldLodChainRaw {
    pub origin: [f32; 3],
    pub last_dist: f32,
    pub first_lod_info: u32,
    pub lod_info_count: u16,
}
assert_size!(GfxWorldLodChainRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldLodChain {
    pub origin: Vec3,
    pub last_dist: f32,
    pub first_lod_info: u32,
    pub lod_info_count: u16,
}

impl From<GfxWorldLodChainRaw> for GfxWorldLodChain {
    fn from(value: GfxWorldLodChainRaw) -> Self {
        Self {
            origin: value.origin.into(),
            last_dist: value.last_dist,
            first_lod_info: value.first_lod_info,
            lod_info_count: value.lod_info_count,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxWorldLodInfo {
    pub dist: f32,
    pub first_surf: u32,
    pub surf_count: u16,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWaterBufferRaw<'a> {
    pub buffer: FatPointerCountFirstU32<'a, [f32; 4]>,
}
assert_size!(GfxWaterBufferRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWaterBuffer {
    pub buffer: Vec<Vec4>,
}

impl<'a> XFileDeserializeInto<GfxWaterBuffer, ()> for GfxWaterBufferRaw<'a> {
    fn xfile_deserialize_into(
        &self,
        de: &mut impl T5XFileDeserialize,
        _data: (),
    ) -> Result<GfxWaterBuffer> {
        let buffer = self.buffer.to_vec_into(de)?;
        Ok(GfxWaterBuffer { buffer })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct OccluderRaw {
    pub flags: u32,
    pub name: GfxName16,
    pub points: [[f32; 3]; 4],
}
assert_size!(OccluderRaw, 68);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Occluder {
    pub flags: u32,
    pub name: String,
    pub points: [Vec3; 4],
}

impl From<OccluderRaw> for Occluder {
    fn from(value: OccluderRaw) -> Self {
        Self {
            flags: value.flags,
            name: value.name.to_string(),
            points: [
                value.points[0].into(),
                value.points[1].into(),
                value.points[2].into(),
                value.points[3].into(),
            ],
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxOutdoorBoundsRaw {
    pub bounds: [[f32; 3]; 2],
}
assert_size!(GfxOutdoorBoundsRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxOutdoorBounds {
    pub bounds: [Vec3; 2],
}

impl From<GfxOutdoorBoundsRaw> for GfxOutdoorBounds {
    fn from(value: GfxOutdoorBoundsRaw) -> Self {
        Self {
            bounds: [value.bounds[0].into(), value.bounds[1].into()],
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxHeroLightRaw {
    pub type_: u8,
    #[allow(dead_code)]
    unused: [u8; 3],
    pub color: [f32; 3],
    pub dir: [f32; 3],
    pub origin: [f32; 3],
    pub radius: f32,
    pub cos_half_fov_outer: f32,
    pub cos_half_fov_inner: f32,
    pub exponent: i32,
}
assert_size!(GfxHeroLightRaw, 56);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxHeroLight {
    pub type_: u8,
    pub color: Vec3,
    pub dir: Vec3,
    pub origin: Vec3,
    pub radius: f32,
    pub cos_half_fov_outer: f32,
    pub cos_half_fov_inner: f32,
    pub exponent: i32,
}

impl From<GfxHeroLightRaw> for GfxHeroLight {
    fn from(value: GfxHeroLightRaw) -> Self {
        Self {
            type_: value.type_,
            color: value.color.into(),
            dir: value.dir.into(),
            origin: value.origin.into(),
            radius: value.radius,
            cos_half_fov_outer: value.cos_half_fov_outer,
            cos_half_fov_inner: value.cos_half_fov_inner,
            exponent: value.exponent,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxHeroLightTreeRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
}
assert_size!(GfxHeroLightTreeRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxHeroLightTree {
    pub mins: Vec3,
    pub maxs: Vec3,
}

impl From<GfxHeroLightTreeRaw> for GfxHeroLightTree {
    fn from(value: GfxHeroLightTreeRaw) -> Self {
        Self {
            mins: value.mins.into(),
            maxs: value.maxs.into(),
        }
    }
}
