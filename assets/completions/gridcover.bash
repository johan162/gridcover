_gridcover() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="gridcover"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        gridcover)
            opts="-o -i -z -r -l -W -H -s -x -y -v -p -k -b -t -c -m -d -P -S -Z -C -R -T -D -J -B -A -M -G -Q -q -f -F -a -U -X -h -V --args-write-file-name --args-read-file-name --step-size --radius --blade-len --grid-width --grid-height --cell-size --start-x --start-y --velocity --start-dir-x --start-dir-y --perturb --perturb-segment --perturb-segment-percent --stop-bounces --stop-time --stop-coverage --stop-simsteps --stop-distance --verbosity --parallel --random-seed --image-width-mm --image-height-mm --paper-size --track-center --show-progress --cutter-type --dpi --json-output --battery-run-time --battery-charge-time --map-file-name --show-gridlines --database-file --quiet --generate-frames --frame-rate --frames-dir --create-animation --animation-file-name --animation-speedup --hw-encoding --delete-frames --color-theme --wheel-slippage --slippage-probability --slippage-min-distance --slippage-max-distance --slippage-radius-min --slippage-radius-max --slippage-check-activation-distance --slippage-adjustment-step --wheel-inbalance --wheel-inbalance-radius-min --wheel-inbalance-radius-max --wheel-inbalance-adjustment-step --show-quad-tree --min-qnode-size --use-quad-tree --save-quad-tree --show-image-label --generate-json-files --generate-completions --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --args-write-file-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --args-read-file-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --step-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -z)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --radius)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --blade-len)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --grid-width)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -W)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --grid-height)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cell-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --start-x)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -x)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --start-y)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -y)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --velocity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -v)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --start-dir-x)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --start-dir-y)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --perturb)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --perturb-segment)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -k)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --perturb-segment-percent)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stop-bounces)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -b)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stop-time)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stop-coverage)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stop-simsteps)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --stop-distance)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --verbosity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --parallel)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -P)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --random-seed)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -S)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --image-width-mm)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --image-height-mm)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --paper-size)
                    COMPREPLY=($(compgen -W "a5 a4 a3 a2 a1 a0 letter legal tabloid executive custom" -- "${cur}"))
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                -Z)
                    COMPREPLY=($(compgen -W "a5 a4 a3 a2 a1 a0 letter legal tabloid executive custom" -- "${cur}"))
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --track-center)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --show-progress)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -R)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --cutter-type)
                    COMPREPLY=($(compgen -W "blade circular" -- "${cur}"))
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                -T)
                    COMPREPLY=($(compgen -W "blade circular" -- "${cur}"))
                    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
                        compopt -o nospace
                    fi
                    return 0
                    ;;
                --dpi)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -D)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --json-output)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -J)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --battery-run-time)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -B)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --battery-charge-time)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -A)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --map-file-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -M)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --show-gridlines)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -G)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --database-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -Q)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --quiet)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -q)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --generate-frames)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --frame-rate)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -F)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --frames-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --create-animation)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -a)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --animation-file-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --animation-speedup)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -U)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --hw-encoding)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --delete-frames)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --color-theme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wheel-slippage)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --slippage-probability)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --slippage-min-distance)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --slippage-max-distance)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --slippage-radius-min)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --slippage-radius-max)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --slippage-check-activation-distance)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --slippage-adjustment-step)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wheel-inbalance)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --wheel-inbalance-radius-min)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wheel-inbalance-radius-max)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wheel-inbalance-adjustment-step)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --show-quad-tree)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --min-qnode-size)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --use-quad-tree)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --save-quad-tree)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --show-image-label)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                --generate-json-files)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                -X)
                    COMPREPLY=($(compgen -W "true false" -- "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _gridcover -o nosort -o bashdefault -o default gridcover
else
    complete -F _gridcover -o bashdefault -o default gridcover
fi
