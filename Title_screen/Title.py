import pygame
import TitleData

pygame.init()
# The Creators
Creators = [
    "Isaac",
    "Pakthan",
    "Taj"]
IncrementName = pygame.USEREVENT + 1
pygame.time.set_timer(IncrementName, 1000)  # Increment every second
currentCreator = 0

Title = True
pygame.display.set_caption("Title Screen")
screen = pygame.display.set_mode((0,0),pygame.FULLSCREEN)
ScreenSize = screen.get_size()
ScreenCenter = (ScreenSize[0] // 2, ScreenSize[1] // 2)
print(ScreenSize)


hoverd = False
TitleNormal = pygame.transform.scale(pygame.image.load(TitleData.TitleNormal).convert_alpha(),screen.get_size())

#button
button_image = pygame.image.load(TitleData.Title + r"\Button\NotHoverButton.png").convert_alpha()
button = button_image.get_rect()
button.center = (ScreenCenter[0], 600)  # Center of the button
#hoverbutton
button_hover_image = pygame.image.load(TitleData.Title + r"\Button\HoverButton.png").convert_alpha()
buttonHover = button_hover_image.get_rect()
buttonHover.center = (ScreenCenter[0], 600)  # Center of the button

#credits
font = pygame.font.Font(TitleData.Font, 70)
text = font.render(f"$ {Creators[currentCreator]}", True, (255, 255, 255))
text_rect = text.get_rect()
text_rect.topleft = (500, 100)

"""
2.1 for x
1.7 for y
# button corner points
(414, 338)
(414, 428)
(819, 339)
(820, 426)

(613, 415)

(895, 628)

(1016, 571)
"""


def is_point_in_rectangle(x, y, x1, y1, x2, y2):
  return x1 <= x <= x2 and y1 <= y <= y2

while Title:
  mouse = pygame.mouse.get_pos()
  if is_point_in_rectangle(mouse[0], mouse[1], button.topleft[0], button.topleft[1], button.bottomright[0], button.bottomright[1]):
    #if the mouse is over the button
    hoverd = True
  else:
    #if the mouse is not over the button
    hoverd = False

  screen.blit(TitleNormal, (0, 0))
  if hoverd:
    screen.blit(button_hover_image, buttonHover)
  else:  
    screen.blit(button_image, button)
    
  for event in pygame.event.get():
    if event.type == pygame.QUIT:
      Title = False
    elif event.type == pygame.MOUSEBUTTONDOWN:
      #uses this to find the corners of the buttons
      print("Mouse clicked at:", mouse)
      if hoverd:
        print("Button clicked!")
        #Here you can add the code to start the game or go to the next screen
        Title = False
    elif event.type == IncrementName:
      # Increment the current creator index
      currentCreator += 1
      if currentCreator >= len(Creators):
        currentCreator = 0
      # Update the text to display the current creator
  screen.blit(TitleNormal, (0, 0))
  if hoverd:
    screen.blit(button_hover_image, buttonHover)
  else:  
    screen.blit(button_image, button)
  
  text = font.render(f"$ {Creators[currentCreator]}", True, (255, 255, 255))
  text_rect.topleft = (500, 100)
  screen.blit(text, text_rect)

  pygame.display.flip()
  screen.fill((0, 0, 0))
  