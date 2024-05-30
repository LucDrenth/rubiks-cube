- Fix rotations when cube is rotated in another way than CubeRotationEvent. 
  Possible solution may be to create invisible parent Transforms for the middle + each of the 6 faces. We can not use
  the middle piece of the faces because it will not work for even sized cubes (2x2, 4x4 etc.).
- Handle other sizes than 3x3
- Animated rotations
- Move cube configs to a struct instead of having them as const values
- Implement scrambling algorithm
- Implement solving algorithms
