from enum import Enum
from pathlib import Path
import dataclasses
from dataclasses import dataclass
from typing import Protocol
import pygame, data, random, math

pygame.init()

class Font:
    def __init__(self, path: Path, sm = 24, md = 36, lg = 48, xlg = 64) -> None:
        self.sm = pygame.font.Font(path, sm)
        self.md = pygame.font.Font(path, md)
        self.lg = pygame.font.Font(path, lg)
        self.xlg = pygame.font.Font(path, xlg)

class Game:
    screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
    SCREEN_WIDTH, SCREEN_HEIGHT = screen.get_size()
    clock = pygame.time.Clock()
    font = Font(Path(data.FONT_PATH)) 

    current_time = 0
    current_time_sec = 0
    running = True
    ended = False
    PLAYER_COUNT = 4

    background_sky = pygame.transform.scale(
        pygame.image.load(data.SKY_IMG_PATH), screen.get_size()).convert()
    background_grass = pygame.transform.scale(
        pygame.image.load(data.GRASS_IMG_PATH), (screen.get_width(), 100)).convert_alpha()

    @staticmethod
    def draw_background():
        Game.screen.fill((0, 0, 0))
        Game.screen.blit(Game.background_sky, (0, 0))
        Game.screen.blit(Game.background_grass, (0, Game.SCREEN_HEIGHT - 100))

    @staticmethod
    def clock_tick():
        Game.current_time += Game.clock.tick(data.MAX_FPS)
        Game.current_time_sec = math.floor(Game.current_time / 1000)

    @staticmethod
    def draw_text(
        font: pygame.font.Font, text: str, pos: tuple[int, int], color=(0, 0, 0)
    ) -> None:
        surf = font.render(text, True, color)
        Game.screen.blit(surf, surf.get_rect(center=pos))

class SpriteControls(Protocol):
    def draw(self) -> None:
        ...

    def rotate(self, value) -> None:
        ...

    def flipx(self) -> None:
        ...

    def flipy(self) -> None:
        ...
        
    def get_rect(self) -> pygame.rect.Rect:
        ...

    def set_rect(self, rect) -> None:
        ...

class Sprite(SpriteControls):
    def __init__(
        self,
        path: Path,
        pos: tuple[int, int] = (0, 0), scale: tuple[int, int] = (100, 100)
    ) -> None:
        self._image = pygame.transform.scale(pygame.image.load(path), scale).convert_alpha()
        self._rect = self._image.get_rect(center=pos)

    def rotate(self, value) -> None:
        self._image = pygame.transform.rotate(self._image, value).convert_alpha()

    def flipx(self) -> None:
        self._image = pygame.transform.flip(self._image, True, False).convert_alpha()

    def flipy(self) -> None:
        self._image = pygame.transform.flip(self._image, False, True).convert_alpha()

    def draw(self) -> None:
        Game.screen.blit(self._image, self._rect.center)
        
    def get_rect(self) -> pygame.rect.Rect:
        return self._rect

    def set_rect(self, rect) -> None:
        self._rect = rect

