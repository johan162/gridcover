# GridCover - Autonomous Lawn Mower Simulation

## Overview
GridCover is a simulation program that models how an autonomous lawn mower, or "cutter", cuts grass in a rectangular area with optionally placed obstacles. The simulation tracks the cutter as it moves across the area, bouncing off boundaries and specified obstacles and changing direction according to the defined strategy to achieve complete grass coverage. 

The simulation is a close physical model of the cutting mechanism of the two most common types of robotic cutter. Currently the bounding area is a rectangle where an optional obstacle-map can be added with a user-specified number of obstacles. Obtsacles can be shaped like reactangles, arbitrary polygons, circles or line with a thickness. The simulation is accurate down to a chosen cell size (usually somewhere around 1cm) and hence will very accurately simulate real time. The performance is such that it is possible to simulate a 100x100m rectangle with 1cm cell size in slightly less than 3min on a desktop computer.

The complete simulation can be viewed as a cretaed animation with a user specifeid fps or as an image of the completed simulation (or both)


### Main features

- **Realistic physics**, with configurable cutter detaiks like mower radius, speed, and direction perturbation
- **Multiple stopping conditions**, bounces, time, coverage percentage, distance, or simulation steps
- **Visual output** with color-coded PNG images showing coverage patterns os as a animated video of the entire simulation
- **Reproducible results** using random seeds for deterministic simulations
- **Flexible configuration** with extensive command-line options


## Installation

### MacOS

The distribution have pre-built packages for both Apple silicon and Intel architecture.
These packages can be installed either from *Finder* by double-clicking on the file or the command line with:

```bash
installer -pkg <pkg-name>
```

by default the program wil be installed in `/usr/local/bin`

### Linux

See the user guide for installation from source


### Windows

See the user guide for installation from source

# Full list of command line options

See user guide for categorization of these options together with detailed explanations.

```txt
Grid coverage simulation

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
          Size of each grid square [default: -1]
  -x, --start-x <START_X>
          Starting X coordinate for the circle center [default: -1]
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
          Show progress bar during simulation (default: true) [default: false] [possible values: true, false]
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
  -M, --map-file-name <MAP-FILE>
          Path to map file with obstacles
  -G, --show-gridlines <SHOW_GRIDLINES>
          Show or hide gridlines in the output image [default: false] [possible values: true, false]
  -Q, --database-file <DATABASE-FILE>
          Store simulation results and model parameters in SQLite database file
  -q, --quiet <QUIET>
          Quiet, no output at all [default: false] [possible values: true, false]
  -f, --generate-frames <GENERATE_FRAMES>
          Generate frames for an animation [default: false] [possible values: true, false]
  -F, --frame-rate <FRAME_RATE>
          Specify frame-rate for the animation [default: 10]
      --frames-dir <FRAMES-DIR>
          [default: frames_dir]
  -a, --create-animation <CREATE_ANIMATION>
          Generate an animation video from the frames [default: false] [possible values: true, false]
      --animation-file-name <ANIMATION-FILE-NAME>
          Animation file name [default: cutter_sim.mp4]
      --hw-encoding <HW_ENCODING>
          Use HW assisted encoding for the animation. This is only available on macOS and Linux [default: true] [possible values: true, false]
      --delete-frames <DELETE_FRAMES>
          Delete frames after animation has been created [default: false] [possible values: true, false]
  -h, --help
          Print help
  -V, --version
          Print version
```