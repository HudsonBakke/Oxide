#ifndef OXIDE_H
#define OXIDE_H

/**
 * @file oxide.h
 * @brief C interface to the Oxide engine.
 *
 * Use a 64-bit Windows process and link the import library alongside oxide.dll.
 * All strings are NUL-terminated UTF-8 unless an explicit byte count is given.
 * Status-returning calls report failures through oxide_last_error_message().
 * Output pointers must be valid and writable; inspect outputs only on success.
 * Array pointers must reference the declared number of elements.
 * Resource handles are owned: release each successful create/load result once.
 * Graphics handles are borrowed and valid only inside the current draw callback.
 */

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>



#ifdef _WIN32
/** Import visibility for declarations supplied by oxide.dll. */
#define OXIDE_API __declspec(dllimport)
#else
/** Import visibility for declarations supplied by oxide.dll. */
#define OXIDE_API
#endif

#ifdef __cplusplus
extern "C"
{
#endif

    /* ============================================================
 * Status / errors
 * ============================================================ */

    /** Native operation status; zero indicates success and nonzero values identify failure categories. */
    typedef enum OxideStatus
    {
        /** Operation succeeded. */
        OXIDE_STATUS_OK = 0,
        /** Invalid argument or handle. */
        OXIDE_STATUS_INVALID_ARGUMENT = 1,
        /** Invalid operation for the current state. */
        OXIDE_STATUS_INVALID_OPERATION = 2,
        /** Content import or I/O failure. */
        OXIDE_STATUS_CONTENT_ERROR = 3,
        /** Graphics initialization or rendering failure. */
        OXIDE_STATUS_GRAPHICS_ERROR = 4,
        /** Unexpected internal failure. */
        OXIDE_STATUS_INTERNAL_ERROR = 5
    } OxideStatus;

    /**
 * @brief Returns the last diagnostic for the calling thread. The borrowed UTF-8 string is valid until the next error on that thread; do not free it.
 */
    OXIDE_API const char *oxide_last_error_message(void);

    /* ============================================================
 * Math types
 * ============================================================ */

    /** Angle stored in radians; construct from radians or degrees explicitly. */
    typedef struct OxideAngle
    {
        /** Angle storage measured in radians. */
        float radians;
    } OxideAngle;

    /** Two-component floating-point vector with X and Y coordinates. */
    typedef struct OxideVec2
    {
        /** X component. */
        float x;
        /** Y component. */
        float y;
    } OxideVec2;

    /** Three-component floating-point vector for positions, directions, and scales. */
    typedef struct OxideVec3
    {
        /** X component. */
        float x;
        /** Y component. */
        float y;
        /** Z component. */
        float z;
    } OxideVec3;

    /** Four-component floating-point vector; W distinguishes homogeneous positions from directions. */
    typedef struct OxideVec4
    {
        /** X component. */
        float x;
        /** Y component. */
        float y;
        /** Z component. */
        float z;
        /** W component. */
        float w;
    } OxideVec4;

    /** Rotation quaternion with XYZW components; normalize manually constructed values before rotating vectors. */
    typedef struct OxideQuaternion
    {
        /** X component. */
        float x;
        /** Y component. */
        float y;
        /** Z component. */
        float z;
        /** W component. */
        float w;
    } OxideQuaternion;

    /** Pixel rectangle with top-left X/Y coordinates and width/height extents. */
    typedef struct OxideRect
    {
        /** Left edge in physical pixels. */
        float x;
        /** Top edge in physical pixels. */
        float y;
        /** Width in physical pixels. */
        float width;
        /** Height in physical pixels. */
        float height;
    } OxideRect;

    /** World-space position, quaternion orientation, and component-wise local scale. A default constructed transform has unit scale. */
    typedef struct OxideTransform
    {
        /** World-space light position. */
        OxideVec3 position;
        /** Normalized orientation quaternion. */
        OxideQuaternion rotation;
        /** Component-wise scale along the model's local axes. */
        OxideVec3 scale;
    } OxideTransform;

    /* ============================================================
 * Graphics value types
 * ============================================================ */

    /** Floating-point RGBA color; values are stored without clamping. */
    typedef struct OxideColor
    {
        /** Red component. */
        float r;
        /** Green component. */
        float g;
        /** Blue component. */
        float b;
        /** Alpha component: zero is transparent and one is opaque. */
        float a;
    } OxideColor;

    /** Model-space vertex with position, normal, UV coordinates, and tangent XYZ plus handedness W. */
    typedef struct OxideVertex
    {
        /** Model-space vertex position. */
        OxideVec3 position;
        /** Model-space surface normal used for lighting. */
        OxideVec3 normal;
        /** Texture UV coordinates; (0, 0) addresses the top-left corner. */
        OxideVec2 tex_coord;
        /** Model-space tangent XYZ and bitangent handedness W (normally +1 or -1). */
        OxideVec4 tangent;
    } OxideVertex;

    /** Omnidirectional point light with inverse-square distance attenuation. */
    typedef struct OxidePointLight
    {
        /** World-space light position. */
        OxideVec3 position;
        /** RGB light color; alpha is not used for lighting. */
        OxideColor color;
        /** Non-negative light intensity; point and spot lights attenuate with squared distance. */
        float intensity;
    } OxidePointLight;

    /** Point light restricted to a directional cone, with inverse-square distance attenuation. */
    typedef struct OxideSpotLight
    {
        /** World-space light position. */
        OxideVec3 position;
        /** World-space light-ray direction; use a nonzero vector. */
        OxideVec3 direction;
        /** Spotlight cone half-angle measured from its direction; use an angle strictly between zero and pi radians. */
        OxideAngle angle;
        /** RGB light color; alpha is not used for lighting. */
        OxideColor color;
        /** Non-negative light intensity; point and spot lights attenuate with squared distance. */
        float intensity;
    } OxideSpotLight;

    /** Light whose rays travel in one world-space direction, with no distance attenuation. */
    typedef struct OxideDirectionalLight
    {
        /** World-space light-ray direction; use a nonzero vector. */
        OxideVec3 direction;
        /** RGB light color; alpha is not used for lighting. */
        OxideColor color;
        /** Non-negative light intensity; point and spot lights attenuate with squared distance. */
        float intensity;
    } OxideDirectionalLight;

    /** Orthographic directional shadow volume and receiver bias. Initialize with oxide_directional_shadow_settings_default. */
    typedef struct OxideDirectionalShadowSettings
    {
        /** Finite world-space volume center; default origin. */
        OxideVec3 center;
        /** Finite positive full square width and height in world units; default 30. */
        float size;
        /** Finite positive full depth along light rays, centered on center; default 60. */
        float depth;
        /** Power-of-two square resolution from 128 through 4096; default 1024. */
        uint32_t resolution;
        /** Finite non-negative comparison offset in normalized depth units; default 0.0001. */
        float depth_bias;
        /** Finite non-negative receiver offset along the geometric normal in world units; default 0.01. */
        float normal_bias;
    } OxideDirectionalShadowSettings;

    /* ============================================================
 * Handles
 * ============================================================ */

    /** Borrowed draw-callback token; never dereference, retain beyond the callback, or destroy it. */
    typedef struct OxideGraphicsHandle
    {
        /** Opaque callback token; do not dereference. */
        void *ptr;
    } OxideGraphicsHandle;

/**
 * @brief Reads the persistent linear exposure multiplier (default one).
 * @param graphics Borrowed handle valid during the current draw callback.
 * @param[out] result Non-NULL output pointer.
 * @return OXIDE_STATUS_OK on success, or an error with a thread-local diagnostic.
 */
OXIDE_API OxideStatus oxide_graphics_get_exposure(OxideGraphicsHandle graphics, float* result);

/**
 * @brief Sets finite, non-negative linear exposure; zero produces black.
 * The last value set in a draw callback applies to the entire final frame,
 * including every viewport and background, and persists across frames.
 * @param graphics Borrowed handle valid during the current draw callback.
 * @param exposure Linear exposure multiplier; must be finite and non-negative.
 * @return OXIDE_STATUS_OK on success, or an error with a thread-local diagnostic.
 */
OXIDE_API OxideStatus oxide_graphics_set_exposure(OxideGraphicsHandle graphics, float exposure);


    /** Opaque owned Camera resource handle. Copying the number does not transfer or duplicate ownership. */
    typedef struct OxideCameraHandle
    {
        /** Opaque registry identifier; do not inspect or modify. */
        uint64_t value;
    } OxideCameraHandle;

    /** Opaque owned ContentManager resource handle. Copying the number does not transfer or duplicate ownership. */
    typedef struct OxideContentManagerHandle
    {
        /** Opaque registry identifier; do not inspect or modify. */
        uint64_t value;
    } OxideContentManagerHandle;

    /** Opaque owned Mesh resource handle. Copying the number does not transfer or duplicate ownership. */
    typedef struct OxideMeshHandle
    {
        /** Opaque registry identifier; do not inspect or modify. */
        uint64_t value;
    } OxideMeshHandle;

    /** Opaque owned Material resource handle. Copying the number does not transfer or duplicate ownership. */
    typedef struct OxideMaterialHandle
    {
        /** Opaque registry identifier; do not inspect or modify. */
        uint64_t value;
    } OxideMaterialHandle;

    /** Opaque owned Texture resource handle. Copying the number does not transfer or duplicate ownership. */
    typedef struct OxideTextureHandle
    {
        /** Opaque registry identifier; do not inspect or modify. */
        uint64_t value;
    } OxideTextureHandle;

    /* ============================================================
 * Application
 * ============================================================ */

    /** Frame timing in seconds; delta time varies with actual frame duration. */
    typedef struct OxideGameTime
    {
        /** Seconds elapsed since the previous frame's update. */
        double delta_seconds;
        /** Seconds elapsed since the application clock started. */
        double total_seconds;
    } OxideGameTime;

    /** Initial window title and logical-pixel client size; defaults to Oxide at 1280 by 720. */
    typedef struct OxideAppConfig
    {
        /** NUL-terminated UTF-8 initial window title. */
        const char *title;
        /** Initial logical-pixel width. */
        uint32_t width;
        /** Initial logical-pixel height. */
        uint32_t height;
    } OxideAppConfig;

    /** Called once after the window and graphics are ready; load persistent resources here. Never unwind exceptions across this callback boundary. */
    typedef void (*OxideInitializeCallback)(void *user_data);

    /** Called once before drawing each frame with elapsed time in seconds; update game logic and read input here. Never unwind exceptions across this callback boundary. */
    typedef void (*OxideUpdateCallback)(void *user_data, OxideGameTime time);

    /** Called each frame to bind cameras, set lighting, and submit draw calls. The graphics token expires when this function returns. Never unwind exceptions across this callback boundary. */
    typedef void (*OxideDrawCallback)(void *user_data, OxideGraphicsHandle graphics);

    /** Lifecycle callbacks and caller-owned state; callback pointers may be NULL. */
    typedef struct OxideGameCallbacks
    {
        /** Caller-owned state passed unchanged to every callback. */
        void *user_data;

        /** Optional initialization callback invoked after window and graphics setup. */
        OxideInitializeCallback initialize;
        /** Optional update callback invoked before drawing each frame. */
        OxideUpdateCallback update;
        /** Optional drawing callback; its graphics token expires on return. */
        OxideDrawCallback draw;
    } OxideGameCallbacks;

    /**
 * @brief Runs the blocking event loop, invoking initialize once and update/draw each frame. Callback function pointers and user_data must remain valid until return.
 * @param config Window configuration; title is a NUL-terminated UTF-8 string.
 * @param callbacks Callback table; a NULL callback is skipped.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_run(OxideAppConfig config, OxideGameCallbacks callbacks);

    /* ============================================================
 * Application / window control
 * ============================================================ */

    /**
 * @brief Queues a request to leave the application event loop.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_exit(void);
    /**
 * @brief Queues a change to the native window title.
 * @param name NUL-terminated UTF-8 window title.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_set_window_name(const char *name);
    /**
 * @brief Queues an icon change using an RGBA8 texture.
 * @param texture Live texture handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_set_window_logo(OxideTextureHandle texture);

    /**
 * @brief Requests a client/backbuffer size in physical pixels; both dimensions must be positive.
 * @param width Width in pixels.
 * @param height Height in pixels.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_set_preferred_backbuffer_size(uint32_t width, uint32_t height);

    OXIDE_API uint32_t oxide_app_window_width(void);
    OXIDE_API uint32_t oxide_app_window_height(void);
    OXIDE_API uint8_t oxide_app_is_window_focused(void);

    /**
 * @brief Requests borderless fullscreen on the current monitor, or restores windowed mode.
 * @param fullscreen Whether to request borderless fullscreen.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_set_fullscreen(uint8_t fullscreen);
    OXIDE_API uint8_t oxide_app_is_fullscreen(void);

    /**
 * @brief Queues a change to the visibility of the OS cursor over the game window.
 * @param visible Whether the OS cursor should be visible.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_set_cursor_visible(uint8_t visible);
    /**
 * @brief Requests cursor locking, falling back to confinement when locking is unavailable.
 * @param grabbed Whether to lock or confine the OS cursor.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_app_set_cursor_grabbed(uint8_t grabbed);

    /* ============================================================
 * Camera
 * ============================================================ */

    /**
 * @brief Creates a camera at the origin with identity rotation, a 60-degree vertical field of view, and clipping distances 0.1 and 1000. Writes a new owned handle on success.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_create(OxideCameraHandle *result);

    /**
 * @brief Releases this owned Camera handle; do not use it after a successful call.
 * @param camera Live camera handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_destroy(OxideCameraHandle camera);

    /**
 * @brief Position in world space.
 * @param camera Live camera handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_get_position(OxideCameraHandle camera, OxideVec3 *result);

    /**
 * @brief Sets the position in world space.
 * @param camera Live camera handle.
 * @param position Position in world space.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_set_position(OxideCameraHandle camera, OxideVec3 position);

    /**
 * @brief Adds a world-space translation without rotating it by the current orientation.
 * @param camera Live camera handle.
 * @param translation World-space displacement.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_translate_by(OxideCameraHandle camera,
                                                    OxideVec3 translation);

    /**
 * @brief Orientation quaternion; assigned values are normalized.
 * @param camera Live camera handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_get_rotation(OxideCameraHandle camera,
                                                    OxideQuaternion *result);

    /**
 * @brief Sets the orientation to a normalized copy of the quaternion.
 * @param camera Live camera handle.
 * @param rotation Additional twist about direction.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_set_rotation(OxideCameraHandle camera,
                                                    OxideQuaternion rotation);

    /**
 * @brief Sets orientation to yaw * pitch * roll, about Y, X, and Z respectively.
 * @param camera Live camera handle.
 * @param pitch Rotation angle about X.
 * @param yaw Rotation angle about Y.
 * @param roll Rotation angle about Z.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_set_euler(OxideCameraHandle camera, OxideAngle pitch,
                                                 OxideAngle yaw, OxideAngle roll);

    /**
 * @brief Applies an additional rotation about the world X axis.
 * @param camera Live camera handle.
 * @param angle Spotlight cone half-angle measured from its direction; use an angle strictly between zero and pi radians.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_rotate_pitch_by(OxideCameraHandle camera, OxideAngle angle);

    /**
 * @brief Applies an additional rotation about the world Y axis.
 * @param camera Live camera handle.
 * @param angle Spotlight cone half-angle measured from its direction; use an angle strictly between zero and pi radians.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_rotate_yaw_by(OxideCameraHandle camera, OxideAngle angle);

    /**
 * @brief Applies an additional rotation about the world Z axis.
 * @param camera Live camera handle.
 * @param angle Spotlight cone half-angle measured from its direction; use an angle strictly between zero and pi radians.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_rotate_roll_by(OxideCameraHandle camera, OxideAngle angle);

    /**
 * @brief Vertical perspective field of view; use an angle strictly between zero and pi radians.
 * @param camera Live camera handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_get_field_of_view(OxideCameraHandle camera,
                                                         OxideAngle *result);

    /**
 * @brief Sets the vertical field of view; returns an error outside the open interval (0, pi) radians.
 * @param camera Live camera handle.
 * @param field_of_view Vertical field of view, strictly between zero and pi radians.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_set_field_of_view(OxideCameraHandle camera,
                                                         OxideAngle field_of_view);

    /**
 * @brief Distance from the camera to the near clipping plane in world units.
 * @param camera Live camera handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_get_near_plane(OxideCameraHandle camera, float *result);

    /**
 * @brief Distance from the camera to the far clipping plane in world units.
 * @param camera Live camera handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_get_far_plane(OxideCameraHandle camera, float *result);

    /**
 * @brief Sets clipping distances in world units; requires 0 < near < far.
 * @param camera Live camera handle.
 * @param near_plane Near clipping distance; must be greater than zero.
 * @param far_plane Far clipping distance; must be greater than near_plane.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_camera_set_clip_planes(OxideCameraHandle camera, float near_plane,
                                                       float far_plane);

    /* ============================================================
 * Graphics
 * ============================================================ */

    /**
 * @brief Rectangle covering the entire render surface in physical pixels.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_full_rect(OxideGraphicsHandle graphics, OxideRect *result);

    /**
 * @brief Binds a camera and a positive-size pixel viewport for subsequent draw calls.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param camera Live camera handle.
 * @param destination Caller-owned output buffer, or the positive-size pixel viewport for camera binding.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_bind_camera(OxideGraphicsHandle graphics,
                                                     OxideCameraHandle camera,
                                                     OxideRect destination);

    /** Initializes shadow settings to documented defaults; result must not be null. */
    OXIDE_API OxideStatus oxide_directional_shadow_settings_default(OxideDirectionalShadowSettings *result);

    /** Configures automatic directional shadows before any mesh submission; persists across frames. Add the same unchanged light for scene draws. */
    OXIDE_API OxideStatus oxide_graphics_set_directional_shadows(OxideGraphicsHandle graphics,
                                                              OxideDirectionalLight light,
                                                              OxideDirectionalShadowSettings settings);

    /** Disables automatic shadows; call before any mesh submission in this frame. */
    OXIDE_API OxideStatus oxide_graphics_clear_directional_shadows(OxideGraphicsHandle graphics);

    /** Submits geometry with finite shadow transparency in [0,1]: zero blocks light, one casts no shadow. Does not change visible material opacity. State is captured now; rendering occurs at frame end. Up to eight transparent depths are retained per ray. */
    OXIDE_API OxideStatus oxide_graphics_draw_mesh_with_transparency(OxideGraphicsHandle graphics,
        OxideMeshHandle mesh, OxideMaterialHandle material, OxideTransform transform, float transparency);

    /**
 * @brief Submits a mesh with opaque shadows and captures its material, transform, lighting, and camera state for rendering at frame end.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param mesh Live mesh handle.
 * @param material Live material handle.
 * @param transform World-space placement, applied around the mesh's model-space pivot.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_draw_mesh(OxideGraphicsHandle graphics,
                                                   OxideMeshHandle mesh,
                                                   OxideMaterialHandle material,
                                                   OxideTransform transform);

    /**
 * @brief Sets independent ambient, image-based lighting, and visible background inputs. Intensity multipliers must be finite and non-negative.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param ambient_color RGB color of constant ambient lighting.
 * @param ambient_intensity Finite, non-negative multiplier for constant ambient lighting.
 * @param environment_texture Lighting texture handle; ignored when has_environment_texture is false.
 * @param has_environment_texture Whether environment_texture is supplied.
 * @param environment_intensity Finite, non-negative multiplier for image-based lighting.
 * @param background_texture Visible background handle; ignored when has_background_texture is false.
 * @param has_background_texture Whether background_texture is supplied.
 * @param background_color Clear color used when no background texture is displayed.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_set_environment_explicit(
        OxideGraphicsHandle graphics, OxideColor ambient_color, float ambient_intensity,
        OxideTextureHandle environment_texture, bool has_environment_texture,
        float environment_intensity, OxideTextureHandle background_texture,
        bool has_background_texture, OxideColor background_color);

    /**
 * @brief Removes environment lighting and its background settings.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_clear_environment(OxideGraphicsHandle graphics);

    /**
 * @brief Removes lights added so far in the current frame.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_clear_lights(OxideGraphicsHandle graphics);

    /**
 * @brief Adds a point, spot, or directional light for subsequent draws; lights reset at frame start.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param light Light value; position and direction use world coordinates.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_add_point_light(OxideGraphicsHandle graphics,
                                                         OxidePointLight light);

    /**
 * @brief Adds a point, spot, or directional light for subsequent draws; lights reset at frame start.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param light Light value; position and direction use world coordinates.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_add_spot_light(OxideGraphicsHandle graphics,
                                                        OxideSpotLight light);

    /**
 * @brief Adds a point, spot, or directional light for subsequent draws; lights reset at frame start.
 * @param graphics Borrowed rendering handle, valid only during the current draw callback.
 * @param light Light value; position and direction use world coordinates.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_graphics_add_directional_light(OxideGraphicsHandle graphics,
                                                               OxideDirectionalLight light);

    /* ============================================================
 * Content manager
 * ============================================================ */

    /**
 * @brief Creates a loader using the default target/oxide-content cache, or a supplied cache directory. Writes a new owned handle on success.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_manager_create(OxideContentManagerHandle *result);

    /**
 * @brief Creates an owned content manager with a caller-selected cache directory.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_manager_create_with_cache_directory(
        const char *path, OxideContentManagerHandle *result);

    /**
 * @brief Releases this owned ContentManager handle; do not use it after a successful call.
 * @param manager Live content-manager handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_manager_destroy(OxideContentManagerHandle manager);

    /**
 * @brief Writes the UTF-8 cache-directory byte length, excluding its terminating NUL.
 * @param manager Live content-manager handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus
    oxide_content_manager_cache_directory_length(OxideContentManagerHandle manager, size_t *result);

    /**
 * @brief Copies the UTF-8 cache directory including NUL; capacity must be at least the reported byte length plus one.
 * @param manager Live content-manager handle.
 * @param destination Caller-owned output buffer, or the positive-size pixel viewport for camera binding.
 * @param capacity Number of elements available in destination, measured in the element type rather than bytes.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_manager_copy_cache_directory(
        OxideContentManagerHandle manager, uint8_t *destination, size_t capacity);

    /* ============================================================
 * Content loading
 * ============================================================ */

    /**
 * @brief Loads an owned mesh handle from a source file; multiple compatible glTF objects require a named-subobject load.
 * @param manager Live content-manager handle.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_load_mesh(OxideContentManagerHandle manager,
                                                  const char *path, OxideMeshHandle *result);

    /**
 * @brief Loads an owned mesh handle from a source file, selecting a unique, nonempty glTF node name.
 * @param manager Live content-manager handle.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param subobject Nonempty NUL-terminated UTF-8 glTF object name, unique within the selected asset category.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_load_mesh_sub(OxideContentManagerHandle manager,
                                                      const char *path, const char *subobject,
                                                      OxideMeshHandle *result);

    /**
 * @brief Loads an owned material handle from a source file; multiple compatible glTF objects require a named-subobject load.
 * @param manager Live content-manager handle.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_load_material(OxideContentManagerHandle manager,
                                                      const char *path,
                                                      OxideMaterialHandle *result);

    /**
 * @brief Loads an owned material handle from a source file, selecting a unique, nonempty glTF material name.
 * @param manager Live content-manager handle.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param subobject Nonempty NUL-terminated UTF-8 glTF object name, unique within the selected asset category.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_load_material_sub(OxideContentManagerHandle manager,
                                                          const char *path, const char *subobject,
                                                          OxideMaterialHandle *result);

    /**
 * @brief Loads an owned texture handle from a source file; multiple compatible glTF objects require a named-subobject load.
 * @param manager Live content-manager handle.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_load_texture(OxideContentManagerHandle manager,
                                                     const char *path, OxideTextureHandle *result);

    /**
 * @brief Loads an owned texture handle from a source file, selecting a unique, nonempty glTF texture name.
 * @param manager Live content-manager handle.
 * @param path NUL-terminated UTF-8 source or cache-directory path; relative paths use the working directory.
 * @param subobject Nonempty NUL-terminated UTF-8 glTF object name, unique within the selected asset category.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_content_load_texture_sub(OxideContentManagerHandle manager,
                                                         const char *path, const char *subobject,
                                                         OxideTextureHandle *result);

    /* ============================================================
 * Mesh
 * ============================================================ */

    /**
 * @brief Copies vertices and triangle indices into a new mesh and computes its pivot and tangent frame. Input arrays are copied.
 * @param vertices Array of model-space vertices; may be NULL only when the count is zero.
 * @param vertex_count Number of vertices in the input array.
 * @param indices Array of triangle vertex indices; may be NULL only when the count is zero.
 * @param index_count Number of indices in the input array; triangles use groups of three.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_create(const OxideVertex *vertices, size_t vertex_count,

                                            const uint32_t *indices, size_t index_count,

                                            OxideMeshHandle *result);

    /**
 * @brief Releases this owned Mesh handle; do not use it after a successful call.
 * @param mesh Live mesh handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_destroy(OxideMeshHandle mesh);

    /**
 * @brief Number of vertices in the mesh.
 * @param mesh Live mesh handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_vertex_count(OxideMeshHandle mesh, size_t *result);

    /**
 * @brief Number of vertex indices; each group of three describes a triangle.
 * @param mesh Live mesh handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_index_count(OxideMeshHandle mesh, size_t *result);

    /**
 * @brief Model-space point placed at the transform's position when drawing or attaching this mesh.
 * @param mesh Live mesh handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_get_pivot(OxideMeshHandle mesh, OxideVec3 *result);

    /**
 * @brief Sets the model-space point used as the mesh's transformation origin.
 * @param mesh Live mesh handle.
 * @param pivot Model-space point used as the mesh transformation origin.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_set_pivot(OxideMeshHandle mesh, OxideVec3 pivot);

    /**
 * @brief Sets the pivot to the arithmetic mean of all model-space vertex positions.
 * @param mesh Live mesh handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_recalculate_pivot(OxideMeshHandle mesh);

    /**
 * @brief Normalized forward direction of the mesh's model-space attachment frame.
 * @param mesh Live mesh handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_get_direction(OxideMeshHandle mesh, OxideVec3 *result);

    /**
 * @brief Normalizes the nonzero model-space direction and rebuilds the attachment frame.
 * @param mesh Live mesh handle.
 * @param direction Nonzero desired world-space child direction.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_set_direction(OxideMeshHandle mesh, OxideVec3 direction);

    /**
 * @brief Normalized model-space reference axis perpendicular to the attachment direction.
 * @param mesh Live mesh handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_get_reference(OxideMeshHandle mesh, OxideVec3 *result);

    /**
 * @brief Sets a nonzero reference axis and orthogonalizes it against the attachment direction.
 * @param mesh Live mesh handle.
 * @param reference Nonzero model-space reference axis; must not be parallel to mesh direction.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_set_reference(OxideMeshHandle mesh, OxideVec3 reference);

    /**
 * @brief Normalized side axis computed from direction cross reference.
 * @param mesh Live mesh handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_get_side(OxideMeshHandle mesh, OxideVec3 *result);

    /**
 * @brief Builds an orthonormal model-space attachment frame from nonzero direction and reference vectors.
 * @param mesh Live mesh handle.
 * @param direction Nonzero desired world-space child direction.
 * @param reference Nonzero model-space reference axis; must not be parallel to mesh direction.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_set_orientation(OxideMeshHandle mesh, OxideVec3 direction,
                                                     OxideVec3 reference);

    /**
 * @brief Copies all vertices into caller-owned storage; capacity is measured in vertices and must be at least the current vertex count.
 * @param mesh Live mesh handle.
 * @param destination Caller-owned output buffer, or the positive-size pixel viewport for camera binding.
 * @param capacity Number of elements available in destination, measured in the element type rather than bytes.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_copy_vertices(OxideMeshHandle mesh, OxideVertex *destination,
                                                   size_t capacity);

    /**
 * @brief Copies all indices into caller-owned storage; capacity is measured in indices and must be at least the current index count.
 * @param mesh Live mesh handle.
 * @param destination Caller-owned output buffer, or the positive-size pixel viewport for camera binding.
 * @param capacity Number of elements available in destination, measured in the element type rather than bytes.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_copy_indices(OxideMeshHandle mesh, uint32_t *destination,
                                                  size_t capacity);

    /**
 * @brief Copies replacement vertex data into the mesh; the vertex count must remain unchanged.
 * @param mesh Live mesh handle.
 * @param vertices Array of model-space vertices; may be NULL only when the count is zero.
 * @param count Number of input elements; must match the mesh's current element count.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_set_vertices(OxideMeshHandle mesh, const OxideVertex *vertices,
                                                  size_t count);

    /**
 * @brief Copies replacement triangle indices into the mesh; the index count must remain unchanged.
 * @param mesh Live mesh handle.
 * @param indices Array of triangle vertex indices; may be NULL only when the count is zero.
 * @param count Number of input elements; must match the mesh's current element count.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mesh_set_indices(OxideMeshHandle mesh, const uint32_t *indices,
                                                 size_t count);

    /* ============================================================
 * Texture
 * ============================================================ */

    /**
 * @brief Releases this owned Texture handle; do not use it after a successful call.
 * @param texture Live texture handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_texture_destroy(OxideTextureHandle texture);

    /**
 * @brief Creates a texture from row-major RGBA8 bytes; requires exactly width * height * 4 bytes. Pixel data is copied.
 * @param width Width in pixels.
 * @param height Height in pixels.
 * @param pixels Row-major RGBA8 bytes, copied by the call; may be NULL only for an empty input.
 * @param pixel_count Number of input bytes; must equal width * height * 4.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_texture_create_rgba(uint32_t width, uint32_t height,
                                                    const uint8_t *pixels, size_t pixel_count,
                                                    OxideTextureHandle *result);

    /**
 * @brief Creates an opaque or translucent 1x1 RGBA8 texture, clamping color channels to [0, 1].
 * @param color Floating-point RGBA color.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_texture_solid(OxideColor color, OxideTextureHandle *result);

    /**
 * @brief Texture width in pixels.
 * @param texture Live texture handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_texture_width(OxideTextureHandle texture, uint32_t *result);

    /**
 * @brief Texture height in pixels.
 * @param texture Live texture handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_texture_height(OxideTextureHandle texture, uint32_t *result);

    /* ============================================================
 * Material
 * ============================================================ */

    /**
 * @brief Creates an owned texture handle referring to the material's immutable normal map; destroy this handle independently.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_normal_texture(OxideMaterialHandle material,
                                                        OxideTextureHandle *result);
    /**
 * @brief Sets tangent-space normal map; RGB encodes a direction from -1 to 1.
 * @param material Live material handle.
 * @param texture Live texture handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_normal_texture(OxideMaterialHandle material,
                                                            OxideTextureHandle texture);
    /**
 * @brief Finite multiplier applied to the normal map's XY components; zero disables perturbation.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_get_normal_strength(OxideMaterialHandle material,
                                                             float *result);
    /**
 * @brief Sets finite multiplier applied to the normal map's XY components; zero disables perturbation.
 * @param material Live material handle.
 * @param value Replacement material property; see the range and units in this function's description.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_normal_strength(OxideMaterialHandle material,
                                                             float value);
    /**
 * @brief RGBA multiplier applied to the base-color texture.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_get_base_color(OxideMaterialHandle material,
                                                        OxideColor *result);
    /**
 * @brief Sets rGBA multiplier applied to the base-color texture.
 * @param material Live material handle.
 * @param value Replacement material property; see the range and units in this function's description.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_base_color(OxideMaterialHandle material,
                                                        OxideColor value);
    /**
 * @brief Metallic factor in [0, 1]: zero is dielectric and one is metallic.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_get_metallic(OxideMaterialHandle material, float *result);
    /**
 * @brief Sets metallic factor in [0, 1]: zero is dielectric and one is metallic.
 * @param material Live material handle.
 * @param value Replacement material property; see the range and units in this function's description.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_metallic(OxideMaterialHandle material, float value);
    /**
 * @brief Perceptual roughness in [0, 1]: zero is smooth and one is fully rough.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_get_roughness(OxideMaterialHandle material, float *result);
    /**
 * @brief Sets perceptual roughness in [0, 1]: zero is smooth and one is fully rough.
 * @param material Live material handle.
 * @param value Replacement material property; see the range and units in this function's description.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_roughness(OxideMaterialHandle material, float value);
    /**
 * @brief RGB color emitted by the surface, multiplied by emissive strength.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_get_emissive_color(OxideMaterialHandle material,
                                                            OxideColor *result);
    /**
 * @brief Sets rGB color emitted by the surface, multiplied by emissive strength.
 * @param material Live material handle.
 * @param value Replacement material property; see the range and units in this function's description.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_emissive_color(OxideMaterialHandle material,
                                                            OxideColor value);
    /**
 * @brief Finite, non-negative multiplier applied to the emissive color.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_get_emissive_strength(OxideMaterialHandle material,
                                                               float *result);
    /**
 * @brief Sets finite, non-negative multiplier applied to the emissive color.
 * @param material Live material handle.
 * @param value Replacement material property; see the range and units in this function's description.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_set_emissive_strength(OxideMaterialHandle material,
                                                               float value);

    /**
 * @brief Releases this owned Material handle; do not use it after a successful call.
 * @param material Live material handle.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_destroy(OxideMaterialHandle material);

    /**
 * @brief Creates a material using a base-color texture, metallic factor zero, roughness one, and no emission. Writes a new owned handle on success.
 * @param texture Live texture handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_create(OxideTextureHandle texture,
                                                OxideMaterialHandle *result);

    /**
 * @brief Creates an untextured-looking material from a solid-color base texture.
 * @param color Floating-point RGBA color.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_solid(OxideColor color, OxideMaterialHandle *result);

    /**
 * @brief Creates an owned texture handle referring to the material's immutable base image; destroy this handle independently.
 * @param material Live material handle.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_material_base_texture(OxideMaterialHandle material,
                                                      OxideTextureHandle *result);

    /* ============================================================
 * Attachment
 * ============================================================ */

    /**
 * @brief Returns a child transform aligning model-space attachment points using a world-space direction, twist angle, and child scale.
 * @param parent Live parent mesh handle.
 * @param parent_transform World-space placement of the parent mesh.
 * @param parent_attach_point Attachment position in the parent's model coordinates.
 * @param child Live child mesh handle.
 * @param child_attach_point Attachment position in the child's model coordinates.
 * @param direction Nonzero desired world-space child direction.
 * @param rotation Additional twist about direction.
 * @param scale Component-wise child scale.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_transform_from_attachment(
        OxideMeshHandle parent, OxideTransform parent_transform, OxideVec3 parent_attach_point,

        OxideMeshHandle child, OxideVec3 child_attach_point,

        OxideVec3 direction, OxideAngle rotation, OxideVec3 scale,

        OxideTransform *result);

    /* ============================================================
 * Collision
 * ============================================================ */

    /**
 * @brief Tests intersection of two transformed triangle meshes, including containment, using their model-space pivots.
 * @param mesh_a First live mesh handle.
 * @param transform_a First mesh's world-space placement.
 * @param mesh_b Second live mesh handle.
 * @param transform_b Second mesh's world-space placement.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_collision_collides(OxideMeshHandle mesh_a,
                                                   OxideTransform transform_a,

                                                   OxideMeshHandle mesh_b,
                                                   OxideTransform transform_b,

                                                   uint8_t *result);

    /* ============================================================
 * Input
 * ============================================================ */

    /** Physical keyboard keys, independent of text input and keyboard layout. */
    typedef enum OxideKey
    {

        /** Physical a key. */
        OXIDE_KEY_A = 0,
        /** Physical b key. */
        OXIDE_KEY_B,
        /** Physical c key. */
        OXIDE_KEY_C,
        /** Physical d key. */
        OXIDE_KEY_D,

        /** Physical e key. */
        OXIDE_KEY_E,
        /** Physical f key. */
        OXIDE_KEY_F,
        /** Physical g key. */
        OXIDE_KEY_G,
        /** Physical h key. */
        OXIDE_KEY_H,

        /** Physical i key. */
        OXIDE_KEY_I,
        /** Physical j key. */
        OXIDE_KEY_J,
        /** Physical k key. */
        OXIDE_KEY_K,
        /** Physical l key. */
        OXIDE_KEY_L,

        /** Physical m key. */
        OXIDE_KEY_M,
        /** Physical n key. */
        OXIDE_KEY_N,
        /** Physical o key. */
        OXIDE_KEY_O,
        /** Physical p key. */
        OXIDE_KEY_P,

        /** Physical q key. */
        OXIDE_KEY_Q,
        /** Physical r key. */
        OXIDE_KEY_R,
        /** Physical s key. */
        OXIDE_KEY_S,
        /** Physical t key. */
        OXIDE_KEY_T,

        /** Physical u key. */
        OXIDE_KEY_U,
        /** Physical v key. */
        OXIDE_KEY_V,
        /** Physical w key. */
        OXIDE_KEY_W,
        /** Physical x key. */
        OXIDE_KEY_X,

        /** Physical y key. */
        OXIDE_KEY_Y,
        /** Physical z key. */
        OXIDE_KEY_Z,

        /** Top-row digit 0 key. */
        OXIDE_KEY_0,
        /** Top-row digit 1 key. */
        OXIDE_KEY_1,
        /** Top-row digit 2 key. */
        OXIDE_KEY_2,
        /** Top-row digit 3 key. */
        OXIDE_KEY_3,

        /** Top-row digit 4 key. */
        OXIDE_KEY_4,
        /** Top-row digit 5 key. */
        OXIDE_KEY_5,
        /** Top-row digit 6 key. */
        OXIDE_KEY_6,
        /** Top-row digit 7 key. */
        OXIDE_KEY_7,

        /** Top-row digit 8 key. */
        OXIDE_KEY_8,
        /** Top-row digit 9 key. */
        OXIDE_KEY_9,

        /** Physical escape key. */
        OXIDE_KEY_ESCAPE,
        /** Physical space key. */
        OXIDE_KEY_SPACE,
        /** Physical enter key. */
        OXIDE_KEY_ENTER,

        /** Physical tab key. */
        OXIDE_KEY_TAB,
        /** Physical backspace key. */
        OXIDE_KEY_BACKSPACE,

        /** Physical left shift key. */
        OXIDE_KEY_LEFT_SHIFT,
        /** Physical right shift key. */
        OXIDE_KEY_RIGHT_SHIFT,

        /** Physical left control key. */
        OXIDE_KEY_LEFT_CONTROL,
        /** Physical right control key. */
        OXIDE_KEY_RIGHT_CONTROL,

        /** Physical left alt key. */
        OXIDE_KEY_LEFT_ALT,
        /** Physical right alt key. */
        OXIDE_KEY_RIGHT_ALT,

        /** Physical up key. */
        OXIDE_KEY_UP,
        /** Physical down key. */
        OXIDE_KEY_DOWN,
        /** Physical left key. */
        OXIDE_KEY_LEFT,
        /** Physical right key. */
        OXIDE_KEY_RIGHT,

        /** Physical insert key. */
        OXIDE_KEY_INSERT,
        /** Physical delete key. */
        OXIDE_KEY_DELETE,
        /** Physical home key. */
        OXIDE_KEY_HOME,
        /** Physical end key. */
        OXIDE_KEY_END,

        /** Physical page up key. */
        OXIDE_KEY_PAGE_UP,
        /** Physical page down key. */
        OXIDE_KEY_PAGE_DOWN,

        /** Physical f1 key. */
        OXIDE_KEY_F1,
        /** Physical f2 key. */
        OXIDE_KEY_F2,
        /** Physical f3 key. */
        OXIDE_KEY_F3,
        /** Physical f4 key. */
        OXIDE_KEY_F4,

        /** Physical f5 key. */
        OXIDE_KEY_F5,
        /** Physical f6 key. */
        OXIDE_KEY_F6,
        /** Physical f7 key. */
        OXIDE_KEY_F7,
        /** Physical f8 key. */
        OXIDE_KEY_F8,

        /** Physical f9 key. */
        OXIDE_KEY_F9,
        /** Physical f10 key. */
        OXIDE_KEY_F10,
        /** Physical f11 key. */
        OXIDE_KEY_F11,
        /** Physical f12 key. */
        OXIDE_KEY_F12
    } OxideKey;

    /** Physical mouse button, including OS-specific additional button indices. */
    typedef enum OxideMouseButton
    {

        /** Left mouse button. */
        OXIDE_MOUSE_BUTTON_LEFT = 0,

        /** Right mouse button. */
        OXIDE_MOUSE_BUTTON_RIGHT = 1,

        /** Middle mouse button. */
        OXIDE_MOUSE_BUTTON_MIDDLE = 2,

        /** Back mouse button. */
        OXIDE_MOUSE_BUTTON_BACK = 3,

        /** Forward mouse button. */
        OXIDE_MOUSE_BUTTON_FORWARD = 4
    } OxideMouseButton;

    /** Current and previous physical key state for held and per-frame transition queries. */
    typedef struct OxideKeyboardState
    {
        /** Current-frame key bitset; key value selects bit key % 64 in word key / 64. */
        uint64_t current[2];
        /** Previous-frame key bitset, using the same layout as current. */
        uint64_t previous[2];
    } OxideKeyboardState;

    /** Mouse snapshot with held buttons, per-frame transitions, position, raw motion, and scrolling. */
    typedef struct OxideMouseState
    {
        /** Current button bits: named buttons use bits 0 through 4; additional indices 0 through 58 use bits 5 through 63. */
        uint64_t current_buttons;
        /** Previous-frame button bits, using the same layout as current_buttons. */
        uint64_t previous_buttons;
        /** World-space light position. */
        OxideVec2 position;
        /** Raw mouse motion accumulated this frame in device-reported units. */
        OxideVec2 delta;
        /** Scroll accumulated this frame, in lines or pixels according to the OS event. */
        OxideVec2 scroll_delta;
    } OxideMouseState;

    /**
 * @brief Writes current and previous physical key bitsets for this frame.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_keyboard_get_state(OxideKeyboardState *result);

    /**
 * @brief Writes current/previous button bits, physical-pixel position, raw delta, and scroll delta.
 * @param result [out] Non-NULL output pointer. Written on success; newly returned resource handles must be destroyed by the caller.
 * @return OXIDE_STATUS_OK on success; otherwise a status code with a thread-local diagnostic.
 */
    OXIDE_API OxideStatus oxide_mouse_get_state(OxideMouseState *result);

#ifdef __cplusplus
}
#endif

#endif
