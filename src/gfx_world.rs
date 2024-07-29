use common::{IDirect3DVertexBuffer9, Mat3, Mat4, Vec2, Vec3, Vec4};

use crate::*;

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
    pub sky_image: Ptr32<'a, techset::GfxImageRaw<'a>>,
    pub sky_sampler_state: u8,
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
    pub outdoor_image: Ptr32<'a, techset::GfxImageRaw<'a>>,
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
    pub water_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub corona_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub rope_material: Ptr32<'a, techset::MaterialRaw<'a>>,
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
    pub sky_image: Option<Box<techset::GfxImage>>,
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
    pub outdoor_image: Option<Box<techset::GfxImage>>,
    pub cell_caster_bits: Vec<u32>,
    pub scene_dyn_model: Option<Box<GfxSceneDynModel>>,
    pub scene_dyn_brush: Option<Box<GfxSceneDynBrush>>,
    pub primary_light_entity_shadow_vis: Vec<u32>,
    pub primary_light_dyn_ent_shadow_vis: [Vec<u32>; 2],
    pub non_sun_primary_light_for_model_dyn_ent: Vec<u8>,
    pub shadow_geom: Option<Box<GfxShadowGeometry>>,
    pub light_region: Option<Box<GfxLightRegion>>,
    pub dpvs: GfxWorldDpvsStatic,
    pub dpvs_dyn: GfxWorldDpvsDynamic,
    pub world_lod_chains: Vec<GfxWorldLodChain>,
    pub world_lod_infos: Vec<GfxWorldLodInfo>,
    pub world_lod_surfaces: Vec<u32>,
    pub water_direction: f32,
    pub water_buffers: [GfxWaterBuffer; 2],
    pub water_material: Option<Box<techset::Material>>,
    pub corona_material: Option<Box<techset::Material>>,
    pub rope_material: Option<Box<techset::Material>>,
    pub occluders: Vec<Occluder>,
    pub outdoor_bounds: Vec<GfxOutdoorBounds>,
    pub hero_light_count: u32,
    pub hero_light_tree_count: u32,
    pub hero_lights: Vec<GfxHeroLight>,
    pub hero_light_tree: Vec<GfxHeroLightTree>,
}

