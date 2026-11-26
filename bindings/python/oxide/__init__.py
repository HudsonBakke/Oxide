"""Public Python interface to Oxide. Build the native DLL before importing this package."""

from .input import (
    Keyboard,
    KeyboardState,
    Keys,
    Mouse,
    MouseButtons,
    MouseState,
)
from .errors import (
    ContentError,
    GraphicsError,
    InternalError,
    InvalidArgumentError,
    InvalidOperationError,
    OxideError,
    OxideStatus,
)
from .collision import collides
from .app import (
    AppConfig,
    Game,
    GameTime,
)

from .math import (
    Angle,
    Quaternion,
    Rect,
    Transform,
    Vec2,
    Vec3,
    Vec4,
)

from .content import (
    ContentManager,
    Material,
    Mesh,
    Texture,
    Vertex,
)

from .graphics import (
    Camera,
    Color,
    DirectionalLight,
    DirectionalShadowSettings,
    Graphics,
    PointLight,
    SpotLight,
)

__all__ = [
    "Keyboard",
    "KeyboardState",
    "Keys",
    "Mouse",
    "MouseButtons",
    "MouseState",
    "ContentError",
    "GraphicsError",
    "InternalError",
    "InvalidArgumentError",
    "InvalidOperationError",
    "OxideError",
    "OxideStatus",
    "collides",
    "Angle",
    "AppConfig",
    "Game",
    "GameTime",
    "Quaternion",
    "Rect",
    "Transform",
    "Vec2",
    "Vec3",
    "Vec4",
    "ContentManager",
    "Material",
    "Mesh",
    "Texture",
    "Vertex",
    "Camera",
    "Color",
    "DirectionalLight",
    "DirectionalShadowSettings",
    "Graphics",
    "PointLight",
    "SpotLight",
]
