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

OXIDE_API void oxide_run(
    OxideAppConfig config,
    OxideGameCallbacks callbacks
);

#ifdef __cplusplus
}
#endif

#endif