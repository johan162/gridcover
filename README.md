# GridCover - Autonomous Lawn Mower Simulation

GridCover is a simulation program that models how an autonomous lawn mower cuts grass in a large rectangular area. The simulation tracks the circular cutting path of the mower as it moves across the lawn, bouncing off boundaries and changing direction to achieve complete grass coverage.

## Overview

This program simulates a robotic lawn mower with a circular cutting blade that moves across a rectangular lawn area. The mower follows a realistic physics model where:

- The mower moves in straight lines with option probability based perturberation until it hits a boundary
- Upon hitting a boundary, it bounces off at an angle (with optional random perturbation)
- The simulation tracks which areas of the lawn have been completely covered by the cutting blade
- The program generates both terminal output and PNG images showing the coverage pattern
- Two different type of cutter geometry can be used: Circular and blade A circular cutter models a traditoinal lawn mower where the knife covers the entire diameter of the rotating cutter. The blade types models modern robotic cutters where a 3-4cm knife is attached to the outer edge of a rotating disc.
- To give an accurate estimation of the simulated time an optional battery run-time and charge time can be specified (tis will also include a simulated time for the cutter to find its charging station)


## Features

- **High-performance simulation** with optional parallel processing using multiple CPU cores
- **Realistic physics** with configurable mower radius, speed, and direction perturbation
- **Multiple stopping conditions** (bounces, time, coverage percentage, distance, or simulation steps)
- **Visual output** with color-coded PNG images showing coverage patterns
- **Progress tracking** with optional progress bars and detailed statistics
- **Reproducible results** using random seeds for deterministic simulations
- **Flexible configuration** with extensive command-line options

## Installation from source

Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

Clone the repository and build the project:

```bash
git clone <repository-url>
cd gridcover
cargo build --release
```

## Usage

### Basic Usage from source

This assumes that the binaries have been built and placed in the path searched for binaries.

Run a basic simulation with default parameters:

```bash
gridcover --stop-coverage 50
```
or the equivalent with short arguments
```bash
gridcover -c 50
```

This will simulate a lawn mower with:
- A lawn 10x10 unit size (with default values for speed, radious etc. this is consistent with a 10x10m grid)
- The area will be divided in 400x400 cells (each 0.025 unit size)
- Each cell representing 0.025 unit × 0.025 units
- Run the simulation until 50% of the lawn has been cut
- Circular cutting size is 0.2 units (default)
- Random starting direction
- Random starting position
- Will write progress information during simulation
- Will print a short summary of simulation result


## Command Line Arguments

### Grid Configuration
- `-w, --width <WIDTH>` - Grid width in cells (default: 500)
- `-g, --height <HEIGHT>` - Grid height in cells (default: 500) 
- `-s, --square-size <SQUARE_SIZE>` - Size of each grid square in units (default: 0.1)

### Mower Configuration
- `-r, --radius <RADIUS>` - Radius of the circular cutting blade in units (default: 0.3)
- `-v, --velocity <VELOCITY>` - Movement velocity in units/second (default: 0.5)
- `-x, --start-x <START_X>` - Starting X coordinate for mower center (default: 0, randomized)
- `-y, --start-y <START_Y>` - Starting Y coordinate for mower center (default: 0, randomized)
- `--dir-x <DIR_X>` - Direction X component (default: 0, randomized)
- `--dir-y <DIR_Y>` - Direction Y component (default: 0, randomized)
- `-T, --cutter-type <CUTTER-TYPE>` Cutter type to use for the simulation. Options: 'blade', 'circular'. [default: blade]
- `--blade-len <BLADE_LEN>` - Length of knife blade in units (default: 0.05)
- `-B, --battery-run-time <BATTERY_RUN_TIME>` Battery duration in minutes for the cutter [default: 0]
- `-A, --battery-charge-time <BATTERY_CHARGE_TIME>` Battery charging time in minutes for the cutter when it runs out [default: 0]


### Stopping Conditions
At least one stopping condition must be specified:

