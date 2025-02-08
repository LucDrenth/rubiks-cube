# TODO's

## Features
### Must have
- Implement solving algorithm(s).

### Nice to have
- Base slice interaction axis on the direction from which a piece face was entered
- Animation for updating cube size (scale down to 0 before despawn, scale up from 0 to 1 at spawn). Then remove scaling functionality with arrow keys, which is just an example
- Show full scramble sequence as text (make current move pop out)
- Automatically move the camera so the cube takes up the same amount of space on the screen, no matter the cube size
- Skybox (upgrade to bevy 15.2 for skybox fixes!)
- Some (rectangle shaped) platform underneath so it is not just floating in the air
- Algorithm with optimal solving strategy ('Gods algorithm'). This is expected to take about 10 seconds to calculate. To prevent lag it will have to be either done on a separate thread or spread out over multiple ticks
- Support 1x1 cube size


### Bugs
- Changing rotation speed during sequence makes the progress bar flicker when (nearly) done.
- Changing PIECE_SIZE to something other than 1 causes incorrect cube picking colliders. After some initial investigatino, the collider sizes looked good. Could be a miscalculation with the piece spawning positions
