import pygame
import TitleData
Title = True
pygame.init()
pygame.display.set_caption("Title Screen")
screen = pygame.display.set_mode((1280, 720))

hoverd = False
TitleNormal = pygame.image.load(TitleData.TitleNormal)
TitleHover = pygame.image.load(TitleData.TitleHover)

"""
# button corner points
(414, 338)
(414, 428)
(819, 339)
(820, 426)
"""


while Title:
  mouse = pygame.mouse.get_pos()
  if mouse[0] >= 414 and mouse[0] <= 819 and mouse[1] >= 338 and mouse[1] <= 426:
    hoverd = True
  else:
    hoverd = False
  for event in pygame.event.get():
    if event.type == pygame.QUIT:
      Title = False
    elif event.type == pygame.MOUSEBUTTONDOWN:
      #uses this to find the corners of the buttons
      print(mouse)
      if hoverd:
        Title = False
  if hoverd:
    screen.blit(TitleHover, (0, 0))  
  if not hoverd:
    screen.fill((0, 0, 0))
    screen.blit(TitleNormal, (0, 0))
  pygame.display.flip()
  