- `-b, --stop-bounces <BOUNCES>` - Stop after this many wall bounces (default: 0, disabled)
- `-t, --stop-time <TIME>` - Stop after simulated time in seconds (default: 0, disabled)
- `-c, --stop-coverage <COVERAGE>` - Stop when coverage percentage reached (1-99%) (default: 0, disabled)
- `-m, --stop-simsteps <STEPS>` - Stop after this many simulation steps (default: 0, disabled)
- `-d, --stop-distance <DISTANCE>` - Stop after mower travels this distance (default: 0, disabled)

### Output Configuration
- `-o [IMAGE-FILE-NAME]` - Output PNG image filename (default: coverage_grid.png)
- `-Z, --paper-size <PAPER-SIZE>` - Paper size to use for the output image, (A0,A1,A2,A3,A4,A5,Letter,Legal) (default: a4)
- `--image-width <WIDTH>` - Output image width in mm (50-500, default: 200)
- `--image-height <HEIGHT>` - Output image height in mm (50-500, default: 200)
- `--dpi <DPI>` - Output image DPI (default: 300)
- `-C, --track-center <TRACK_CENTER>` Add option to turn centerpoint tracking on or off [default: true] [possible values: true, false]
- `-J, --json-output <JSON_OUTPUT>` Print results as a json object [default: false] [possible values: true, false]
- `--verbosity <LEVEL>` - Verbosity level 0-2 (default: 0)
  - 0: Minimal output with only result shown
  - 1: Show simulation parameters and results + additional info
  - 2: Include an ASCII version to count the initial legs color coded and numbered (only applicable for grids ≤100×100)

### Simulation Parameters
- `-z, --step-size <STEP_SIZE>` - Simulation step size in units if not specified will be calculated from the square size (default: 0)
- `-S, --random-seed <SEED>` - Random seed for reproducible results (default: 0, random)
- `-p, --perturb <PERTURB>` - Enable random angle perturbation on bounces (default: true)
- `--perturb-segment <PERTURB_SEGMENT>` Use perturbation randomly while moving in a straight line [default: false] [possible values: true, false]
- `--perturb-segment-percent <PERTURB_SEGMENT_PERCENT>` Perturb segment percent chance per cell travelled
- `args-write-file-name [ARGS-FILE-NAME]` - Write program arguments file in TOML format
- `-i, args-read-file-name [ARGS-FILE-NAME]` - Read program arguments from a TOML file, arguments also specified on the command line will override the file


### Performance and Behavior Options
- `-P, --parallel <PARALLEL>` - Enable parallel processing (default: true)
- `-C, --track-center <TRACK_CENTER>` - Track mower center position in image (default: true)
- `-R, --show-progress <SHOW_PROGRESS>` - Show progress bar during simulation (default: true)
- `-o <IMAGE-FILE-NAME>` - Output image file name

## Understanding the Output

### Terminal Output
The default output is only a short summary of the simulation results:
- Time for the lawn mover (in simulation time to reach the stop condition)
- The theoretical minimum time to over the area
- Coverage
- Distance traveled
- Number of bounces, how many time the cutter hit the edge and reverted direction
- Battery information, (not enabled in this example)


**Example Text Output**
```txt
$ gridcover -c 50
Coverage:  50.02% (  80035/ 160000 cells covered), Distance: 190.80, Bounces:   32, Sim-Time: 00:10:35

Simulation Results:
  Simulation time          : 00:00
  Simulated elapsed time   : 0:10:35
  Theoretical minimum time : 0:06:57
  Coverage                 : 50.0% (80,035 out of 160,000 cells)
  Distance traveled:       : 190.8 units
  Number of bounces:       : 32
  Battery Charge Count     : 0
  Battery Charge Left      : 100.0%
  Total Simulation Steps   : 12,720 (Step size = 0.01 units, Sim steps/cell = 0.60)
```

To get a more complete output you can enable output in JSON format. This will give all simulation results togeher will all important simulation parameters

