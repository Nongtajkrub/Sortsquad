from enum import Enum
from pathlib import Path
import pygame, data, random

pygame.init()

screen = pygame.display.set_mode(data.SCREEN_DIMENSION)
clock = pygame.time.Clock()
font = pygame.font.Font(None, 24)
running = True

class Sprite:
    def __init__(
        self,
        path: Path,
        pos: tuple[int, int] = (0, 0), scale: tuple[int, int] = (100, 100)
    ) -> None:
        self._image = pygame.transform.scale(pygame.image.load(path), scale) 
        self._rect = self._image.get_rect(center=pos)

    def draw(self):
        screen.blit(self._image, self._rect.center)

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

class Trash(Sprite):
    SPAWN_EVENT = pygame.USEREVENT + 2
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

    def set_alive(self, value) -> None:
        self._alive = value

    def is_alive(self) -> bool:
        if self._rect.centery > data.SCREEN_HEIGHT:
            return False
        else:
            return self._alive 

    def get_rect(self) -> pygame.rect.Rect:
        return self._rect


class TrashBin(Sprite):
    def __init__(
        self,
        path: Path,  control: tuple[int, int], category: TrashCategories
    ) -> None:
        super().__init__(path, (0, 600))
        self._left_key, self._right_key = control
        self._score = 0
        self._bin_category = category 

    def movement(self, keys, vel: int = data.DEFAULT_PLAYER_VEL) -> None:
        if keys[self._left_key] and self._rect.topleft[0] > 0:
            self._rect.centerx -= vel
        elif keys[self._right_key] and self._rect.topright[0] < data.SCREEN_WIDTH:
            self._rect.centerx += vel 

    def check_collision(self, trashes: list[Trash]):
        for trash in trashes:
            if self._rect.colliderect(trash.get_rect()):
                self._score += 1 if trash.get_category() == self._bin_category else -1
                trash.set_alive(False)

    def get_score(self) -> int:
        return self._score

general_bin = TrashBin(
    Path(data.GENERAL_IMG_PATH),
    (pygame.K_a, pygame.K_d), TrashCategories.GENERAL)
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

def event_loop() -> None:
    global running

    for event in pygame.event.get():
        match event.type:
            case pygame.QUIT:
                running = False
            case Trash.SPAWN_EVENT:
                trashes.append(Trash())

def trash_bins_loop() -> None:
    general_bin.movement(keys)
    general_bin.check_collision(trashes)
    general_bin.draw()

    organic_bin.movement(keys)
    organic_bin.check_collision(trashes)
    organic_bin.draw()

    hazardous_bin.movement(keys)
    hazardous_bin.check_collision(trashes)
    hazardous_bin.draw()

    recyclable_bin.movement(keys)
    recyclable_bin.check_collision(trashes)
    recyclable_bin.draw()

def trashes_loop() -> None:
    # Loop backward to prevent skipping while deleting trashes.
    for i in range(len(trashes) - 1, -1, -1):
        trashes[i].movement()
        trashes[i].draw()

        if not trashes[i].is_alive():
            del trashes[i]

def show_score() -> None:
    screen.blit(font.render(f"General score: {general_bin.get_score()}", False, (255, 255, 255)), (50, 50))
    screen.blit(font.render(f"Organic score: {organic_bin.get_score()}", False, (255, 255, 255)), (350, 50))
    screen.blit(font.render(f"Hazardous score: {hazardous_bin.get_score()}", False, (255, 255, 255)), (650, 50))
    screen.blit(font.render(f"Recyclable score: {recyclable_bin.get_score()}", False, (255, 255, 255)), (950, 50))

while running:
    event_loop()

    keys = pygame.key.get_pressed()
    screen.fill((0, 0, 0))

    show_score()
    trash_bins_loop()
    trashes_loop()

    pygame.display.flip()
    clock.tick(data.MAX_FPS)

pygame.quit()