# A horrible error prone animation system
class SpriteAnimations(SpriteControls):
    def __init__(
        self,
        path: Path,
        grid_size: int,
        grid_count: int,
        delay: int,
        loop: bool = False,
        cloneable: bool = False,
        pos: tuple[int, int] = (0, 0), scale: tuple[int, int] = (100, 100)
    ) -> None:
        self._delay = delay 
        self._loop = loop
        self._last_update = Game.current_time
        self._grid_count = grid_count
        self._framse: list[pygame.surface.Surface] = self._generate_framse(
            path, grid_size, scale)
        self._current_frame = 0
        self._rect = pygame.rect.Rect(pos, (grid_size, grid_size)) 

        self._cloneable = cloneable 
        if cloneable:
            self._path = path
            self._grid_size = grid_size
            self._scale = scale

    def _generate_framse(
        self,
        path: Path, grid_size: int, scale: tuple[int, int] = (100, 100)
    ) -> list[pygame.surface.Surface]:
        sheet = pygame.image.load(path).convert_alpha()

        return [
            pygame.transform.scale(
                sheet.subsurface(pygame.Rect(i * grid_size, 0, grid_size, grid_size)), scale)
            for i in range(self._grid_count)
        ]

    def restart(self) -> None:
        self._last_update = Game.current_time
        self._current_frame = 0

    def draw(self) -> None:
        Game.screen.blit(self._framse[self._current_frame], self.get_rect())

        # increment the current_frame base on whether animation loop or not.
        if Game.current_time - self._last_update >= self._delay:
            self._last_update = Game.current_time
            self._current_frame = (
                (self._current_frame + 1) % self._grid_count if self._loop
                else min(self._current_frame + 1, self._grid_count))

    def rotate(self, value) -> None:
        for frame in self._framse:
            frame = pygame.transform.rotate(frame, value)

    def flipx(self) -> None:
        self._framse = [
            pygame.transform.flip(frame, True, False).convert_alpha() 
            for frame in self._framse]

    def flipy(self) -> None:
        self._framse = [
            pygame.transform.flip(frame, False, True).convert_alpha() 
            for frame in self._framse]

    def get_rect(self) -> pygame.rect.Rect:
        return self._rect

    def set_rect(self, rect) -> None:
        self._rect = rect

    def is_finish(self) -> bool:
        return (not self._loop and self._current_frame == self._grid_count)

    def is_loop(self) -> bool:
        return self._loop

    def is_cloneable(self) -> bool:
        return self._cloneable

    def clone(self) -> "SpriteAnimations":
        if not self._cloneable:
            raise Exception("SpriteAnimations uncloneable.")

        return SpriteAnimations(
            path=self._path,
            grid_size=self._grid_size,
            grid_count=self._grid_count,
            delay=self._delay,
            loop=self._loop,
            pos=self.get_rect().topleft,
            scale=self._scale)

@dataclass
class AnimationHeapData:
    sprite: SpriteAnimations
    _free: bool = dataclasses.field(default=True)

    def is_free(self) -> bool:
        return self._free

    def free(self) -> None:
        self._free = True 
        self.sprite.restart()

class AnimationHeap:
    _heap: dict[str, list[AnimationHeapData]] = {}

    # !!! Expensive !!!
    @staticmethod
    def malloc(category: str, sprite: SpriteAnimations, n: int) -> None:
        if not sprite.is_cloneable():
            raise Exception("AnimationManager does not handle uncloneable animation")

        AnimationHeap._heap.setdefault(category, [])
        AnimationHeap._heap[category].extend(
            [AnimationHeapData(sprite.clone()) for _ in range(n)])

    @staticmethod
    def request(category: str) -> AnimationHeapData:
        for sprite in AnimationHeap._heap[category]:
            if sprite.is_free():
                sprite._free = False 
                return sprite

        raise Exception("AnimationManager out of memory.")

class AnimationCycler(SpriteControls):
    def __init__(self, cycle: tuple[SpriteAnimations, ...]) -> None:
        self._cycle = list(cycle)
        self._current_cycle = 0

    def next(self) -> None:
        # Restsart the current cycle before moving on
        self._cycle[self._current_cycle].restart()

        self._current_cycle = (self._current_cycle + 1) % len(self._cycle)

        # Update the rect on the new cycle from the last cycle.
        self._cycle[self._current_cycle].set_rect(
            self._cycle[(self._current_cycle - 1) % len(self._cycle)].get_rect())

    def restart(self) -> None:
        for sprite in self._cycle:
            sprite.restart()

        self._current_cycle = 0

    def draw(self) -> None:
        this_cycle = self._cycle[self._current_cycle]

        if this_cycle.is_finish():
            self.next()
        else:
            this_cycle.draw()
            
    def rotate(self, value) -> None:
        for sprite in self._cycle:
            sprite.rotate(value)

    def flipx(self) -> None:
        for sprite in self._cycle:
            sprite.flipx()

    def flipy(self) -> None:
        for sprite in self._cycle:
            sprite.flipy()

    def get_rect(self) -> pygame.rect.Rect:
        return self._cycle[self._current_cycle].get_rect()

    def set_rect(self, rect) -> None:
        for sprite in self._cycle:
            sprite.set_rect(rect)

