from enum import Enum
from pathlib import Path
import pygame, data, random

pygame.init()

screen = pygame.display.set_mode(data.SCREEN_DIMENSION)
clock = pygame.time.Clock()
font = pygame.font.Font(data.FONT_PATH, 24)
current_time = 0
running = True

def draw_text(
    text: str,
    pos: tuple[int, int], color: tuple[int, int, int] = (255, 255, 255)
) -> None:
    screen.blit(font.render(text, True, color), pos)

class Sprite:
    def __init__(
        self,
        path: Path,
        pos: tuple[int, int] = (0, 0), scale: tuple[int, int] = (100, 100)
    ) -> None:
        self._image = pygame.transform.scale(pygame.image.load(path), scale) 
        self._rect = self._image.get_rect(center=pos)

    def rotate(self, value) -> None:
        self._image = pygame.transform.rotate(self._image, value)

    def draw(self) -> None:
        screen.blit(self._image, self._rect.center)
        
    def get_rect(self) -> pygame.rect.Rect:
        return self._rect

class TrashCategories(Enum):
    ORGANIC = 0
    HAZARDOUS = 1 
    RECYCLABLE = 2
    GENERAL = 3

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> Path:
        match self:
            case TrashCategories.ORGANIC:
                return Path(data.ORGANIC_IMG_PATH)
            case TrashCategories.HAZARDOUS:
                return Path(data.HAZARDOUS_IMG_PATH)
            case TrashCategories.RECYCLABLE:
                return Path(data.RECYCLABLE_IMG_PATH)
            case TrashCategories.GENERAL:
                return Path(data.GENERAL_IMG_PATH)

class PowerUpCategories(Enum):
    SPEED = 0
    DOUBLE_POINT = 1
    SHIELD = 2
    
    @classmethod
    def random(cls):
        return random.choice(list(cls))
    
    def to_string(self):
        match self:
            case PowerUpCategories.SPEED:
                return "Speed"
            case PowerUpCategories.DOUBLE_POINT:
                return "Double Point"
            case PowerUpCategories.SHIELD:
                return "Shield"

class PowerUp(Sprite):
    SPAWN_EVENT = SPAWN_EVENT = pygame.USEREVENT + 2
    pygame.time.set_timer(SPAWN_EVENT, data.POWER_UP_SPAWN_FREQ)
    
    def __init__(self) -> None:
        super().__init__(Path(data.GENERAL_IMG_PATH), scale=(150, 50))
        self._category: PowerUpCategories | None = None

    def movement(self) -> None:
        if self._rect.centery < data.DEFAULT_POWER_UP_Y:
            self._rect.centery += data.DEFAULT_POWER_UP_VEL 

    def spawn(self) -> None:
        self._rect.center = (random.randint(0, data.SCREEN_WIDTH), 0)
        self._category = PowerUpCategories.random()
        
    def despawn(self) -> None:
        self._category = None
    
    def is_alive(self) -> bool:
        return self._category != None

    def get_category(self) -> PowerUpCategories | None:
        return self._category

class Trash(Sprite):
    SPAWN_EVENT = pygame.USEREVENT + 1
    pygame.time.set_timer(SPAWN_EVENT, data.TRASH_SPAWN_FREQ)

    def __init__(self) -> None:
        self._category = TrashCategories.random()
        super().__init__(
            self._category.to_path(),
            (random.randint(0, data.SCREEN_WIDTH), -50), (50, 50))
        self._alive = True

    def movement(self) -> None:
        self._rect.centery += data.DEFAULT_TRASH_VEL

    def get_category(self) -> TrashCategories:
        return self._category

    def despawn(self) -> None:
        self._alive = False 

    def is_alive(self) -> bool:
        if self._rect.centery > data.SCREEN_HEIGHT:
            return False
        else:
            return self._alive 

