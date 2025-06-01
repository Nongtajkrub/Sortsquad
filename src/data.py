_ASSETS_PATH = "/home/nongtajkrub/Coding/Python/trash-game/assets/"

_TESTS_PATH = _ASSETS_PATH + "/tests"
_ENV_PATH = _ASSETS_PATH + "/env"
_BINS_PATH = _ASSETS_PATH + "/bins"
_TRASHES_PATH = _ASSETS_PATH + "/trahse"
_POWERUP_PATH = _ASSETS_PATH + "/powerup"
_FONT_PATH = _ASSETS_PATH + "/fonts"

ANIMATION_TEST_IMG_PATH = _TESTS_PATH + "animation_test.png"

SKY_IMG_PATH = _ENV_PATH + "/sky.jpg"
GRASS_IMG_PATH = _ENV_PATH + "/grass.png"
PORTAL_IMG_PATH = _ENV_PATH + "/portal.png"

HAZARDOUS_IMG_PATH = _BINS_PATH + "/hazardous/static.png"
RECYCLABLE_IMG_PATH = _BINS_PATH + "/recyclable/static.png"
ORGANIC_IMG_PATH = _BINS_PATH + "/organic/static.png"

GENERAL_IMG_PATH = _BINS_PATH + "/general/static.png"
GENERAL_IDLE_PATH = _BINS_PATH + "/general/idle.png"
GENERAL_PRERUN_PATH = _BINS_PATH + "/general/prerunning.png"
GENERAL_RUN_PATH = _BINS_PATH + "/general/running.png"

APPLE_IMG_PATH = _TRASHES_PATH + "/organic/apple.png"
BANANA_IMG_PATH = _TRASHES_PATH + "/organic/banana.png"
VEGETABLE_IMG_PATH = _TRASHES_PATH + "/organic/vegetable.png"

BATTERY_IMG_PATH = _TRASHES_PATH + "/hazardous/battery.png"
ELECTRONIC_IMG_PATH = _TRASHES_PATH + "/hazardous/electronic.png"
BLEACH_IMG_PATH = _TRASHES_PATH + "/hazardous/bleach.png"

POWER_UP_IMG_PATH = _POWERUP_PATH + "/powerup.png"
SHIELD_IMG_PATH = _POWERUP_PATH + "/shield.png"

FONT_PATH = _FONT_PATH + "/font.tff"

DEFAULT_PLAYER_VEL = 10
BOOSTED_PLAYER_VEL = 15

DEFAULT_POWER_UP_VEL = 6
POWER_UP_TIME = 5000
POWER_UP_SPAWN_FREQ = 10000

DEFAULT_TRASH_VEL = 4
TRASH_SPAWN_FREQ = 270
PORTAL_ANIMATION_CACHE_N = round(TRASH_SPAWN_FREQ / 12)

GAME_TIME = 60000
MAX_FPS = 60