impl<'a, const MAX_LOCAL_CLIENTS: usize> XFileInto<GfxWorld<MAX_LOCAL_CLIENTS>, ()>
    for GfxWorldRaw<'a, MAX_LOCAL_CLIENTS>
{
    fn xfile_into(
        &self,
        _de: &mut T5XFileDeserializer,
        _data: (),
    ) -> Result<GfxWorld<MAX_LOCAL_CLIENTS>> {
        todo!()
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxName64(#[serde(with = "serde_arrays")] [u8; 64]);

impl Display for GfxName64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self
            .0
            .into_iter()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
            .unwrap_or(64);
        let s = self.0[..len]
            .iter()
            .map(|c| *c as char)
            .collect::<String>();
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self
            .0
            .into_iter()
            .enumerate()
            .find(|(_, c)| *c == 0)
            .map(|(i, _)| i)
            .unwrap_or(64);
        let s = self.0[..len]
            .iter()
            .map(|c| *c as char)
            .collect::<String>();
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
    #[serde(with = "serde_arrays")]
    pub sun_settings: [GfxWorldSunColor; MAX_LOCAL_CLIENTS],
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
    pad: [u8; 4],
    pub view_matrix: [[f32; 4]; 4],
    pub proj_matrix: [[f32; 4]; 4],
    pub def: Ptr32<'a, light::GfxLightDefRaw<'a>>,
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
    pub spot_shadows_index: u32,
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
    pub def: Option<Box<light::GfxLightDef>>,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxShadowMapVolumeRaw {
    pub control: u32,
    pad: [u8; 12],
}
assert_size!(GfxShadowMapVolumeRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxShadowMapVolume {
    pub control: u32,
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
    pub planes: Ptr32<'a, xmodel::CPlaneRaw>,
    pub nodes: Ptr32<'a, u16>,
    pub scene_ent_cell_bits: Ptr32<'a, u32>,
}
assert_size!(GfxWorldDpvsPlanesRaw, 16);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldDpvsPlanes {
    pub cell_count: i32,
    pub planes: Vec<xmodel::CPlane>,
    pub nodes: Vec<u16>,
    pub scene_ent_cell_bits: Vec<u32>,
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
    pub child_count: u16,
    pub surface_count: u16,
    pub start_surf_index: usize,
    pub smodel_index_count: u16,
    pub smodel_indexes: Vec<usize>,
    pub children_offset: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxPortalRaw<'a> {
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
    pub writable: GfxPortalWritable,
    pub plane: DpvsPlane,
    pub cell: Option<Box<GfxCell>>,
    pub vertices: Vec<Vec3>,
    pub hull_axis: [Vec3; 2],
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxPortalWritableRaw<'a> {
    pub is_queued: bool,
    pub is_ancestor: bool,
    pub recursion_depth: u8,
    pub hull_point_count: u8,
    pub hull_points: Ptr32<'a, [f32; 2]>,
    pub queued_parent: Ptr32<'a, GfxPortalRaw<'a>>,
}
assert_size!(GfxPortalWritableRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxPortalWritable {
    pub is_queued: bool,
    pub is_ancestor: bool,
    pub recursion_depth: u8,
    pub hull_point_count: u8,
    pub hull_points: Vec<Vec2>,
    pub queued_parent: Option<Box<GfxPortal>>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct DpvsPlaneRaw {
    pub coeffs: [f32; 4],
    pub side: [u8; 3],
    pad: u8,
}
assert_size!(DpvsPlaneRaw, 20);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct DpvsPlane {
    pub coeffs: Vec4,
    pub side: [u8; 3],
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldDrawRaw<'a> {
    pub reflection_probes: FatPointerCountFirstU32<'a, GfxReflectionProbeRaw<'a>>,
    pub reflection_probe_textures: Ptr32<'a, techset::GfxTextureRaw<'a>>,
    pub lightmaps: FatPointerCountFirstU32<'a, GfxLightmapArrayRaw<'a>>,
    pub lightmap_primary_textures: Ptr32<'a, techset::GfxTextureRaw<'a>>,
    pub lightmap_secondary_textures: Ptr32<'a, techset::GfxTextureRaw<'a>>,
    pub lightmap_secondary_textures_b: Ptr32<'a, techset::GfxTextureRaw<'a>>,
    pub terrain_scorch_images: [Ptr32<'a, techset::GfxImageRaw<'a>>; 31],
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
    pub reflection_probe_textures: Vec<techset::GfxTexture>,
    pub lightmaps: Vec<GfxLightmapArray>,
    pub lightmap_primary_textures: Vec<techset::GfxTexture>,
    pub lightmap_secondary_textures: Vec<techset::GfxTexture>,
    pub lightmap_secondary_textures_b: Vec<techset::GfxTexture>,
    pub terrain_scorch_images: Vec<techset::GfxImage>,
    pub vertex_count: u32,
    pub vd: GfxWorldVertexData,
    pub vertex_layer_data_size: u32,
    pub vld: GfxWorldVertexLayerData,
    pub vertex_stream_2_data_size: u32,
    pub indices: Vec<u16>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxReflectionProbeRaw<'a> {
    pub origin: [f32; 3],
    pub image: Ptr32<'a, techset::GfxImageRaw<'a>>,
    pub probe_volumes: FatPointerCountLastU32<'a, GfxReflectionProbeVolumeDataRaw>,
}
assert_size!(GfxReflectionProbeRaw, 24);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxReflectionProbe {
    pub origin: Vec3,
    pub image: Option<Box<techset::GfxImage>>,
    pub probe_volumes: Vec<GfxReflectionProbeVolumeData>,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightmapArrayRaw<'a> {
    pub primary: Ptr32<'a, techset::GfxImageRaw<'a>>,
    pub secondary: Ptr32<'a, techset::GfxImageRaw<'a>>,
    pub secondary_b: Ptr32<'a, techset::GfxImageRaw<'a>>,
}
assert_size!(GfxLightmapArrayRaw, 12);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxLightmapArray {
    pub primary: Option<Box<techset::GfxImage>>,
    pub secondary: Option<Box<techset::GfxImage>>,
    pub secondary_b: Option<Box<techset::GfxImage>>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldVertexDataRaw<'a> {
    pub vertices: Ptr32<'a, GfxWorldVertexRaw>,
    pub world_vb: Ptr32<'a, ()>,
}
assert_size!(GfxWorldVertexDataRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldVertexData {
    pub vertices: Vec<GfxWorldVertex>,
    pub world_vb: Option<Box<IDirect3DVertexBuffer9>>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldVertexRaw {
    pub xyz: [f32; 3],
    pub binormal_sign: f32,
    pub color: xmodel::GfxColor,
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
    pub color: xmodel::GfxColor,
    pub tex_coord: Vec2,
    pub lmap_coord: Vec2,
    pub normal: [u8; 4],
    pub tangent: [u8; 4],
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldVertexLayerDataRaw<'a> {
    pub data: Ptr32<'a, u8>,
    pub layer_vb: Ptr32<'a, ()>,
}
assert_size!(GfxWorldVertexLayerDataRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldVertexLayerData {
    pub data: Vec<u8>,
    pub layer_vb: Option<Box<IDirect3DVertexBuffer9>>,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightGridRaw<'a> {
    pub has_light_regions: bool,
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
    pub surface_count: u32,
    pub start_surf_index: usize,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct MaterialMemoryRaw<'a> {
    pub material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub memory: i32,
}
assert_size!(MaterialMemoryRaw, 8);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct MaterialMemory {
    pub material: Option<Box<techset::Material>>,
    pub memory: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct SunflareRaw<'a> {
    pub has_valid_data: bool,
    pad: [u8; 3],
    pub sprite_material: Ptr32<'a, techset::MaterialRaw<'a>>,
    pub flare_material: Ptr32<'a, techset::MaterialRaw<'a>>,
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
    pub sprite_material: Option<Box<techset::Material>>,
    pub flare_material: Option<Box<techset::Material>>,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct GfxSceneDynModel {
    pub info: xmodel::XModelDrawInfo,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxLightRegionHullRaw<'a> {
    pub kdop_mid_point: [f32; 9],
    pub kdop_half_size: [f32; 9],
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
    pub surface_materials: Ptr32<'a, techset::GfxDrawSurf>,
    pub sirface_casts_sun_shadow: Ptr32<'a, u32>,
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
    pub smodel_vis_data_count: u32,
    pub surface_vis_data_count: u32,
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
    pub surface_materials: Vec<techset::GfxDrawSurf>,
    pub sirface_casts_sun_shadow: Vec<u32>,
    pub usage_count: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxStaticModelInstRaw {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub lighting_origin: [f32; 3],
    pub ground_lighting: xmodel::GfxColor,
}
assert_size!(GfxStaticModelInstRaw, 40);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxStaticModelInst {
    pub mins: Vec3,
    pub maxs: Vec3,
    pub lighting_origin: Vec3,
    pub ground_lighting: xmodel::GfxColor,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxSurfaceRaw<'a> {
    pub tris: SrfTrianglesRaw,
    pub material: Ptr32<'a, techset::MaterialRaw<'a>>,
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
    pub material: Option<Box<techset::Material>>,
    pub lightmap_index: u8,
    pub reflection_probe_index: u8,
    pub primary_light_index: u8,
    pub flags: u8,
    pub bounds: [Vec3; 2],
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
    pub vertex_count: u16,
    pub tri_count: u16,
    pub base_index: usize,
    pub himip_radius_sq: f32,
    pub stream_2_byte_offset: i32,
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
    pub surface_count: i32,
    pub start_surf_index: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxStaticModelDrawInstRaw<'a> {
    pub cull_dist: f32,
    pub placement: GfxPackedPlacementRaw,
    pub model: Ptr32<'a, xmodel::XModelRaw<'a>>,
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
    pub model: Option<Box<xmodel::XModel>>,
    pub flags: i32,
    pub smodel_cache_index: [u16; 4],
    pub lighting_handle: u16,
    pub reflection_probe_index: usize,
    pub primary_light_index: usize,
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxWorldDpvsDynamicRaw<'a> {
    pub dyn_ent_client_word_count: [u32; 2],
    pub dyn_ent_client_count: [u32; 2],
    pub dyn_ent_cell_bits: [Ptr32<'a, u32>; 2],
    pub dyn_ent_vis_data: [[Ptr32<'a, u8>; 3]; 2],
}
assert_size!(GfxWorldDpvsDynamicRaw, 48);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct GfxWorldDpvsDynamic {
    pub dyn_ent_client_word_count: [u32; 2],
    pub dyn_ent_client_count: [u32; 2],
    pub dyn_ent_cell_bits: [Vec<u32>; 2],
    pub dyn_ent_vis_data: [[Vec<u8>; 3]; 2],
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, Debug, Deserialize)]
pub(crate) struct GfxHeroLightRaw {
    pub type_: u8,
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
