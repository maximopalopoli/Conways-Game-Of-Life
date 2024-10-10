# Conways-Game-Of-Life

An implementation in Rust of the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) made by John Conway.

To run the game, use:

`cargo run start`

Or, if you want to use a predefined seed, you can use:

`cargo run start 1x 1y 2x 2y ... Nx Ny`

Where [(1x, 1y), (2x, 2y) ... (Nx, Ny)] will be the coordinates for the alive cells in the initial seed. Note that arguments after 'start' should be pairs.

Once the program is executed, you'll see the next interface:

![Initial-view](/img/initial-display.png "Initial interface")

Now, you'll see the passed points of seed if you passed it. Anyway, you can click on the cells you want to be alive or dead (a black square represents an alive cell).

To the right of the table you will see the folowing menu of options:

![Menu](/img/menu-options.png "Menu options")

The options have the following meaning:
- Next generation is used to advance the generation manually
- Automatic advance is used to have the program advance automatically (Should define a time between iterations before)
- Stop automatic advance to stop the automatic advance
- Clear to reset the grid and iterations
- You can set the time between iterations with the slider showed (the value must be between 0 and 5 seconds)

### Example of execution
Once you've set the seed, with the automatic advance option the generation will advance when the time between iterations pass.

![Cycle](/img/lifecycle.gif "Cycle example")
