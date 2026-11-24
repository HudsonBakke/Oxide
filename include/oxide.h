#ifndef OXIDE_H
#define OXIDE_H

#include <stddef.h>
#include <stdint.h>

#ifdef _WIN32
    #define OXIDE_API __declspec(dllimport)
#else
    #define OXIDE_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Status / errors
 * ============================================================ */

typedef enum OxideStatus
{
    OXIDE_STATUS_OK = 0,
    OXIDE_STATUS_INVALID_ARGUMENT = 1,
    OXIDE_STATUS_INVALID_OPERATION = 2,
    OXIDE_STATUS_CONTENT_ERROR = 3,
    OXIDE_STATUS_GRAPHICS_ERROR = 4,
    OXIDE_STATUS_INTERNAL_ERROR = 5
} OxideStatus;

OXIDE_API const char*
oxide_last_error_message(void);


/* ============================================================
 * Math types
 * ============================================================ */

typedef struct OxideAngle
{
    float radians;
} OxideAngle;

typedef struct OxideVec2
{
    float x;
    float y;
} OxideVec2;

typedef struct OxideVec3
{
    float x;
    float y;
    float z;
} OxideVec3;

typedef struct OxideVec4
{
    float x;
    float y;
    float z;
    float w;
} OxideVec4;

typedef struct OxideQuaternion
{
    float x;
    float y;
    float z;
    float w;
} OxideQuaternion;

typedef struct OxideRect
{
    float x;
    float y;
    float width;
    float height;
} OxideRect;

typedef struct OxideTransform
{
    OxideVec3 position;
    OxideQuaternion rotation;
    OxideVec3 scale;
} OxideTransform;


/* ============================================================
 * Graphics value types
 * ============================================================ */

typedef struct OxideColor
{
    float r;
    float g;
    float b;
    float a;
} OxideColor;

typedef struct OxideVertex
{
    OxideVec3 position;
    OxideVec3 normal;
    OxideVec2 tex_coord;
} OxideVertex;

typedef struct OxidePointLight
{
    OxideVec3 position;
    OxideColor color;
    float intensity;
} OxidePointLight;

typedef struct OxideSpotLight
{
    OxideVec3 position;
    OxideVec3 direction;
    OxideAngle angle;
    OxideColor color;
    float intensity;
} OxideSpotLight;

typedef struct OxideDirectionalLight
{
    OxideVec3 direction;
    OxideColor color;
    float intensity;
} OxideDirectionalLight;


/* ============================================================
 * Handles
 * ============================================================ */

typedef struct OxideGraphicsHandle
{
    void* ptr;
} OxideGraphicsHandle;

typedef struct OxideCameraHandle
{
    uint64_t value;
} OxideCameraHandle;

typedef struct OxideContentManagerHandle
{
    uint64_t value;
} OxideContentManagerHandle;

typedef struct OxideMeshHandle
{
    uint64_t value;
} OxideMeshHandle;

typedef struct OxideMaterialHandle
{
    uint64_t value;
} OxideMaterialHandle;

typedef struct OxideTextureHandle
{
    uint64_t value;
} OxideTextureHandle;


/* ============================================================
 * Application
 * ============================================================ */

typedef struct OxideGameTime
{
    double delta_seconds;
    double total_seconds;
} OxideGameTime;

typedef struct OxideAppConfig
{
    const char* title;
    uint32_t width;
    uint32_t height;
} OxideAppConfig;

typedef void (*OxideInitializeCallback)(
    void* user_data
);

typedef void (*OxideUpdateCallback)(
    void* user_data,
    OxideGameTime time
);

typedef void (*OxideDrawCallback)(
    void* user_data,
    OxideGraphicsHandle graphics
);

typedef struct OxideGameCallbacks
{
    void* user_data;

    OxideInitializeCallback initialize;
    OxideUpdateCallback update;
    OxideDrawCallback draw;
} OxideGameCallbacks;

OXIDE_API OxideStatus
oxide_run(
    OxideAppConfig config,
    OxideGameCallbacks callbacks
);


/* ============================================================
 * Camera
 * ============================================================ */

OXIDE_API OxideStatus
oxide_camera_create(
    OxideCameraHandle* result
);

OXIDE_API OxideStatus
oxide_camera_destroy(
    OxideCameraHandle camera
);

OXIDE_API OxideStatus
oxide_camera_get_position(
    OxideCameraHandle camera,
    OxideVec3* result
);

OXIDE_API OxideStatus
oxide_camera_set_position(
    OxideCameraHandle camera,
    OxideVec3 position
);