class OrganicTrashes(Enum):
    APPLE = 0
    BANANA = 1
    VEGETABLE = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> Path:
        match self:
            case OrganicTrashes.APPLE: return Path(data.APPLE_IMG_PATH)
            case OrganicTrashes.BANANA: return Path(data.BANANA_IMG_PATH)
            case OrganicTrashes.VEGETABLE: return Path(data.VEGETABLE_IMG_PATH)

class HazardousTrashes(Enum):
    BATTERY = 0
    ELECTRONIC = 1
    BLEACH = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> Path:
        match self:
            case HazardousTrashes.BATTERY: return Path(data.BATTERY_IMG_PATH)
            case HazardousTrashes.ELECTRONIC: return Path(data.ELECTRONIC_IMG_PATH)
            case HazardousTrashes.BLEACH: return Path(data.BLEACH_IMG_PATH)

class RecyclableTrashes(Enum):
    BOTTLE = 0
    COCACOLA = 1
    PAPER = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> Path:
        return Path(data.RECYCLABLE_IMG_PATH)

class GeneralTrashes(Enum):
    SHOES = 0
    FOAM = 1
    CIGARETTE = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> Path:
        return Path(data.GENERAL_IMG_PATH)

class TrashCategories(Enum):
    ORGANIC = 0
    HAZARDOUS = 1 
    RECYCLABLE = 2
    GENERAL = 3

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_trash(self):
        match self:
            case TrashCategories.ORGANIC: return OrganicTrashes 
            case TrashCategories.HAZARDOUS: return HazardousTrashes
            case TrashCategories.RECYCLABLE: return RecyclableTrashes
            case TrashCategories.GENERAL: return GeneralTrashes

    def to_bin_animation_cycler(self) -> AnimationCycler :
        idle_path, prerun_path, run_path = None, None, None

        match self:
            case TrashCategories.ORGANIC:
                idle_path = data.ORGANIC_IDLE_PATH
                prerun_path = data.ORGANIC_PRERUN_PATH
                run_path = data.ORGANIC_RUN_PATH
            case TrashCategories.GENERAL:
                idle_path = data.GENERAL_IDLE_PATH
                prerun_path = data.GENERAL_PRERUN_PATH
                run_path = data.GENERAL_RUN_PATH
            case TrashCategories.RECYCLABLE:
                idle_path = data.RECYCLABLE_IDLE_PATH
                prerun_path = data.RECYCLABLE_PRERUN_PATH
                run_path = data.RECYCLABLE_RUN_PATH
            case TrashCategories.HAZARDOUS:
                idle_path = data.HAZARDOUS_IDLE_PATH
                prerun_path = data.HAZARDOUS_PRERUN_PATH
                run_path = data.HAZARDOUS_RUN_PATH

        pos = (0, Game.SCREEN_HEIGHT - 110)
        return AnimationCycler((
            SpriteAnimations(Path(idle_path), 45, 8, 200, loop=True, pos=pos),
            SpriteAnimations(Path(prerun_path), 45, 4, 100, pos=pos),
            SpriteAnimations(Path(run_path), 45, 4, 100, loop=True, pos=pos)))

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
        super().__init__(Path(data.POWER_UP_IMG_PATH), scale=(50, 50))
        self._category: PowerUpCategories | None = None

    def movement(self) -> None:
        if self.get_rect().centery < Game.SCREEN_HEIGHT - 80:
            self.get_rect().centery += data.DEFAULT_POWER_UP_VEL 

    def spawn(self) -> None:
        self.get_rect().center = (random.randint(0, Game.SCREEN_WIDTH), 0)
        self._category = PowerUpCategories.random()
        
    def despawn(self) -> None:
        self._category = None
    
    def is_alive(self) -> bool:
        return self._category != None

    def get_category(self) -> PowerUpCategories | None:
        return self._category

