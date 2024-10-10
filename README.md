# Conways-Game-Of-Life

An implementation in Rust of the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) made by John Conway.

### Usage
There is a Makefile to simplify the usage of the program. Some options are:

- `make test` to execute the tests

- `make clippy` to execute clippy

- `make start` to run the game without a seed

- `make seed coords="<coords>"`to tun the game with a seed. Parameter `<coords>` will be the coordinates for the alive cells in the initial seed, note that the numbers that make the coordinates should be pairs. An example of usage can be:
`make seed coords="10 10 10 9 10 11 9 10 11 10"`


### Interface
Once the program is executed, you'll see the next interface:

![Initial-view](/img/initial-display.png "Initial interface")

Now, you'll see the passed points of seed if you passed it. Anyway, you can click on the cells to give them life or click again to kill them (a black square represents an alive cell).

To the right of the table you will see the folowing menu of options:

![Menu](/img/menu-options.png "Menu options")

The options have the following meaning:
- Next generation is used to advance the generation manually
- Automatic advance is used to have the program advance automatically (Should define a time between iterations before)
- Stop automatic advance to stop the automatic advance
- Clear to reset the grid and iterations
- You can set the time between iterations with the slider showed (the value must be between 0 and 5 seconds)
- Grid change will show the last cell selected, whether to give life or to kill

### Example of execution
Once you've set the seed, with the automatic advance option the generation will advance when the time between iterations pass.

An example of an short iteration of the game is the following:

![Cycle](/img/lifecycle.gif "Cycle example")

Note that since 8th generation the next generation is se same as the previous. That is called a pattern, and there is a lot of them, you can try to find them in the game!
