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
    OxideVec4 tangent;
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
/** Runs the application loop using the supplied configuration and callbacks. */
oxide_run(
    OxideAppConfig config,
    OxideGameCallbacks callbacks
);



/* ============================================================
 * Application / window control
 * ============================================================ */

/** Requests that the running application exit. */
OXIDE_API OxideStatus oxide_app_exit(void);
OXIDE_API OxideStatus oxide_app_set_window_name(const char* name);
OXIDE_API OxideStatus oxide_app_set_window_logo(OxideTextureHandle texture);

OXIDE_API OxideStatus
/** Requests a new backbuffer size. */
oxide_app_set_preferred_backbuffer_size(
    uint32_t width,
    uint32_t height
);

OXIDE_API uint32_t oxide_app_window_width(void);
OXIDE_API uint32_t oxide_app_window_height(void);
OXIDE_API uint8_t oxide_app_is_window_focused(void);

OXIDE_API OxideStatus oxide_app_set_fullscreen(uint8_t fullscreen);
OXIDE_API uint8_t oxide_app_is_fullscreen(void);

OXIDE_API OxideStatus oxide_app_set_cursor_visible(uint8_t visible);
OXIDE_API OxideStatus oxide_app_set_cursor_grabbed(uint8_t grabbed);


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
/** Binds a camera and viewport for subsequent draw calls. */
oxide_graphics_bind_camera(
    OxideGraphicsHandle graphics,
    OxideCameraHandle camera,
    OxideRect destination
);

OXIDE_API OxideStatus
/** Draws a mesh with a material and transform. */
oxide_graphics_draw_mesh(
    OxideGraphicsHandle graphics,
    OxideMeshHandle mesh,
    OxideMaterialHandle material,
    OxideTransform transform
);

/** Sets explicit ambient lighting, IBL, and visible-background inputs. */
OXIDE_API OxideStatus oxide_graphics_set_environment_explicit(
    OxideGraphicsHandle graphics,
    OxideColor ambient_color,
    float ambient_intensity,
    OxideTextureHandle environment_texture,
    bool has_environment_texture,
    float environment_intensity,
    OxideTextureHandle background_texture,
    bool has_background_texture,
    OxideColor background_color
);