**Example JSON Output**
```txt
$ gridcover -c 50 -J true
Coverage:  50.01% (  80012/ 160000 cells covered), Distance: 194.40, Bounces:   32, Sim-Time: 00:10:47
{
  "Simulation Result": {
    "Coverage": {
      "Bounce count": 32,
      "Count": 80012,
      "Percent": 50.00750000000001
    },
    "Cutter": {
      "Battery": {
        "Charge count": 0,
        "Charge left (percent)": 100.0,
        "Charge time": 120.0,
        "Run time": 0.0
      },
      "Blade Length": 0.05,
      "Cells covered": 256.0,
      "Distance": 194.39999999994407,
      "Radius": 0.2,
      "Type": "blade",
      "Velocity": 0.3
    },
    "Grid": {
      "Area": 160000,
      "Cell size": 0.025,
      "Cells x": 400,
      "Cells y": 400,
      "Height": 10.0,
      "Width": 10.0
    },
    "Output image": {
      "DPI": 300,
      "File name": "",
      "Paper size": {
        "height_mm": 297.0,
        "paper_size": "A4",
        "width_mm": 210.0
      },
      "Pixels": {
        "height": 3508,
        "width": 2480
      }
    },
    "Start": {
      "Angle (degrees)": 30.366393199230345,
      "Direction": {
        "X": 0.8628103345831091,
        "Y": 0.5055277702921802
      },
      "Position": {
        "X": 2.3411700813678884,
        "Y": 1.8522634664794135
      }
    },
    "Steps": {
      "Per cell": 0.6,
      "Size": 0.015,
      "Total": 12960
    },
    "Time": {
      "Efficiency": 64.35,
      "Min. Time": "00:06:57",
      "Real": "00:00",
      "Simulation": "00:10:47"
    }
  }
}
```


### Visual Output
The generated PNG image uses a color-coded system:
- **Dark grey areas**: Uncovered lawn areas
- **Colored areas**: Covered areas, with different shades representing how often a spot has been visited. The darker the cell the more visits
- **White dots**: Mower center positions (if tracking enabled)

The image is scaled to the specified dimensions (default A4 size) suitable for printing or analysis. Note since the program is not a image processing program the chosen output size must be large enough to have enough pixels to match the number of cells

### Grid Display (Terminal)
For small grids (≤100×100 cells) with verbosity level 2, a text representation is shown:
- `--`: Uncovered cells
- `0-9`: Covered cells (number indicates bounce phase)
- `*`: Mower center positions

## Performance Considerations

- **Parallel processing**: Enabled by default, uses multiple CPU cores for faster simulation
- **Grid size**: Larger grids require more memory and computation time
- **Step size**: Automatically calculated based on mower radius for optimal accuracy
- **Release builds**: Use `--release` flag for better performance


## Limitations
- Partial covered cells are not counted, only fully covered cells are counted as cut. With small enough cells this is not an issue.
- Memory usage is O(WxH) as the whole grid is in memory

## License

MIT License

# Example output

## Without center tracking

Using defaults and stop simulation at 50% coverage

```bash
gridcover -c 90 -o assets/coverage-c50.png
```
<img src="assets/coverage-c50.png" width="300">

Using defaults and stop simulation at 99% coverage

```bash
gridcover -c 99 -o assets/coverage-c99.png
``` 
<img src="assets/coverage-c99.png" width="300">


## With center point tracking

Draw the center line of travel to make it obvious how the cutter have moved.
Stop after 10 bounces.

```bash
gridcover -b 10 -C true -o assets/coverage-b10-Ctrue.png
```

<img src="assets/coverage-b10-Ctrue.png" width="300">


Stop after travelling 500 units


```bash
gridcover -d 500 -C true -o assets/coverage-d500-Ctrue.png
```
<img src="assets/coverage-d500-Ctrue.png" width="300">


Stop after 100 units of traveled distance (100m) and enabling random perturbation
while running between boundary conditions

```bash
gridcover -d 100 -k true -C true -o assets/coverage-d100-ktrue-Ctrue.png
```

<img src="assets/coverage-d100-ktrue-Ctrue.png" width="300">



# Full list of command line options