OXIDE_API OxideStatus
oxide_camera_translate_by(
    OxideCameraHandle camera,
    OxideVec3 translation
);

OXIDE_API OxideStatus
oxide_camera_get_rotation(
    OxideCameraHandle camera,
    OxideQuaternion* result
);

OXIDE_API OxideStatus
oxide_camera_set_rotation(
    OxideCameraHandle camera,
    OxideQuaternion rotation
);

OXIDE_API OxideStatus
oxide_camera_set_euler(
    OxideCameraHandle camera,
    OxideAngle pitch,
    OxideAngle yaw,
    OxideAngle roll
);

OXIDE_API OxideStatus
oxide_camera_rotate_pitch_by(
    OxideCameraHandle camera,
    OxideAngle angle
);

OXIDE_API OxideStatus
oxide_camera_rotate_yaw_by(
    OxideCameraHandle camera,
    OxideAngle angle
);

OXIDE_API OxideStatus
oxide_camera_rotate_roll_by(
    OxideCameraHandle camera,
    OxideAngle angle
);

OXIDE_API OxideStatus
oxide_camera_get_field_of_view(
    OxideCameraHandle camera,
    OxideAngle* result
);

OXIDE_API OxideStatus
oxide_camera_set_field_of_view(
    OxideCameraHandle camera,
    OxideAngle field_of_view
);

OXIDE_API OxideStatus
oxide_camera_get_near_plane(
    OxideCameraHandle camera,
    float* result
);

OXIDE_API OxideStatus
oxide_camera_get_far_plane(
    OxideCameraHandle camera,
    float* result
);

OXIDE_API OxideStatus
oxide_camera_set_clip_planes(
    OxideCameraHandle camera,
    float near_plane,
    float far_plane
);


/* ============================================================
 * Graphics
 * ============================================================ */

OXIDE_API OxideStatus
oxide_graphics_full_rect(
    OxideGraphicsHandle graphics,
    OxideRect* result
);

OXIDE_API OxideStatus
oxide_graphics_bind_camera(
    OxideGraphicsHandle graphics,
    OxideCameraHandle camera,
    OxideRect destination
);

OXIDE_API OxideStatus
oxide_graphics_draw_mesh(
    OxideGraphicsHandle graphics,
    OxideMeshHandle mesh,
    OxideMaterialHandle material,
    OxideTransform transform
);

OXIDE_API OxideStatus
oxide_graphics_clear_lights(
    OxideGraphicsHandle graphics
);

OXIDE_API OxideStatus
oxide_graphics_add_point_light(
    OxideGraphicsHandle graphics,
    OxidePointLight light
);

OXIDE_API OxideStatus
oxide_graphics_add_spot_light(
    OxideGraphicsHandle graphics,
    OxideSpotLight light
);

OXIDE_API OxideStatus
oxide_graphics_add_directional_light(
    OxideGraphicsHandle graphics,
    OxideDirectionalLight light
);


/* ============================================================
 * Content manager
 * ============================================================ */

OXIDE_API OxideStatus
oxide_content_manager_create(
    OxideContentManagerHandle* result
);

OXIDE_API OxideStatus
oxide_content_manager_create_with_cache_directory(
    const char* path,
    OxideContentManagerHandle* result
);

OXIDE_API OxideStatus
oxide_content_manager_destroy(
    OxideContentManagerHandle manager
);

OXIDE_API OxideStatus
oxide_content_manager_cache_directory_length(
    OxideContentManagerHandle manager,
    size_t* result
);

OXIDE_API OxideStatus
oxide_content_manager_copy_cache_directory(
    OxideContentManagerHandle manager,
    uint8_t* destination,
    size_t capacity
);


/* ============================================================
 * Content loading
 * ============================================================ */

OXIDE_API OxideStatus
oxide_content_load_mesh(
    OxideContentManagerHandle manager,
    const char* path,
    OxideMeshHandle* result
);

OXIDE_API OxideStatus
oxide_content_load_mesh_sub(
    OxideContentManagerHandle manager,
    const char* path,
    const char* subobject,
    OxideMeshHandle* result
);

OXIDE_API OxideStatus
oxide_content_load_material(
    OxideContentManagerHandle manager,
    const char* path,
    OxideMaterialHandle* result
);

OXIDE_API OxideStatus
oxide_content_load_material_sub(
    OxideContentManagerHandle manager,
    const char* path,
    const char* subobject,
    OxideMaterialHandle* result
);

OXIDE_API OxideStatus
oxide_content_load_texture(
    OxideContentManagerHandle manager,
    const char* path,
    OxideTextureHandle* result
);

