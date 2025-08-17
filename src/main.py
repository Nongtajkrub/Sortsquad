from enum import Enum
import dataclasses
from dataclasses import dataclass
from typing import Protocol
from functools import lru_cache
from sys import exit
import pygame, data, random, math

pygame.init()
pygame.mixer.music.load(data.MUSIC_PATH)

try:
    pygame.image.load(data.TEST_ASSET_PATH)
except (pygame.error, FileNotFoundError) as e:
    print("\033[1;31m")
    print("Please make sure you are starting this game from the project root directory.")
    print("If you're unsure, refer to the 'Setup' section in the README for setup instructions.")
    print("\033[0m")
    exit(1)

class Color:
    class Hex:
        @staticmethod
        @lru_cache(maxsize=32)
        def to_rgb(hex: str) -> tuple[int, int, int]:
            hex = hex.lstrip("#")
            return (int(hex[0:2], 16), int(hex[2:4], 16), int(hex[4:6], 16))

class Font:
    def __init__(self, path: str, sm = 24, md = 36, lg = 48, xlg = 64, xxlg = 96) -> None:
        self.sm = pygame.font.Font(path, sm)
        self.md = pygame.font.Font(path, md)
        self.lg = pygame.font.Font(path, lg)
        self.xlg = pygame.font.Font(path, xlg)
        self.xxlg = pygame.font.Font(path, xxlg)

class MyEvent:
    _offset = 0

    @classmethod
    def new_timer(cls, time) -> int:
        cls._offset += 1
        event = pygame.USEREVENT + cls._offset
        pygame.time.set_timer(event, time)
        return event

class GameState(Enum):
    MENU = 0
    STORY = 1
    RUNNING = 2
    ENDED = 3

class Game:
    screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
    SCREEN_WIDTH, SCREEN_HEIGHT = screen.get_size()
    SCREEN_CENX = round(SCREEN_WIDTH / 2)
    SCREEN_CENY = round(SCREEN_HEIGHT / 2)
    SCREEN_CEN = (SCREEN_CENX, SCREEN_CENY)
    clock = pygame.time.Clock()
    font = Font(str(data.FONT_PATH)) 
    state = GameState.MENU
    mouse = pygame.mouse.get_pos()
    keys = pygame.key.get_pressed()

    current_time = 0
    current_time_sec = 0
    running = True
    PLAYER_COUNT = 4

    @classmethod
    def clear_screen(cls):
       cls.screen.fill((0, 0, 0))

    @classmethod
    def clock_tick(cls):
        cls.current_time += cls.clock.tick(data.MAX_FPS)
        cls.current_time_sec = math.floor(cls.current_time / 1000)

    @classmethod
    def update_input(cls):
        cls.mouse = pygame.mouse.get_pos()
        cls.keys = pygame.key.get_pressed()

    @classmethod
    def draw_text(
        cls,
        font: pygame.font.Font, text: str, pos: tuple[int, int], color=(0, 0, 0)
    ) -> None:
        surf = font.render(text, True, color)
        cls.screen.blit(surf, surf.get_rect(center=pos))

    @classmethod
    def draw_text_outline(
        cls,
        font: pygame.font.Font,
        text: str,
        pos: tuple[int, int],
        color=(0, 0, 0), outline_color=(0, 0, 0), outline_width=3
    ) -> None:
        base = font.render(text, True, color)
        outline = font.render(text, True, outline_color)
        x, y = pos

        # Draw outline
        for dx in range(-outline_width, outline_width + 1):
            for dy in range(-outline_width, outline_width + 1):
                if dx**2 + dy**2 <= outline_width**2:  # circular outline
                    cls.screen.blit(outline, outline.get_rect(center=(x + dx, y + dy)))

        # Draw main text on top
        cls.screen.blit(base, base.get_rect(center=pos))

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
        path: str,
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
        Game.screen.blit(self._image, self._rect.topleft)
        
    def get_rect(self) -> pygame.rect.Rect:
        return self._rect

    def set_rect(self, rect) -> None:
        self._rect = rect

class Background:
    def __init__(self, path: str) -> None:
        self._image = pygame.transform.scale(
            pygame.image.load(path), Game.screen.get_size()).convert()

    def draw(self):
        Game.screen.blit(self._image, (0, 0))

