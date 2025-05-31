import pygame
import data
import random

# Define colors
colors = {
    "red": (255, 0, 0), 
    "green": (0, 255, 0),
    "blue": (0, 0, 255),
    "yellow": (255, 255, 0),
    "purple": (128, 0, 128),
}

pygame.init()
clock = pygame.time.Clock()

# For credits
Creators = ["Isaac", "Pakthan", "Taj"]
IncrementName = pygame.USEREVENT + 1
ColorChange = pygame.USEREVENT + 2

pygame.time.set_timer(IncrementName, 1000)
pygame.time.set_timer(ColorChange, 1000)

currentCreator = 0
Text_color = (255, 255, 255)

# Title screen
Title = True
pygame.display.set_caption("Title Screen")
screen = pygame.display.set_mode((0, 0), pygame.FULLSCREEN)
ScreenSize = screen.get_size()
ScreenCenter = (ScreenSize[0] // 2, ScreenSize[1] // 2)

hoverd = False
TitleNormal = pygame.transform.scale(pygame.image.load(data.TitleNormal).convert_alpha(), screen.get_size())

# Button setup
class Sprite:
    def __init__(self, path: str, pos: tuple[int, int], scale: tuple[int, int]) -> None:
        self.image = pygame.transform.scale(pygame.image.load(path), scale)
        self.rect = self.image.get_rect(center=pos)

# Change the button position.
button_hover = Sprite(data.BUTTON_HOVER_IMG_PATH, (600, 600), (200, 100))
button_unpress = Sprite(data.BUTTON_UNPRESS_IMG_PATH, (600, 600), (200, 100))
button_pressed = Sprite(data.BUTTON_FULL_PRESS_IMG_PATH, (600, 600), (200, 100))

pressed = False

# Fade setup
fade_surface = pygame.Surface(ScreenSize)
fade_surface.fill((0, 0, 0))
fade_alpha = 0
fade_speed = 5

# Credits text setup
font = pygame.font.Font(data.Font, 70)

while Title:
    mouse = pygame.mouse.get_pos()
    hoverd = button_unpress.rect.collidepoint(mouse)

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            Title = False
        elif event.type == pygame.MOUSEBUTTONDOWN:
            if hoverd:
                pressed = True
        elif event.type == IncrementName:
            currentCreator = (currentCreator + 1) % len(Creators)
        elif event.type == ColorChange:
            Text_color = random.choice(list(colors.values()))

    screen.blit(TitleNormal, (0, 0))
    text = font.render(f"$ {Creators[currentCreator]}", True, Text_color)
    screen.blit(text, (500, 100))

    if not pressed:
        screen.blit(button_hover.image if hoverd else button_unpress.image, button_unpress.rect)
    else:
        screen.blit(button_pressed.image, button_pressed.rect)

        if fade_alpha < 255:
            fade_alpha += fade_speed
            fade_surface.set_alpha(fade_alpha)
            screen.blit(fade_surface, (0, 0))
        else:
            Title = False  # Fade completed

    pygame.display.flip()
    screen.fill((0, 0, 0))
    clock.tick(60)