OXIDE_API OxideStatus
oxide_content_load_texture_sub(
    OxideContentManagerHandle manager,
    const char* path,
    const char* subobject,
    OxideTextureHandle* result
);


/* ============================================================
 * Mesh
 * ============================================================ */

OXIDE_API OxideStatus
oxide_mesh_create(
    const OxideVertex* vertices,
    size_t vertex_count,

    const uint32_t* indices,
    size_t index_count,

    OxideMeshHandle* result
);

OXIDE_API OxideStatus
oxide_mesh_destroy(
    OxideMeshHandle mesh
);

OXIDE_API OxideStatus
oxide_mesh_vertex_count(
    OxideMeshHandle mesh,
    size_t* result
);

OXIDE_API OxideStatus
oxide_mesh_index_count(
    OxideMeshHandle mesh,
    size_t* result
);

OXIDE_API OxideStatus
oxide_mesh_get_pivot(
    OxideMeshHandle mesh,
    OxideVec3* result
);

OXIDE_API OxideStatus
oxide_mesh_set_pivot(
    OxideMeshHandle mesh,
    OxideVec3 pivot
);

OXIDE_API OxideStatus
oxide_mesh_recalculate_pivot(
    OxideMeshHandle mesh
);

OXIDE_API OxideStatus
oxide_mesh_get_direction(
    OxideMeshHandle mesh,
    OxideVec3* result
);

OXIDE_API OxideStatus
oxide_mesh_set_direction(
    OxideMeshHandle mesh,
    OxideVec3 direction
);

OXIDE_API OxideStatus
oxide_mesh_get_reference(
    OxideMeshHandle mesh,
    OxideVec3* result
);

OXIDE_API OxideStatus
oxide_mesh_set_reference(
    OxideMeshHandle mesh,
    OxideVec3 reference
);

OXIDE_API OxideStatus
oxide_mesh_get_side(
    OxideMeshHandle mesh,
    OxideVec3* result
);

OXIDE_API OxideStatus
oxide_mesh_set_orientation(
    OxideMeshHandle mesh,
    OxideVec3 direction,
    OxideVec3 reference
);

OXIDE_API OxideStatus
oxide_mesh_copy_vertices(
    OxideMeshHandle mesh,
    OxideVertex* destination,
    size_t capacity
);

OXIDE_API OxideStatus
oxide_mesh_copy_indices(
    OxideMeshHandle mesh,
    uint32_t* destination,
    size_t capacity
);

OXIDE_API OxideStatus
oxide_mesh_set_vertices(
    OxideMeshHandle mesh,
    const OxideVertex* vertices,
    size_t count
);

OXIDE_API OxideStatus
oxide_mesh_set_indices(
    OxideMeshHandle mesh,
    const uint32_t* indices,
    size_t count
);


/* ============================================================
 * Texture
 * ============================================================ */

OXIDE_API OxideStatus
oxide_texture_destroy(
    OxideTextureHandle texture
);

OXIDE_API OxideStatus
oxide_texture_solid(
    OxideColor color,
    OxideTextureHandle* result
);

OXIDE_API OxideStatus
oxide_texture_width(
    OxideTextureHandle texture,
    uint32_t* result
);

OXIDE_API OxideStatus
oxide_texture_height(
    OxideTextureHandle texture,
    uint32_t* result
);


/* ============================================================
 * Material
 * ============================================================ */

OXIDE_API OxideStatus
oxide_material_destroy(
    OxideMaterialHandle material
);

OXIDE_API OxideStatus
oxide_material_create(
    OxideTextureHandle texture,
    OxideMaterialHandle* result
);

OXIDE_API OxideStatus
oxide_material_solid(
    OxideColor color,
    OxideMaterialHandle* result
);

OXIDE_API OxideStatus
oxide_material_base_texture(
    OxideMaterialHandle material,
    OxideTextureHandle* result
);


/* ============================================================
 * Attachment
 * ============================================================ */

OXIDE_API OxideStatus
oxide_transform_from_attachment(
    OxideMeshHandle parent,
    OxideTransform parent_transform,
    OxideVec3 parent_attach_point,

    OxideMeshHandle child,
    OxideVec3 child_attach_point,

    OxideVec3 direction,
    OxideAngle rotation,
    OxideVec3 scale,

    OxideTransform* result
);


/* ============================================================
 * Collision
 * ============================================================ */

OXIDE_API OxideStatus
oxide_collision_collides(
    OxideMeshHandle mesh_a,
    OxideTransform transform_a,

    OxideMeshHandle mesh_b,
    OxideTransform transform_b,

    uint8_t* result
);


#ifdef __cplusplus
}
#endif

#endif