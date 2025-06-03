CREDITS = [
    ("Taj Borthwick", (255, 0, 0)),
    ("Pakthan Fullname", (0, 255, 0)),
    ("Issac Fullname", (0, 0, 255))
]

_ASSETS_PATH = "/home/nongtajkrub/Coding/Python/trash-game/assets"

_MENU_PATH = _ASSETS_PATH + "/menu"
_CUTSCENE_PATH = _ASSETS_PATH + "/cutscene"
_AUDIO_PATH = _ASSETS_PATH + "/audio"
_ENV_PATH = _ASSETS_PATH + "/env"
_BINS_PATH = _ASSETS_PATH + "/bins"
_TRASHES_PATH = _ASSETS_PATH + "/trahse"
_POWERUP_PATH = _ASSETS_PATH + "/powerup"
_FONT_PATH = _ASSETS_PATH + "/fonts"

MENU_BUTNORMAL_IMG_PATH = _MENU_PATH + "/buttonnormal.png" 
MENU_BUTPRESSED_IMG_PATH = _MENU_PATH + "/buttonpressed.png" 
MENU_BUTHOVER_IMG_PATH = _MENU_PATH + "/buttonhover.png" 
MENU_NAME_CHANGE_FREQ = 1500

CUTSCENES_IMG_PATHS = (
    _CUTSCENE_PATH + "/1.png",
    _CUTSCENE_PATH + "/2.png",
    _CUTSCENE_PATH + "/3.png",
    _CUTSCENE_PATH + "/4.png",
    _CUTSCENE_PATH + "/5.png",
    _CUTSCENE_PATH + "/6.png",
    _CUTSCENE_PATH + "/7.png",
    _CUTSCENE_PATH + "/8.png",
    _CUTSCENE_PATH + "/9.png",
    _CUTSCENE_PATH + "/10.png",
    _CUTSCENE_PATH + "/11.png",
    _CUTSCENE_PATH + "/12.png",
    _CUTSCENE_PATH + "/13.png",
)

MUSIC_PATH = _AUDIO_PATH + "/music.mp3"

SKY_IMG_PATH = _ENV_PATH + "/sky.jpg"
GRASS_IMG_PATH = _ENV_PATH + "/grass.png"
PORTAL_IMG_PATH = _ENV_PATH + "/portal.png"

CLOUDE_BASE_VEL = 0.6
CLOUDE_SPAWN_RANGE = (100, 150, 200, 250, 300, 350, 400, 450, 500)
CLOUDE_SPAWN_FREQ = 3000
CLOUDE1_IMG_PATH = _ENV_PATH + "/cloudes/cloude1.png"

HAZARDOUS_IMG_PATH = _BINS_PATH + "/hazardous/static.png"
HAZARDOUS_IDLE_PATH = _BINS_PATH + "/hazardous/idle.png"
HAZARDOUS_PRERUN_PATH = _BINS_PATH + "/hazardous/prerunning.png"
HAZARDOUS_RUN_PATH = _BINS_PATH + "/hazardous/running.png"

RECYCLABLE_IMG_PATH = _BINS_PATH + "/recyclable/static.png"
RECYCLABLE_IDLE_PATH = _BINS_PATH + "/recyclable/idle.png"
RECYCLABLE_PRERUN_PATH = _BINS_PATH + "/recyclable/prerunning.png"
RECYCLABLE_RUN_PATH = _BINS_PATH + "/recyclable/running.png"

ORGANIC_IMG_PATH = _BINS_PATH + "/organic/static.png"
ORGANIC_IDLE_PATH = _BINS_PATH + "/organic/idle.png"
ORGANIC_PRERUN_PATH = _BINS_PATH + "/organic/prerunning.png"
ORGANIC_RUN_PATH = _BINS_PATH + "/organic/running.png"

GENERAL_IMG_PATH = _BINS_PATH + "/general/static.png"
GENERAL_IDLE_PATH = _BINS_PATH + "/general/idle.png"
GENERAL_PRERUN_PATH = _BINS_PATH + "/general/prerunning.png"
GENERAL_RUN_PATH = _BINS_PATH + "/general/running.png"

APPLE_IMG_PATH = _TRASHES_PATH + "/organic/apple.png"
FISH_IMG_PATH = _TRASHES_PATH + "/organic/fishbone.png"
VEGETABLE_IMG_PATH = _TRASHES_PATH + "/organic/vegatable.png"

BATTERY_IMG_PATH = _TRASHES_PATH + "/hazardous/battery.png"
ELECTRONIC_IMG_PATH = _TRASHES_PATH + "/hazardous/electronic.png"
BLEACH_IMG_PATH = _TRASHES_PATH + "/hazardous/bleach.png"

COKE_IMG_PATH = _TRASHES_PATH + "/recyclable/coke.png"
PAPER_IMG_PATH = _TRASHES_PATH + "/recyclable/newspaper.png"
BOTTLE_IMG_PATH = _TRASHES_PATH + "/recyclable/waterbottle.png"

CIGARETTE_IMG_PATH = _TRASHES_PATH + "/general/ciggarette.png"
TISSUE_IMG_PATH = _TRASHES_PATH + "/general/tissue.png"
SHOE_IMG_PATH = _TRASHES_PATH + "/general/shoe.png" 

POWER_UP_IMG_PATH = _POWERUP_PATH + "/powerup.png"
SHIELD_IMG_PATH = _POWERUP_PATH + "/shield.png"

FONT_PATH = _FONT_PATH + "/font.tff"
SCORE_ANIMATION1_PATH = _FONT_PATH + "/scored_animation1.png"
SCORE_ANIMATION1_HEAP_N = 15
SCORE_ANIMATION2_PATH = _FONT_PATH + "/scored_animation2.png"
SCORE_ANIMATION2_HEAP_N = 5

WRONG1_ANIMATION1_PATH = _FONT_PATH + "/wrong1_animation.png"
WRONG1_ANIMATION1_HEAP_N = 15
"""
WRONG2_ANIMATION1_PATH = _FONT_PATH + "/wrong2_animation.png"
WRONG2_ANIMATION1_HEAP_N = 15
"""

DEFAULT_PLAYER_VEL = 10
BOOSTED_PLAYER_VEL_MULTIPLIER = 1.5

DEFAULT_POWER_UP_VEL = 6
POWER_UP_TIME = 5000
POWER_UP_SPAWN_FREQ = 10000

DEFAULT_TRASH_VEL = 4
TRASH_SPAWN_FREQ = 270
TRASH_ROTATED_RANGE = (-300, 300)
PORTAL_ANIMATION_HEAP_N = round(TRASH_SPAWN_FREQ / 12)

GAME_TIME = 60000
MAX_FPS = 60