class Trash(Sprite):
    SPAWN_EVENT = pygame.USEREVENT + 3
    pygame.time.set_timer(SPAWN_EVENT, data.TRASH_SPAWN_FREQ)

    AnimationHeap.malloc(
        "portal",
        SpriteAnimations(Path(data.PORTAL_IMG_PATH), 32, 6, 100, cloneable=True),
        data.PORTAL_ANIMATION_CACHE_N)

    def __init__(self) -> None:
        self._category = TrashCategories.random()
        posx = random.randint(0, Game.SCREEN_WIDTH)
        super().__init__(
            self._category.to_trash().random().to_path(), (posx, -50), (50, 50))
        self._alive = True

        self._portal: None | AnimationHeapData = AnimationHeap.request("portal")
        self._portal.sprite._rect.center = (posx, 10)

    def _movement_loop(self) -> None:
        self.get_rect().centerx += round(math.sin(Game.current_time * 0.005) * 1)
        self.get_rect().centery += data.DEFAULT_TRASH_VEL

    def _animation_loop(self) -> None:
        if self._portal != None:
            if self._portal.sprite.is_finish():
                self._portal.free()
                self._portal = None
            else:
                self._portal.sprite.draw()

    def loop(self) -> None:
        self._animation_loop()
        self._movement_loop()
        self.draw()

    def get_category(self) -> TrashCategories:
        return self._category

    def despawn(self) -> None:
        self._alive = False 

    def is_alive(self) -> bool:
        if self.get_rect().centery > Game.SCREEN_HEIGHT:
            return False
        else:
            return self._alive 

class Direction(Enum):
    LEFT = 0
    RIGHT = 1
    TOP = 2
    BOTTOM = 3

class TrashBin():
    def __init__(self, control: tuple[int, int], category: TrashCategories) -> None:
        self._sprites = category.to_bin_animation_cycler()

        self._left_key, self._right_key = control
        self._facing = Direction.RIGHT
        self._velocity = 0
        self._score = 0
        self._bin_category = category

        self._power_up: PowerUpCategories | None = None
        self._power_up_applied_tick: int | None = None 
        self._power_up_shield_sprite = Sprite(Path(data.SHIELD_IMG_PATH))

    def get_rect(self) -> pygame.rect.Rect:
        return self._sprites.get_rect()

    def _calc_velocity(self) -> int:
        return (-1 if self._facing == Direction.LEFT else 1) * data.DEFAULT_PLAYER_VEL

    def _movement_loop(self, keys) -> None:
        new_facing = self._facing
        new_velocity = 0
        
        if keys[self._left_key] and self.get_rect().topleft[0] > 0:
            new_facing = Direction.LEFT
            new_velocity = self._calc_velocity()
        elif (
            keys[self._right_key] and self.get_rect().topright[0] < Game.SCREEN_WIDTH
        ):
            new_facing = Direction.RIGHT
            new_velocity = self._calc_velocity()

        self.get_rect().centerx += (
            new_velocity if self._power_up != PowerUpCategories.SPEED 
            else round(new_velocity * data.BOOSTED_PLAYER_VEL_MULTIPLIER)) 

        self._animation_loop(new_velocity)
        self._velocity = new_velocity

        if self._facing != new_facing:
            self._sprites.flipx()

        self._facing = new_facing

    def _animation_loop(self, new_velocity: int) -> None:
        if abs(self._velocity) < abs(new_velocity):
            # If start running
            self._sprites.next()
        elif abs(self._velocity) > abs(new_velocity):
            # If stop running
            self._sprites.restart()

    def _score_loop(self, trashes: list[Trash]):
        for trash in trashes:
            if self.get_rect().colliderect(trash.get_rect()):
                # Double point increment if DOUBLE_POINT power up is enable.
                increment = 1 if self._power_up != PowerUpCategories.DOUBLE_POINT else 2
                # Do not decrement point if SHIELD power up is enable.
                decrement = -1 if self._power_up != PowerUpCategories.SHIELD else 0
                self._score += increment if trash.get_category() == self._bin_category else decrement

                self._score = max(0, self._score)

                # Only despawn trash if shield power up is disable
                if self._power_up == PowerUpCategories.SHIELD:
                    if trash.get_category() == self._bin_category:
                        trash.despawn()
                else:
                    trash.despawn()

    def _power_up_loop(self, power_up: PowerUp) -> None:
        if (
            self._power_up_applied_tick != None 
            and Game.current_time - self._power_up_applied_tick > data.POWER_UP_TIME
        ):
            self._power_up = None
            self._power_up_applied_tick = None

        if (
            power_up.is_alive() and self.get_rect().colliderect(power_up.get_rect())
        ):
            self._power_up_applied_tick = Game.current_time
            self._power_up = power_up.get_category()
            power_up.despawn()

    def _graphic_loop(self) -> None:
        # Show power up on player head.
        if self._power_up != None:
            Game.draw_text(
                Game.font.md,
                self._power_up.to_string(),
                (self.get_rect().centerx, Game.SCREEN_HEIGHT - 160))

            # Show shield effect on player if the shield power up is enable.
            if self._power_up == PowerUpCategories.SHIELD:
                self._power_up_shield_sprite.get_rect().center = (self.get_rect().center)
                self._power_up_shield_sprite.draw()

        Game.draw_text(
            Game.font.md,
            f"Score: {self._score}",
            (self.get_rect().centerx, Game.SCREEN_HEIGHT - 140))

    def loop(self, keys, trashes: list[Trash], power_up: PowerUp) -> None:
        self._movement_loop(keys)
        self._score_loop(trashes)
        self._power_up_loop(power_up)
        self._graphic_loop()
        self._sprites.draw()

    def get_score(self) -> int:
        return self._score