/** Removes the current environment. */
OXIDE_API OxideStatus oxide_graphics_clear_environment(
    OxideGraphicsHandle graphics
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
/** Creates a content manager using the default compiled-content cache. */
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
/** Loads a mesh through the content pipeline. */
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
/** Loads a material through the content pipeline. */
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
/** Loads a texture through the content pipeline. */
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

OXIDE_API OxideStatus oxide_texture_create_rgba(
    uint32_t width,
    uint32_t height,
    const uint8_t* pixels,
    size_t pixel_count,
    OxideTextureHandle* result
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


OXIDE_API OxideStatus oxide_material_normal_texture(OxideMaterialHandle material, OxideTextureHandle* result);
OXIDE_API OxideStatus oxide_material_set_normal_texture(OxideMaterialHandle material, OxideTextureHandle texture);
OXIDE_API OxideStatus oxide_material_get_normal_strength(OxideMaterialHandle material, float* result);
OXIDE_API OxideStatus oxide_material_set_normal_strength(OxideMaterialHandle material, float value);
OXIDE_API OxideStatus oxide_material_get_base_color(OxideMaterialHandle material, OxideColor* result);
OXIDE_API OxideStatus oxide_material_set_base_color(OxideMaterialHandle material, OxideColor value);
OXIDE_API OxideStatus oxide_material_get_metallic(OxideMaterialHandle material, float* result);
OXIDE_API OxideStatus oxide_material_set_metallic(OxideMaterialHandle material, float value);
OXIDE_API OxideStatus oxide_material_get_roughness(OxideMaterialHandle material, float* result);
OXIDE_API OxideStatus oxide_material_set_roughness(OxideMaterialHandle material, float value);
OXIDE_API OxideStatus oxide_material_get_emissive_color(OxideMaterialHandle material, OxideColor* result);
OXIDE_API OxideStatus oxide_material_set_emissive_color(OxideMaterialHandle material, OxideColor value);
OXIDE_API OxideStatus oxide_material_get_emissive_strength(OxideMaterialHandle material, float* result);
OXIDE_API OxideStatus oxide_material_set_emissive_strength(OxideMaterialHandle material, float value);

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
/** Tests whether two transformed meshes intersect. */
oxide_collision_collides(
    OxideMeshHandle mesh_a,
    OxideTransform transform_a,

    OxideMeshHandle mesh_b,
    OxideTransform transform_b,

    uint8_t* result
);



/* ============================================================
 * Input
 * ============================================================ */

typedef enum OxideKey
{
    OXIDE_KEY_A = 0, OXIDE_KEY_B, OXIDE_KEY_C, OXIDE_KEY_D,
    OXIDE_KEY_E, OXIDE_KEY_F, OXIDE_KEY_G, OXIDE_KEY_H,
    OXIDE_KEY_I, OXIDE_KEY_J, OXIDE_KEY_K, OXIDE_KEY_L,
    OXIDE_KEY_M, OXIDE_KEY_N, OXIDE_KEY_O, OXIDE_KEY_P,
    OXIDE_KEY_Q, OXIDE_KEY_R, OXIDE_KEY_S, OXIDE_KEY_T,
    OXIDE_KEY_U, OXIDE_KEY_V, OXIDE_KEY_W, OXIDE_KEY_X,
    OXIDE_KEY_Y, OXIDE_KEY_Z,
    OXIDE_KEY_0, OXIDE_KEY_1, OXIDE_KEY_2, OXIDE_KEY_3,
    OXIDE_KEY_4, OXIDE_KEY_5, OXIDE_KEY_6, OXIDE_KEY_7,
    OXIDE_KEY_8, OXIDE_KEY_9,
    OXIDE_KEY_ESCAPE, OXIDE_KEY_SPACE, OXIDE_KEY_ENTER,
    OXIDE_KEY_TAB, OXIDE_KEY_BACKSPACE,
    OXIDE_KEY_LEFT_SHIFT, OXIDE_KEY_RIGHT_SHIFT,
    OXIDE_KEY_LEFT_CONTROL, OXIDE_KEY_RIGHT_CONTROL,
    OXIDE_KEY_LEFT_ALT, OXIDE_KEY_RIGHT_ALT,
    OXIDE_KEY_UP, OXIDE_KEY_DOWN, OXIDE_KEY_LEFT, OXIDE_KEY_RIGHT,
    OXIDE_KEY_INSERT, OXIDE_KEY_DELETE, OXIDE_KEY_HOME, OXIDE_KEY_END,
    OXIDE_KEY_PAGE_UP, OXIDE_KEY_PAGE_DOWN,
    OXIDE_KEY_F1, OXIDE_KEY_F2, OXIDE_KEY_F3, OXIDE_KEY_F4,
    OXIDE_KEY_F5, OXIDE_KEY_F6, OXIDE_KEY_F7, OXIDE_KEY_F8,
    OXIDE_KEY_F9, OXIDE_KEY_F10, OXIDE_KEY_F11, OXIDE_KEY_F12
} OxideKey;

typedef enum OxideMouseButton
{
    OXIDE_MOUSE_BUTTON_LEFT = 0,
    OXIDE_MOUSE_BUTTON_RIGHT = 1,
    OXIDE_MOUSE_BUTTON_MIDDLE = 2,
    OXIDE_MOUSE_BUTTON_BACK = 3,
    OXIDE_MOUSE_BUTTON_FORWARD = 4
} OxideMouseButton;

typedef struct OxideKeyboardState
{
    uint64_t current[2];
    uint64_t previous[2];
} OxideKeyboardState;

typedef struct OxideMouseState
{
    uint64_t current_buttons;
    uint64_t previous_buttons;
    OxideVec2 position;
    OxideVec2 delta;
    OxideVec2 scroll_delta;
} OxideMouseState;

OXIDE_API OxideStatus
/** Copies the current and previous keyboard state for level and edge queries. */
oxide_keyboard_get_state(
    OxideKeyboardState* result
);

OXIDE_API OxideStatus
oxide_mouse_get_state(
    OxideMouseState* result
);


#ifdef __cplusplus
}
#endif

#endif