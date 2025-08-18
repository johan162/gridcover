#!/bin/bash
# filepath: completions/gridcover.bash

_gridcover_completions() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    # Main options
    opts="--grid-width --grid-height --square-size --map-file-name 
          --radius --blade-len --cutter-type --velocity 
          --start-x --start-y --dir-x --dir-y --perturb --perturb-segment 
          --perturb-segment-percent --track-center
          --wheel-slippage --slippage-probability --slippage-min-distance 
          --slippage-max-distance --slippage-radius-min --slippage-radius-max
          --slippage-check-activation-distance --slippage-adjustment-step
          --wheel-inbalance --wheel-inbalance-radius-min --wheel-inbalance-radius-max
          --wheel-inbalance-adjustment-step
          --step-size --stop-bounces --stop-time --stop-coverage 
          --stop-simsteps --stop-distance
          --battery-run-time --battery-charge-time
          --image-width-mm --image-height-mm --paper-size --dpi 
          --show-gridlines --color-theme --show-quad-tree --show-image-label
          --generate-frames --frame-rate --frames-dir --create-animation
          --animation-file-name --hw-encoding --delete-frames --animation-speedup
          --json-output --verbosity --show-progress --quiet --generate-json-files
          --random-seed --parallel --use-quad-tree --min-qnode-size 
          --save-quad-tree --generate-completion
          -W -H -s -M -r -l -T -v -x -y -p -k -C -z -b -t -c -m -d -B -A 
          -o -Z -D -G -f -F -a -U -J -R -q -X -S -P"
    
    case "${prev}" in
        --cutter-type|-T)
            COMPREPLY=( $(compgen -W "blade circular" -- ${cur}) )
            return 0
            ;;
        --paper-size|-Z)
            COMPREPLY=( $(compgen -W "A0 A1 A2 A3 A4 A5 Letter Legal" -- ${cur}) )
            return 0
            ;;
        --color-theme)
            COMPREPLY=( $(compgen -W "default green30 blue orange_red gray_green pure_green" -- ${cur}) )
            return 0
            ;;
        --perturb|-p|--perturb-segment|-k|--track-center|-C|--wheel-slippage|--wheel-inbalance)
            COMPREPLY=( $(compgen -W "true false" -- ${cur}) )
            return 0
            ;;
        --show-gridlines|-G|--show-quad-tree|--show-image-label|--generate-frames|-f)
            COMPREPLY=( $(compgen -W "true false" -- ${cur}) )
            return 0
            ;;
        --create-animation|-a|--hw-encoding|--delete-frames|--json-output|-J)
            COMPREPLY=( $(compgen -W "true false" -- ${cur}) )
            return 0
            ;;
        --show-progress|-R|--quiet|-q|--generate-json-files|-X|--parallel|-P)
            COMPREPLY=( $(compgen -W "true false" -- ${cur}) )
            return 0
            ;;
        --use-quad-tree|--save-quad-tree)
            COMPREPLY=( $(compgen -W "true false" -- ${cur}) )
            return 0
            ;;
        --verbosity)
            COMPREPLY=( $(compgen -W "0 1 2" -- ${cur}) )
            return 0
            ;;
        --map-file-name|-M)
            COMPREPLY=( $(compgen -f -X '!*.@(yaml|yml)' -- ${cur}) )
            return 0
            ;;
        --frames-dir)
            COMPREPLY=( $(compgen -d -- ${cur}) )
            return 0
            ;;
        -o|--animation-file-name)
            COMPREPLY=( $(compgen -f -- ${cur}) )
            return 0
            ;;
        --generate-completion)
            COMPREPLY=( $(compgen -W "bash zsh fish" -- ${cur}) )
            return 0
            ;;
    esac
    
    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
    return 0
}

complete -F _gridcover_completions gridcover
