# Shader_window
Overlay applicaiton which adds custom effects to the desktop.

## Controls
| Key | Description |
|---|---|
| `Ctrl + Alt + C` | Toggle capture and image modes |
| `Ctrl + Alt + O` | Toggle overlay mode on/off |
| `Ctrl + Alt + S` | Toggle share window on/off |
| `Ctrl + Alt + N + I` | Switch to next test image |
| `Ctrl + Alt + N + S` | Switch to next shader |

*`Ctrl` and `Alt` refer to the left versions of the keys.*

## Inspired by:
- [Shader Glass](https://store.steampowered.com/app/3613770/ShaderGlass/) : Tool for applying shader effects on top of Windows desktop for gaming, pixel art and video. Made by Mausimus, available on Steam.
- [Acerola](https://www.youtube.com/@Acerola_t) : Professional shader artist, graphics programmer, and game developer.

## Worked on:
[Adam](https://github.com/Adam-osc) :
- **Shader**: Colored weighted Voronoi stippling effect

[Matej](https://github.com/MatejBursik) :
- **App development**: Implemented prototype application for testing shaders on images
- **App development** : Desktop capture - only below the app window (Windows and MacOS solution)
- **Shader**: Colored ASCII art style effect with edge enhancement ([reference](https://www.youtube.com/watch?v=gg40RWiaHRY))
- **Shader**: Sobel edge detection effect
- **Shader**: Pixel art style effect

## TODO:
- **App development** : Toolbar menu / Update controls
- **App development** : Desktop capture (Linux solution)
- **App development** : Share window (toggleable hidden secondary window which can interact with sharing/recording software)
- **Shader** : CRT effect
- **Shader** : Difference of Gaussians ([reference](https://www.youtube.com/watch?v=5EuYKEvugLU))
- **Shader** : Cross hatch art style effect
