# Conways-Game-Of-Life

An implementation in Rust of the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) made by John Conway.

To run the game, use:

`cargo run start`

Or, if you want to use a predefined seed, you can use:

`cargo run start 1x 1y 2x 2y ... Nx Ny`

Where [(1x, 1y), (2x, 2y) ... (Nx, Ny)] will be the coordinates for the alives cells in the initial seed.

Once the program is opened, you can click on the cells you want to choose as seeds, and by clicking on live cells you can kill them.

The interface options are:
- Next generation to advance the generation manually
- Automatic advance to have the program advance automatically (it is recommended to define a time between iterations before)
- Stop automatic advance to stop the automatic advance
- Clear to reset the table completely