# A horrible error prone animation system
class SpriteAnimations(SpriteControls):
    def __init__(
        self,
        path: str,
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
        self._rect = self._framse[0].get_rect(center=pos)

        self._cloneable = cloneable 
        if cloneable:
            self._path = path
            self._grid_size = grid_size
            self._scale = scale

    def _generate_framse(
        self,
        path: str, grid_size: int, scale: tuple[int, int] = (100, 100)
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
    # Resource Acquisition Is Initialization
    _raii: bool = dataclasses.field(default=False) 

    def is_free(self) -> bool:
        return self._free

    def is_raii(self) -> bool:
        return self._raii

    def raii(self) -> None:
        if self.sprite.is_loop():
            raise Exception("AnimationHeap raii only work with none looping animation.")

        self._raii = True

    def free(self) -> None:
        self._free = True 
        self._raii = False
        self.sprite.restart()

class AnimationHeap:
    _heap: dict[str, list[AnimationHeapData]] = {}

    # !!! Expensive !!!
    @classmethod
    def malloc(cls, category: str, sprite: SpriteAnimations, n: int) -> None:
        if not sprite.is_cloneable():
            raise Exception("AnimationManager does not handle uncloneable animation")

        cls._heap.setdefault(category, [])
        cls._heap[category].extend(
            [AnimationHeapData(sprite.clone()) for _ in range(n)])

    @classmethod
    def request(cls, category: str, pos: tuple[int, int] = (0, 0)) -> AnimationHeapData:
        for sprite in cls._heap[category]:
            if sprite.is_free():
                sprite._free = False 
                sprite.sprite.get_rect().center = pos
                return sprite

        raise Exception("AnimationManager out of memory.")

    @classmethod
    def update_raii(cls, category: str) -> None:
        for sprite in cls._heap[category]:
            if not sprite.is_free() and sprite.is_raii():
                if sprite.sprite.is_finish():
                    sprite.free()
                else:
                    sprite.sprite.draw()

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
            self._cycle[self._current_cycle].draw()
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
        return random.choice(list(cls)) # type: ignore[return-value]

    def to_path(self) -> str:
        match self:
            case OrganicTrashes.APPLE: return data.APPLE_IMG_PATH
            case OrganicTrashes.BANANA: return data.FISH_IMG_PATH
            case OrganicTrashes.VEGETABLE: return data.VEGETABLE_IMG_PATH

class HazardousTrashes(Enum):
    BATTERY = 0
    ELECTRONIC = 1
    BLEACH = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> str:
        match self:
            case HazardousTrashes.BATTERY: return data.BATTERY_IMG_PATH
            case HazardousTrashes.ELECTRONIC: return data.ELECTRONIC_IMG_PATH
            case HazardousTrashes.BLEACH: return data.BLEACH_IMG_PATH

class RecyclableTrashes(Enum):
    BOTTLE = 0
    COKE = 1
    PAPER = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> str:
        match self:
            case RecyclableTrashes.BOTTLE: return data.BOTTLE_IMG_PATH
            case RecyclableTrashes.COKE: return data.COKE_IMG_PATH
            case RecyclableTrashes.PAPER: return data.PAPER_IMG_PATH

class GeneralTrashes(Enum):
    SHOES = 0
    TISSUE = 1
    CIGARETTE = 2

    @classmethod
    def random(cls):
        return random.choice(list(cls))

    def to_path(self) -> str:
        match self:
            case GeneralTrashes.SHOES: return data.SHOE_IMG_PATH
            case GeneralTrashes.TISSUE: return data.TISSUE_IMG_PATH
            case GeneralTrashes.CIGARETTE: return data.CIGARETTE_IMG_PATH

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

    def to_color(self) -> tuple[int, int, int]:
        match self:
            case TrashCategories.ORGANIC: return Color.Hex.to_rgb("#5aba27")
            case TrashCategories.HAZARDOUS: return Color.Hex.to_rgb("#dd0000")
            case TrashCategories.RECYCLABLE: return Color.Hex.to_rgb("#ffe700")
            case TrashCategories.GENERAL: return Color.Hex.to_rgb("#0091de")

    def to_bin_animation_cycler(self) -> AnimationCycler :
        idle_path, prerun_path, run_path = None, None, None

        posx = round(Game.SCREEN_WIDTH / 2)
        posy = Game.SCREEN_HEIGHT - 60

        match self:
            case TrashCategories.ORGANIC:
                idle_path = data.ORGANIC_IDLE_PATH
                prerun_path = data.ORGANIC_PRERUN_PATH
                run_path = data.ORGANIC_RUN_PATH
                posx -= 100
            case TrashCategories.GENERAL:
                idle_path = data.GENERAL_IDLE_PATH
                prerun_path = data.GENERAL_PRERUN_PATH
                run_path = data.GENERAL_RUN_PATH
                posx -= 30
            case TrashCategories.RECYCLABLE:
                idle_path = data.RECYCLABLE_IDLE_PATH
                prerun_path = data.RECYCLABLE_PRERUN_PATH
                run_path = data.RECYCLABLE_RUN_PATH
                posx += 30
            case TrashCategories.HAZARDOUS:
                idle_path = data.HAZARDOUS_IDLE_PATH
                prerun_path = data.HAZARDOUS_PRERUN_PATH
                run_path = data.HAZARDOUS_RUN_PATH
                posx += 100

        return AnimationCycler((
            SpriteAnimations(idle_path, 45, 8, 200, loop=True, pos=(posx, posy)),
            SpriteAnimations(prerun_path, 45, 4, 70, pos=(posx, posy)),
            SpriteAnimations(run_path, 45, 8, 100, loop=True, pos=(posx, posy))))

class PowerUpCategories(Enum):
    SPEED = 0
    DOUBLE_POINT = 1
    SHIELD = 2
    
    @classmethod
    def random(cls):
        return random.choice(list(cls))
    
    def to_string(self) -> str:
        match self:
            case PowerUpCategories.SPEED:
                return "Speed".capitalize()
            case PowerUpCategories.DOUBLE_POINT:
                return "Double Point".capitalize()
            case PowerUpCategories.SHIELD:
                return "Shield".capitalize()

    def to_color(self) -> tuple[int, int, int]:
        match self:
            case PowerUpCategories.SPEED:
                return Color.Hex.to_rgb("#22d3ee")
            case PowerUpCategories.DOUBLE_POINT:
                return Color.Hex.to_rgb("#fde047")
            case PowerUpCategories.SHIELD:
                return Color.Hex.to_rgb("#dc2626")

class PowerUp(Sprite):
    SPAWN_EVENT = MyEvent.new_timer(data.POWER_UP_SPAWN_FREQ) 
    pygame.time.set_timer(SPAWN_EVENT, data.POWER_UP_SPAWN_FREQ)
    
    def __init__(self) -> None:
        super().__init__(data.POWER_UP_IMG_PATH, scale=(70, 70))
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
    SPAWN_EVENT = MyEvent.new_timer(data.TRASH_SPAWN_FREQ) 
    pygame.time.set_timer(SPAWN_EVENT, data.TRASH_SPAWN_FREQ)

    sorted: dict[TrashCategories, int] = {
        TrashCategories.ORGANIC: 0,
        TrashCategories.HAZARDOUS: 0,
        TrashCategories.RECYCLABLE: 0,
        TrashCategories.GENERAL: 0
    }

    AnimationHeap.malloc(
        "portal",
        SpriteAnimations(data.PORTAL_IMG_PATH, 32, 6, 70, cloneable=True),
        data.PORTAL_ANIMATION_HEAP_N)

    def __init__(self) -> None:
        self._category = TrashCategories.random()
        posx = random.randint(0, Game.SCREEN_WIDTH)
        super().__init__(
            self._category.to_trash().random().to_path(), (posx, -50), (60, 60))
        self.rotate(random.randint(*data.TRASH_ROTATED_RANGE))
        self._alive = True

        AnimationHeap.request("portal", (posx, 50)).raii()

    def _movement_loop(self) -> None:
        self.get_rect().centerx += round(math.sin(Game.current_time * 0.005) * 1)
        self.get_rect().centery += data.DEFAULT_TRASH_VEL

    def loop(self) -> None:
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
    AnimationHeap.malloc(
        "scored_animation1",
        SpriteAnimations(data.SCORE_ANIMATION1_PATH, 32, 7, 50, cloneable=True),
        data.SCORE_ANIMATION1_HEAP_N)

    AnimationHeap.malloc(
        "scored_animation2",
        SpriteAnimations(data.SCORE_ANIMATION2_PATH, 32, 7, 50, cloneable=True),
        data.SCORE_ANIMATION2_HEAP_N)

    AnimationHeap.malloc(
        "wrong_animation1",
        SpriteAnimations(data.WRONG1_ANIMATION1_PATH, 32, 7, 50, cloneable=True),
        data.WRONG1_ANIMATION1_HEAP_N)

    def __init__(self, control: tuple[int, int], category: TrashCategories) -> None:
        self._sprites = category.to_bin_animation_cycler()

        self._left_key, self._right_key = control
        self._facing = Direction.RIGHT
        self._velocity = 0
        self._score = 0
        self._bin_category = category

        self._power_up: PowerUpCategories | None = None
        self._power_up_applied_tick: int | None = None 
        self._power_up_shield_sprite = Sprite(data.SHIELD_IMG_PATH)

        self._scored_sound = pygame.mixer.Sound(data.SCORED_AUD_PATH)
        self._scored_sound.set_volume(0.1)

    def get_rect(self) -> pygame.rect.Rect:
        return self._sprites.get_rect()

    def _calc_velocity(self) -> int:
        return (-1 if self._facing == Direction.LEFT else 1) * data.DEFAULT_PLAYER_VEL

    def _movement_loop(self) -> None:
        new_facing = self._facing
        new_velocity = 0
        
        if Game.keys[self._left_key] and self.get_rect().topleft[0] > 0:
            new_facing = Direction.LEFT
            new_velocity = self._calc_velocity()
        elif (
            Game.keys[self._right_key] and self.get_rect().topright[0] < Game.SCREEN_WIDTH
        ):
            new_facing = Direction.RIGHT
            new_velocity = self._calc_velocity()

        self.get_rect().centerx += (
            new_velocity if self._power_up != PowerUpCategories.SPEED 
            else round(new_velocity * data.BOOSTED_PLAYER_VEL_MULTIPLIER)) 

        self._movement_animation_loop(new_velocity)
        self._velocity = new_velocity

        if self._facing != new_facing:
            self._sprites.flipx()

        self._facing = new_facing

    def _movement_animation_loop(self, new_velocity: int) -> None:
        if abs(self._velocity) < abs(new_velocity):
            # If start running
            self._sprites.next()
        elif abs(self._velocity) > abs(new_velocity):
            # If stop running
            self._sprites.restart()

    def _score_loop(self, trashes: list[Trash]):
        for trash in trashes:
            if self.get_rect().colliderect(trash.get_rect()) and trash.is_alive():
                # Double point increment if DOUBLE_POINT power up is enable.
                increment = 1 if self._power_up != PowerUpCategories.DOUBLE_POINT else 2
                # Do not decrement point if SHIELD power up is enable.
                decrement = -1 if self._power_up != PowerUpCategories.SHIELD else 0
                scored = trash.get_category() == self._bin_category

                self._score += increment if scored else decrement
                self._score = max(0, self._score)
                
                if scored:
                    Trash.sorted[self._bin_category] += 1
                    self._scored_sound.play()

                # Only despawn trash if shield power up is disable
                if self._power_up == PowerUpCategories.SHIELD:
                    if trash.get_category() == self._bin_category:
                        trash.despawn()
                else:
                    trash.despawn()

                self._score_animation_loop(scored)

    def _score_animation_loop(self, scored: bool) -> None:
        if scored and self._power_up != PowerUpCategories.DOUBLE_POINT:
            AnimationHeap.request("scored_animation1", self.get_rect().center).raii()
        elif scored and self._power_up == PowerUpCategories.DOUBLE_POINT:
            AnimationHeap.request("scored_animation2", self.get_rect().center).raii()
        elif not scored and self._power_up != PowerUpCategories.SHIELD:
            AnimationHeap.request("wrong_animation1", self.get_rect().center).raii()

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
            Game.draw_text_outline(
                Game.font.sm,
                self._power_up.to_string(),
                (self.get_rect().centerx, Game.SCREEN_HEIGHT - 160),
                color=self._power_up.to_color())

            # Show shield effect on player if the shield power up is enable.
            if self._power_up == PowerUpCategories.SHIELD:
                self._power_up_shield_sprite.get_rect().center = (self.get_rect().center)
                self._power_up_shield_sprite.draw()

        Game.draw_text_outline(
            Game.font.sm,
            f"Score: {self._score}",
            (self.get_rect().centerx, Game.SCREEN_HEIGHT - 140),
            color=(255, 255, 255))

    def loop(self, trashes: list[Trash], power_up: PowerUp) -> None:
        self._movement_loop()
        self._score_loop(trashes)
        self._power_up_loop(power_up)
        self._graphic_loop()
        self._sprites.draw()

    def get_score(self) -> int:
        return self._score

class LinearCutscene:
    def __init__(self, scene_paths: tuple[str, ...]) -> None:
        self._scenes = LinearCutscene._generate_scene(scene_paths)
        self._current_scene = 0
    
    @staticmethod
    def _generate_scene(paths: tuple[str, ...]) -> tuple[pygame.surface.Surface, ...]:
        return tuple([
            pygame.transform.scale(
                pygame.image.load(path), Game.screen.get_size()).convert() 
            for path in paths
        ])

    def next(self):
        self._current_scene = min(len(self._scenes) - 1, self._current_scene + 1)

    def draw(self):
        Game.screen.blit(self._scenes[self._current_scene], (0, 0))

    def is_finish(self):
        return self._current_scene == len(self._scenes) - 1

    def get_scene(self) -> int:
        return self._current_scene

class FadingOutEffect:
    def __init__(
        self,
        speed: int, color: tuple[int, int, int] = (0, 0, 0), hold: int = 0
    ) -> None:
        self._sur = pygame.Surface(Game.screen.get_size())
        self._sur.fill(color)
        self._current_alpha = 0
        self._speed = speed
        self._hold = hold
        self._finish_at: int | None = None

    def loop(self) -> None:
        if self._current_alpha < 255:
            self._current_alpha += self._speed
            self._sur.set_alpha(self._current_alpha)
        else:
            # Set _finish_at if it was not already set.
            self._finish_at = (
                Game.current_time if self._finish_at == None else self._finish_at)

        Game.screen.blit(self._sur, (0, 0))

    def is_finish(self) -> bool:
        return self._finish_at != None and Game.current_time - self._finish_at > self._hold

class Buttons:
    def __init__(
        self,
        normal_path: str,
        hoverd_path: str,
        pressed_path: str,
        pos: tuple[int, int] = (0, 0), scale: tuple[int, int] = (100, 100)
    ) -> None:
        self.normal = Sprite(normal_path, pos, scale) 
        self.hover = Sprite(hoverd_path, pos, scale) 
        self.pressed = Sprite(pressed_path, pos, scale)
        self.is_pressed = False

class Environment:
    _background_sky = Background(data.SKY_IMG_PATH)
    _background_grass = pygame.transform.scale(
        pygame.image.load(data.GRASS_IMG_PATH), (Game.screen.get_width(), 100)).convert_alpha()

    _cloudes: list[Sprite] = []
    CLOUDE_SPAWN_EVENT = MyEvent.new_timer(data.CLOUDE_SPAWN_FREQ) 
    pygame.time.set_timer(CLOUDE_SPAWN_EVENT, data.CLOUDE_SPAWN_FREQ)
   
    @classmethod
    def draw_background(cls) -> None:
        cls._background_sky.draw()
        Game.screen.blit(cls._background_grass, (0, Game.SCREEN_HEIGHT - 100))

    @classmethod
    def spawn_cloude(cls) -> None:
        cls._cloudes.append(
            Sprite(
                data.CLOUDE1_IMG_PATH,
                (-100, random.choice(data.CLOUDE_SPAWN_RANGE))))

    @classmethod
    def draw_cloudes(cls) -> None:
        # Loop backward to prevent skipping while deleting cloudes.
        for i in range(len(cls._cloudes) - 1, -1, -1):
            cls._cloudes[i].get_rect().centerx += 2
            cls._cloudes[i].draw()

            if cls._cloudes[i].get_rect().centerx > Game.SCREEN_WIDTH:
                del cls._cloudes[i]

class MainLoopControls(Protocol):
    @classmethod
    def _event_loop(cls) -> None:
        ...

    @classmethod
    def loop(cls) -> None:
        ...

class MenuLoop(MainLoopControls):
    _background = Background(data.MENU_BACKGROUND_IMG_PATH)
    _button = Buttons(
        data.MENU_BUTNORMAL_IMG_PATH,
        data.MENU_BUTHOVER_IMG_PATH,
        data.MENU_BUTPRESSED_IMG_PATH,
        (round(Game.SCREEN_WIDTH / 2), Game.SCREEN_HEIGHT - 330), scale=(300, 100))
    _fade_to_black = FadingOutEffect(5, hold=1000)
    
    _current_credit = 0
    _NAME_CHANGE_EVENT = MyEvent.new_timer(data.MENU_NAME_CHANGE_FREQ)

    @classmethod
    def _event_loop(cls):
        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    Game.running = False
                case pygame.MOUSEBUTTONDOWN:
                    if cls._button.normal._rect.collidepoint(Game.mouse):
                        cls._button.is_pressed = True
                case cls._NAME_CHANGE_EVENT:
                    cls._current_credit = (cls._current_credit + 1) % len(data.CREDITS)
        
        if cls._fade_to_black.is_finish():
            Game.state = GameState.STORY
            StoryLoop.begin()

    @classmethod
    def _credit_loop(cls) -> None:
        name = data.CREDITS[cls._current_credit][0]
        color = data.CREDITS[cls._current_credit][1]
        Game.draw_text_outline(
            Game.font.xlg,
            name, (round(Game.SCREEN_WIDTH / 2), Game.SCREEN_HEIGHT - 180), color)

    @classmethod
    def _button_loop(cls) -> None:
        if cls._button.is_pressed:
            cls._button.pressed.draw()
            return

        if cls._button.normal._rect.collidepoint(Game.mouse):
            cls._button.hover.draw()
        else:
            cls._button.normal.draw()

    @classmethod
    def _graphic_loop(cls) -> None:
        if cls._button.is_pressed:
            cls._fade_to_black.loop()
    
    @classmethod
    def loop(cls) -> None:
        Game.update_input()
        cls._event_loop()

        Game.clear_screen()
        cls._background.draw()
        cls._credit_loop()
        cls._button_loop()
        cls._graphic_loop()
        
        pygame.display.flip()
        Game.clock_tick()

class StoryLoop(MainLoopControls):
    _cutscene = LinearCutscene(data.CUTSCENES_IMG_PATHS)

    @classmethod
    def begin(cls) -> None:
        pygame.mixer.music.play()

    @classmethod
    def _event_loop(cls) -> None:
        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    Game.running = False
                case pygame.MOUSEBUTTONDOWN:
                    if cls._cutscene.is_finish():
                        Game.state = GameState.RUNNING
                        GameLoop.begin()
                    else:
                        cls._cutscene.next()
    @classmethod
    def _cutscene_loop(cls) -> None:
        cls._cutscene.draw()

        if cls._cutscene.get_scene() == 0:
            Game.draw_text_outline(
                Game.font.lg,
                "Click Mouse To Continue",
                (round(Game.SCREEN_WIDTH / 2), Game.SCREEN_HEIGHT - 100),
                color=(255, 255, 255))

    @classmethod
    def loop(cls) -> None:
        Game.update_input()
        cls._event_loop()

        Game.clear_screen()
        cls._cutscene_loop()

        pygame.display.flip()
        Game.clock_tick()

class GameLoop(MainLoopControls):
    game_started: int | None = None
    bins: tuple[TrashBin, ...] = (
        TrashBin((pygame.K_a, pygame.K_s), TrashCategories.GENERAL),
        TrashBin((pygame.K_LEFT, pygame.K_RIGHT), TrashCategories.ORGANIC),
        TrashBin((pygame.K_g, pygame.K_h), TrashCategories.HAZARDOUS),
        TrashBin((pygame.K_COMMA, pygame.K_PERIOD), TrashCategories.RECYCLABLE))
    trashes: list[Trash] = []
    power_up = PowerUp()

    @classmethod
    def begin(cls) -> None:
        GameLoop.game_started = Game.current_time
        pygame.mixer.music.play(start=68)

    @classmethod
    def _event_loop(cls) -> None:
        if (
            cls.game_started != None and
            Game.current_time - cls.game_started >= data.GAME_TIME
        ):
            Game.state = GameState.ENDED
            EndedLoop.begin()

        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    Game.running = False
                case Trash.SPAWN_EVENT:
                    cls.trashes.append(Trash())
                case Environment.CLOUDE_SPAWN_EVENT:
                    Environment.spawn_cloude()
                case PowerUp.SPAWN_EVENT:
                    cls.power_up.spawn()

    @staticmethod
    def _animation_loop() -> None:
        AnimationHeap.update_raii("portal")
        AnimationHeap.update_raii("scored_animation1")
        AnimationHeap.update_raii("scored_animation2")
        AnimationHeap.update_raii("wrong_animation1")

    @classmethod
    def _trash_bins_loop(cls) -> None:
        for bin in cls.bins[:Game.PLAYER_COUNT]:
            bin.loop(cls.trashes, cls.power_up)

    @classmethod
    def _trashes_loop(cls) -> None:
        # Loop backward to prevent skipping while deleting trashes.
        for i in range(len(GameLoop.trashes) - 1, -1, -1):
            cls.trashes[i].loop()

            if not cls.trashes[i].is_alive():
                del cls.trashes[i]
                
    @classmethod
    def _power_up_loops(cls) -> None:
        if cls.power_up.is_alive():
            cls.power_up.movement()
            cls.power_up.draw()

    @classmethod
    def _timer_graphic_loop(cls) -> None:
        if cls.game_started == None:
            raise Exception("Game started unexpectedly.")

        time_left = round((data.GAME_TIME - (Game.current_time - cls.game_started)) / 1000)
        color_r = 255 - round(time_left * data.TIMER_COLOR_MULTIPLIER)

        Game.draw_text_outline(
            Game.font.xxlg,
            str(time_left),
            (round(Game.SCREEN_WIDTH / 2), 150), (255, 255 - color_r, 255 - color_r))

    @classmethod
    def loop(cls) -> None:
        Game.update_input()
        cls._event_loop()

        Game.clear_screen()

        Environment.draw_background()
        cls._trash_bins_loop()
        cls._trashes_loop()
        cls._power_up_loops()
        cls._animation_loop()
        Environment.draw_cloudes()
        cls._timer_graphic_loop()

        pygame.display.flip()
        Game.clock_tick()

class EndedLoop(MainLoopControls):
    _shown_total_score = False
    _scores_menu = Background(data.MENU_SCORES_IMG_PATH)

    @classmethod
    def begin(cls) -> None:
        pygame.mixer.music.play(start=373)
        pygame.event.set_blocked(PowerUp.SPAWN_EVENT)
        pygame.event.set_blocked(Trash.SPAWN_EVENT)
        pygame.event.set_blocked(Environment.CLOUDE_SPAWN_EVENT)
    
    @classmethod
    def _event_loop(cls) -> None:
        for event in pygame.event.get():
            match event.type:
                case pygame.QUIT:
                    Game.running = False
                case pygame.MOUSEBUTTONDOWN:
                    cls._shown_total_score = True

    @classmethod
    def _total_score_loop(cls) -> None:
        Game.draw_text_outline(
            Game.font.xxlg,
            "Total Score".capitalize(),
            (Game.SCREEN_CENX, Game.SCREEN_CENY - 50),
            color=Color.Hex.to_rgb("#ffe700"), outline_width=5)

        Game.draw_text_outline(
            Game.font.xxlg,
            str(sum([bin.get_score() for bin in GameLoop.bins])),
            (Game.SCREEN_CENX, Game.SCREEN_CENY + 50),
            color=(255, 255, 255), outline_width=5)

    @classmethod
    def _score_loop(cls) -> None:
        cls._scores_menu.draw()

        trash_categories = [
            TrashCategories.RECYCLABLE,
            TrashCategories.HAZARDOUS,
            TrashCategories.ORGANIC, TrashCategories.GENERAL]

        for (i, category) in enumerate(trash_categories):
            x_pos = 275 + (i * 455)

            Game.draw_text_outline(
                Game.font.xlg,
                str(Trash.sorted[category]),
                (x_pos, 420), color=category.to_color())

    @classmethod
    def loop(cls) -> None:
        cls._event_loop()
    
        Game.screen.fill(Color.Hex.to_rgb("#242424"))

        if not cls._shown_total_score:
            cls._total_score_loop()
        else:
            cls._score_loop()

        pygame.display.flip()
        Game.clock_tick()

if __name__ == "__main__":
    if data.IMMEDIATE_START:
        Game.state = GameState.RUNNING
        GameLoop.begin()

    while Game.running:
        match Game.state:
            case GameState.MENU:
                MenuLoop.loop()
            case GameState.STORY:
                StoryLoop.loop()
            case GameState.RUNNING:
                GameLoop.loop()
            case GameState.ENDED:
                EndedLoop.loop()

pygame.quit()
