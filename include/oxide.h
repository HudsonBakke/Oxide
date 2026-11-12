#ifndef OXIDE_H
#define OXIDE_H

#include <stdint.h>

#ifdef _WIN32
    #define OXIDE_API __declspec(dllimport)
#else
    #define OXIDE_API
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef enum OxideStatus
{
    OXIDE_STATUS_OK = 0,
    OXIDE_STATUS_INVALID_ARGUMENT = 1,
    OXIDE_STATUS_INVALID_OPERATION = 2,
    OXIDE_STATUS_CONTENT_ERROR = 3,
    OXIDE_STATUS_GRAPHICS_ERROR = 4,
    OXIDE_STATUS_INTERNAL_ERROR = 5
} OxideStatus;

typedef struct OxideGameTime
{
    double delta_seconds;
    double total_seconds;
} OxideGameTime;

typedef struct OxideGraphicsHandle
{
    void* ptr;
} OxideGraphicsHandle;

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

OXIDE_API OxideStatus oxide_run(
    OxideAppConfig config,
    OxideGameCallbacks callbacks
);

OXIDE_API const char*
oxide_last_error_message(void);

#ifdef __cplusplus
}
#endif

#endif