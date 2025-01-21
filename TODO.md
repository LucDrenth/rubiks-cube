# TODO's

## Features
### Must have
- Implement solving algorithm(s).

### Nice to have
- Animation for updating cube size (scale down to 0 before despawn, scale up from 0 to 1 at spawn)
- Show button to decrease cube size as disabled when cube size is 2
- UI button to scramble instantly (without animations)
- Automatically move the camera so the cube takes up the same amount of space on the screen, no matter the cube size.
- Better lighting
- Sykbox
- Some rectangle underneath so it is not just floating in the air
- Algorithm with optimal solving strategy ('Gods algorithm'). This is expected to take about 10 seconds to calculate. To prevent lag it will have to be either done on a separate thread or spread out over multiple ticks.

## Issues
- Full cube rotation causes lag with cube siize >= 9. Slice rotation does not seem to cause any lag. Full cube rotation should not be that much harder to compute than clibe rotation, so this must be a bug.