class GameLoop:
    bins: tuple[TrashBin, ...] = (
        TrashBin((pygame.K_a, pygame.K_s), TrashCategories.GENERAL),
        TrashBin((pygame.K_LEFT, pygame.K_RIGHT), TrashCategories.ORGANIC),
        TrashBin((pygame.K_g, pygame.K_h), TrashCategories.HAZARDOUS),
        TrashBin((pygame.K_COMMA, pygame.K_PERIOD), TrashCategories.RECYCLABLE))
    trashes: list[Trash] = []
    power_up = PowerUp()

    @staticmethod
    def _event_loop() -> None:
        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    Game.running = False
                case Trash.SPAWN_EVENT:
                    GameLoop.trashes.append(Trash())
                case PowerUp.SPAWN_EVENT:
                    GameLoop.power_up.spawn()

        Game.ended = Game.current_time >= data.GAME_TIME

    @staticmethod
    def _trash_bins_loop() -> None:
        keys = pygame.key.get_pressed()

        for bin in GameLoop.bins[:Game.PLAYER_COUNT]:
            bin.loop(keys, GameLoop.trashes, GameLoop.power_up)

    @staticmethod
    def _trashes_loop() -> None:
        # Loop backward to prevent skipping while deleting trashes.
        for i in range(len(GameLoop.trashes) - 1, -1, -1):
            GameLoop.trashes[i].loop()

            if not GameLoop.trashes[i].is_alive():
                del GameLoop.trashes[i]
                
    @staticmethod
    def _power_up_loops() -> None:
        if GameLoop.power_up.is_alive():
            GameLoop.power_up.movement()
            GameLoop.power_up.draw()

    @staticmethod
    def _timer_graphic_loop() -> None:
        Game.draw_text(
            Game.font.xlg,
            str(Game.current_time_sec), (round(Game.SCREEN_WIDTH / 2), 100))

    @staticmethod
    def main_loop() -> None:
        GameLoop._event_loop()

        Game.draw_background()

        GameLoop._timer_graphic_loop()
        GameLoop._trash_bins_loop()
        GameLoop._trashes_loop()
        GameLoop._power_up_loops()

        pygame.display.flip()
        Game.clock_tick()
        
    @staticmethod
    def ended_loop():
        GameLoop._event_loop()

        total_score = sum([bin.get_score() for bin in GameLoop.bins])
    
        Game.screen.fill((0, 0, 0))
        Game.draw_text(
            Game.font.xlg,
            f"Game Ended! Total Score {total_score}",
            (round(Game.SCREEN_WIDTH / 2), round(Game.SCREEN_HEIGHT / 2)),
            (255, 255, 255))

        pygame.display.flip()
        Game.clock_tick()

while Game.running:
    if not Game.ended:
        GameLoop.main_loop()
    else:
        GameLoop.ended_loop()

pygame.quit()
