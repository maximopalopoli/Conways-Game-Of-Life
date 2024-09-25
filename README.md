# Conways-Game-Of-Life

An implementation in Rust of the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) made by John Conway. This is work in progress.

To run the game, use:

`cargo run start 1x 1y 2x 2y ... Nx Ny`

Where `[(1x, 1y), (2x, 2y) ... (Nx, Ny)]` will be the coordinates for the alives cells in the initial seed.

The start functionality will print the status of the current generation, and advance to the next generation every 3 seconds.

In the next versions, it will be easier to select the points, via UI.
