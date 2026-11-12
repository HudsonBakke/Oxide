#include <stdio.h>

#include "../../include/oxide.h"

static void initialize(
    void* user_data
)
{
    (void)user_data;

    printf(
        "C initialize callback!\n"
    );
}

static void update(
    void* user_data,
    OxideGameTime time
)
{
    (void)user_data;

    static int printed = 0;

    if (!printed)
    {
        printf(
            "C update callback! "
            "delta = %.6f, "
            "total = %.6f\n",
            time.delta_seconds,
            time.total_seconds
        );

        printed = 1;
    }
}

static void draw(
    void* user_data,
    OxideGraphicsHandle graphics
)
{
    (void)user_data;
    (void)graphics;

    static int printed = 0;

    if (!printed)
    {
        printf(
            "C draw callback!\n"
        );

        printed = 1;
    }
}

int main(void)
{
    OxideAppConfig config = {
        .title =
            "Oxide C ABI Test",

        .width =
            1280,

        .height =
            720,
    };

    OxideGameCallbacks callbacks = {
        .user_data =
            NULL,

        .initialize =
            initialize,

        .update =
            update,

        .draw =
            draw,
    };

    printf(
        "Starting Oxide from C...\n"
    );

    oxide_run(
        config,
        callbacks
    );

    printf(
        "Oxide returned to C.\n"
    );

    return 0;
}