class TrashBin(Sprite):
    def __init__(
        self,
        path: Path, control: tuple[int, int], category: TrashCategories
    ) -> None:
        super().__init__(path, (0, data.DEFAULT_PLAYER_Y))

        self._left_key, self._right_key = control
        self._score = 0
        self._bin_category = category

        self._power_up: PowerUpCategories | None = None
        self._power_up_applied_tick: int | None = None 
        self._power_up_shield_sprite = Sprite(Path(data.SHIELD_IMG_PATH))

    def _movement_loop(self, keys) -> None:
        velocity = data.DEFAULT_PLAYER_VEL if self._power_up != PowerUpCategories.SPEED else data.BOOSTED_PLAYER_VEL
        
        if keys[self._left_key] and self._rect.topleft[0] > 0:
            self._rect.centerx -= velocity
        elif keys[self._right_key] and self._rect.topright[0] < data.SCREEN_WIDTH:
            self._rect.centerx += velocity

    def _score_loop(self, trashes: list[Trash]):
        for trash in trashes:
            if self._rect.colliderect(trash.get_rect()):
                # Double point increment if DOUBLE_POINT power up is enable.
                increment = 1 if self._power_up != PowerUpCategories.DOUBLE_POINT else 2
                # Do not decrement point if SHIELD power up is enable.
                decrement = -1 if self._power_up != PowerUpCategories.SHIELD else 0
                self._score += increment if trash.get_category() == self._bin_category else decrement

                # Only despawn trash if shield power up is disable
                if self._power_up == PowerUpCategories.SHIELD:
                    if trash.get_category() == self._bin_category:
                        trash.despawn()
                else:
                    trash.despawn()

    def _power_up_loop(self, power_up: PowerUp) -> None:
        if self._power_up_applied_tick != None and current_time - self._power_up_applied_tick > data.POWER_UP_TIME:
            self._power_up = None
            self._power_up_applied_tick = None

        if power_up.is_alive() and self._rect.colliderect(power_up.get_rect()):         
            self._power_up_applied_tick = current_time
            self._power_up = power_up.get_category()
            power_up.despawn()

    def _graphic_loop(self) -> None:
        # Show power up on player head.
        if self._power_up != None:
            draw_text(
                self._power_up.to_string(),
                (self._rect.centerx, data.DEFAULT_PLAYER_Y - 50))

            # Show shield effect on player if the shield power up is enable.
            if self._power_up == PowerUpCategories.SHIELD:
                self._power_up_shield_sprite._rect.center = self._rect.center
                self._power_up_shield_sprite.draw()

        draw_text(
            f"Score: {self._score}",
            (self._rect.centerx, data.DEFAULT_PLAYER_Y - 30))

    def loop(self, keys, trashes: list[Trash], power_up: PowerUp) -> None:
        self._movement_loop(keys)
        self._score_loop(trashes)
        self._power_up_loop(power_up)
        self._graphic_loop()
        self.draw()

general_bin = TrashBin(
    Path(data.GENERAL_IMG_PATH),
    (pygame.K_a, pygame.K_s), TrashCategories.GENERAL)
organic_bin = TrashBin(
    Path(data.ORGANIC_IMG_PATH),
    (pygame.K_LEFT, pygame.K_RIGHT), TrashCategories.ORGANIC)
hazardous_bin = TrashBin(
    Path(data.HAZARDOUS_IMG_PATH),
    (pygame.K_g, pygame.K_h), TrashCategories.HAZARDOUS)
recyclable_bin = TrashBin(
    Path(data.RECYCLABLE_IMG_PATH),
    (pygame.K_COMMA, pygame.K_PERIOD), TrashCategories.RECYCLABLE)
trashes: list[Trash] = []
power_up = PowerUp()

def event_loop() -> None:
    global running

    for event in pygame.event.get():
        match event.type:
            case pygame.QUIT:
                running = False
            case Trash.SPAWN_EVENT:
                trashes.append(Trash())
            case PowerUp.SPAWN_EVENT:
                power_up.spawn()

def trash_bins_loop() -> None:
    keys = pygame.key.get_pressed()

    general_bin.loop(keys, trashes, power_up)
    organic_bin.loop(keys, trashes, power_up)
    hazardous_bin.loop(keys, trashes, power_up)
    recyclable_bin.loop(keys, trashes, power_up)

def trashes_loop() -> None:
    # Loop backward to prevent skipping while deleting trashes.
    for i in range(len(trashes) - 1, -1, -1):
        trashes[i].movement()
        trashes[i].draw()

        if not trashes[i].is_alive():
            del trashes[i]
            
def power_up_loops() -> None:
    if power_up.is_alive():
        power_up.movement()
        power_up.draw()

while running:
    event_loop()

    keys = pygame.key.get_pressed()
    screen.fill((0, 0, 0))

    trash_bins_loop()
    trashes_loop()
    power_up_loops()

    pygame.display.flip()
    current_time += clock.tick(data.MAX_FPS)

pygame.quit()