```txt
Usage: gridcover [OPTIONS]

Options:
  -o <IMAGE-FILE-NAME>
          Output image file name
      --args-write-file-name <ARGS-FILE-NAME>
          Write program arguments file in TOML format
  -i, --args-read-file-name <ARGS-FILE-NAME>
          Read program arguments from a TOML file
  -z, --step-size <STEP_SIZE>
          Simulation step size in units if not specified will be calculated from the square size [default: 0]
  -r, --radius <RADIUS>
          Radius of the circle [default: 0.2]
  -l, --blade-len <BLADE_LEN>
          Length of knife blade [default: 0.05]
  -W, --grid-width <GRID_WIDTH>
          Width in units of the grid [default: 0]
  -H, --grid-height <GRID_HEIGHT>
          Height in units of the grid [default: 0]
  -s, --square-size <SQUARE_SIZE>
          Size of each grid square [default: 0]
  -x, --start-x <START_X>
          Starting X coordinate for the circle center [default: 0]
  -y, --start-y <START_Y>
          Starting Y coordinate for the circle center [default: 0]
  -v, --velocity <VELOCITY>
          Movement velocity in units/second [default: 0.3]
      --dir-x <DIR_X>
          Direction X component [default: 0]
      --dir-y <DIR_Y>
          Direction Y component [default: 0]
  -p, --perturb <PERTURB>
          Use perturbation angle for direction changes at bounce [default: true] [possible values: true, false]
  -k, --perturb-segment <PERTURB_SEGMENT>
          Use perturbation randomly while moving in a straight line [default: false] [possible values: true, false]
      --perturb-segment-percent <PERTURB_SEGMENT_PERCENT>
          Perturb segment percent chance per cell travelled [default: 0.5]
  -b, --stop-bounces <STOP_BOUNCES>
          Maximum number of bounces before ending simulation [default: 0]
  -t, --stop-time <STOP_TIME>
          Maximum simulated time when to stop in seconds [default: 0]
  -c, --stop-coverage <STOP_COVERAGE>
          Stop when we have reached this coverage percentage This is a soft limit, the simulation will still run until the specified bounces or time is reached if specified [default: 0]
  -m, --stop-simsteps <STOP_SIMSTEPS>
          Stop when we have reached the specified number of simulation steps [default: 0]
  -d, --stop-distance <STOP_DISTANCE>
          Stop when we have reached the specified distance covered [default: 0]
      --verbosity <VERBOSITY>
          Verbosity during simulation [default: 0]
  -P, --parallel <PARALLEL>
          Use parallel processing to speed up simulation [default: false] [possible values: true, false]
  -S, --random-seed <RANDOM_SEED>
          Random seed for the simulation to be able to reproduce results If not specified, a random seed will be generated [default: 0]
      --image-width-mm <IMAGE_WIDTH_MM>
          Image output width in mm (50-2000) [default: 210]
      --image-height-mm <IMAGE_HEIGHT_MM>
          Image output height in mm (50-2000) [default: 297]
  -Z, --paper-size <PAPER-SIZE>
          Paper size to use for the output image. Options: 'A0', 'A1', 'A2', 'A3', 'A4', 'A5', 'Letter', 'Legal'. [default: a4] [possible values: a5, a4, a3, a2, a1, a0, letter, legal, tabloid, executive, custom]
  -C, --track-center <TRACK_CENTER>
          Add option to turn centerpoint tracking on or off [default: false] [possible values: true, false]
  -R, --show-progress <SHOW_PROGRESS>
          Show progress bar during simulation (default: true) [default: true] [possible values: true, false]
  -T, --cutter-type <CUTTER-TYPE>
          Cutter type to use for the simulation. Options: 'blade', 'circular'. [default: blade] [possible values: blade, circular]
  -D, --dpi <DPI>
          DPI setting for image output (default: 300) [default: 300]
  -J, --json-output <JSON_OUTPUT>
          Print results as a json object [default: false] [possible values: true, false]
  -B, --battery-run-time <BATTERY_RUN_TIME>
          Battery duration in minutes for the cutter [default: 0]
  -A, --battery-charge-time <BATTERY_CHARGE_TIME>
          Battery charging time in minutes for the cutter when it runs out [default: 120]
  -h, --help
          Print help
  -V, --version
          Print version
```