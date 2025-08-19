#!/bin/bash
# shellcheck disable=SC2034  # For color variables that might appear unused
# shellcheck disable=SC2181  # For checks on command exit status
# This script converts TTF font files to Rust array files for embedding in the application.
# This way we can use fonts without having to load external font files at runtime which is a
# hassle since we don't know what the user has installed on their system.
# JP 2025-08-08

# Warn about using undefined variables and exit on error
set -o errexit -o nounset -o pipefail

# Script Configuration
readonly SCRIPT_VERSION="1.2"
readonly DEFAULT_COLUMNS=80
readonly MIN_COLUMNS=10
readonly MAX_COLUMNS=100

# DejaVu Font Configuration
readonly DEJAVU_VERSION="2.37"
readonly _DEJAVU_VERSION=${DEJAVU_VERSION//./_} # Version for URL
readonly DEJAVU_URL="https://github.com/dejavu-fonts/dejavu-fonts/releases/download/version_${_DEJAVU_VERSION}/dejavu-fonts-ttf-${DEJAVU_VERSION}.tar.bz2"
readonly DEJAVU_TTF_FONTS=("DejaVuSans.ttf" "DejaVuSans-Bold.ttf")

# Directory Configuration  
# The font directory is where the TTF font files are located or will be downloaded to.
readonly FONT_DIR="../assets/fonts"
# The output directory is where the generated Rust files will be saved.
readonly DEFAULT_OUTPUT_DIR="../src/image"

# The name of the Rust font files and the name of the variables they define will be dynamically
# created based on the TTF font file names. See the function `create_rust_variable_name`.
declare -a RUST_FONT_FILES
declare -a RUST_VARIABLES

RUST_FONT_FILES=()
RUST_VARIABLES=()

# ============================================================
# Default global variables for the options given to the script
# ============================================================
verbose=false
quiet=false
debug=false
no_color=false
no_glyphs=false
opt=""
columns="${DEFAULT_COLUMNS}"
output_dir="${DEFAULT_OUTPUT_DIR}"
font_dir="${FONT_DIR}"

# ============================================================
# Configuration and Setup Functions
# ============================================================

# Color Constants
Green="\033[32m"
GreenBold="\033[1;32m"
Cyan="\033[36m"
CyanBold="\033[1;36m"
Gray="\033[30m"
GrayBold="\033[1;30m"
Red="\033[31m"
RedBold="\033[1;31m"
Yellow="\033[33m"
YellowBold="\033[1;33m"
White="\033[37m"
Blue="\033[34m"
BlueBold="\033[1;34m"
Magenta="\033[35m"
MagentaBold="\033[1;35m"
ResetColor="\033[0m"

# Check if no color should be used in the output
check_no_color() {
    if [[ "$no_color" = true ]]; then
        Green=""
        GreenBold=""
        Cyan=""
        CyanBold=""
        Gray=""
        GrayBold=""
        Red=""
        RedBold=""
        Yellow=""
        YellowBold=""
        White=""
        Blue=""
        BlueBold=""
        Magenta=""
        MagentaBold=""
        ResetColor=""
    fi
}

# Define glyphs for logging output + one space
ERROR_GLYPH="❌ "
SUCCESS_GLYPH="✅ "
WARNING_GLYPH="⚠️ "
NOTICE_GLYPH="🔔 "
INFO_GLYPH="ℹ️  "
DEBUG_GLYPH="🐞 "
HEADER_GLYPH="🔤 "

# Check if no glyphs should be used in the output
check_no_glyphs() {
    if [[ "$no_glyphs" = true ]]; then
        ERROR_GLYPH=""
        SUCCESS_GLYPH=""
        WARNING_GLYPH=""
        NOTICE_GLYPH=""
        INFO_GLYPH=""
        DEBUG_GLYPH=""
        HEADER_GLYPH=""
    fi
}

# Logging function for all script output
log() {
    local level="$1"
    shift
    local message="$*"

    case "$level" in
      "ERROR")   [[ "$quiet" = false ]] && printf '%b\n' "${ERROR_GLYPH}${RedBold}Error: ${message}${ResetColor}" >&2 ;;
      "WARN")    [[ "$quiet" = false ]] && printf '%b\n' "${WARNING_GLYPH}${YellowBold}Warning: ${message}${ResetColor}" >&2 ;;
      "NOTICE")  [[ "$quiet" = false ]] && printf '%b\n' "${NOTICE_GLYPH}${Magenta}Notice: ${message}${ResetColor}" ;;
      "SUCCESS") [[ "$quiet" = false ]] && printf '%b\n' "${SUCCESS_GLYPH}${GreenBold}${message}${ResetColor}" ;;
      "INFO")    [[ "$verbose" = true && "$quiet" = false ]] && printf '%b\n' "${INFO_GLYPH}${Gray}${message}${ResetColor}" ;;
      "DEBUG")   [[ "$debug" = true ]] && printf '%b\n' "${DEBUG_GLYPH}${Cyan}${message}${ResetColor}" ;;
    esac
    return 0
}

# Check for required tools needed by the script
check_dependencies() {
    local -a missing=()

    command -v xxd >/dev/null || missing+=("xxd")
    command -v curl >/dev/null || missing+=("curl")
    command -v tar >/dev/null || missing+=("tar")
    command -v cmp >/dev/null || missing+=("cmp")

    if [[ ${#missing[@]} -gt 0 ]]; then
        printf '%b\n' "${ERROR_GLYPH}${RedBold}Error: Missing required commands: ${missing[*]}${ResetColor}" >&2
        exit 1
    fi
    [[ "$verbose" = true && "$quiet" = false ]] && printf '%b\n' "${INFO_GLYPH}${Gray}All required commands are available.${ResetColor}"
    return 0
}

# Show help message
show_help() {
    cat <<EOF
Generate Rust static data to embed font data in the application from TTF font files.
If font files are not present, they will be downloaded from the DejaVu Fonts repository.
Usage: $0 [OPTIONS]

OPTIONS:
    -h               Show this help message
    -o <output_dir>  Specify the output directory for the generated Rust files (default: ../src/image)
    -v               Show script version
    -V               Show verbose output
    -q               Suppress output
    -c <columns>     Specify the number of hex data columns in the dump (default: 80)
    -n               No ANSI color in output messages
    -g               No glyphs in output messages
EOF
    exit 0
}

# Add trap for cleanup
# shellcheck disable=SC2329
cleanup() {
    [[ -n "${temp_dir:-}" ]] && [[ -d "$temp_dir" ]] && rm -rf "$temp_dir"
}
trap cleanup EXIT

# Utility function to validate numeric ranges
validate_numeric_range() {
    local value="$1"
    local min="$2" 
    local max="$3"
    local name="$4"
    
    if ! [[ "$value" =~ ^[0-9]+$ ]] || [[ "$value" -lt "$min" ]] || [[ "$value" -gt "$max" ]]; then
        log "ERROR" "$name must be a positive integer between $min and $max."
        exit 1
    fi
}

# Parse command line options
read_options() {
    while getopts "hvVqc:o:dng" opt; do
        case $opt in
        h) show_help ;;
        v)
            echo -e "${CyanBold}gen_font_data.sh${ResetColor} version ${SCRIPT_VERSION}"
            exit 0
            ;;
        V) verbose=true ;;
        n) no_color=true ;;
        g) no_glyphs=true ;;
        q) quiet=true ;; # exec &>/dev/null ;;
        c) columns="$OPTARG" ;;
        o) output_dir="$OPTARG" ;;
        d) debug=true
            ;; # Enable debug output
        \?)
            log "ERROR" "Invalid option: -$OPTARG"
            exit 1
            ;;
        esac
    done
    return 0
}

# Validate the given command line options
verify_options() {

    # Check if columns is a positive integer
    validate_numeric_range "$columns" 10 100 "-c, columns"

    # Check that output directory exists
    if [[ ! -d "$output_dir" ]]; then
        log "ERROR" "Output directory \"$output_dir\" does not exist."
        exit 1
    fi
    return 0
}

# ============================================================
# Font Processing Functions  
# ============================================================

# Escape a string for use in sed search/replace (escape / and &)
sed_escape() {
    printf '%s' "$1" | sed -e 's/[\/&]/\\&/g'
}

# Wrapper around xxd to dump a file into a C-header file and convert the C-declaration
# with the default variable name to valid Rust syntax.
make_rust_hex_dump() {
    local input_file="$1"
    local output_file="$2"
    local var_name="$3"

    # The original variable name created by xxd is the full path with special chars
    # replaced by underscores.
    local original_var_name
    original_var_name=$(echo "${input_file}" | tr '.\-/' '_')

    local esc_orig
    esc_orig=$(sed_escape "$original_var_name")
    # Note: With set -o pipefail + errexit the pipeline failure will abort the script.

    xxd -i -c "$columns" "$input_file" | \
    sed -e "s/unsigned char/pub static/g" \
        -e "s/\[\]/: \&[u8]/g" \
        -e "s/unsigned int/pub static/g" \
        -e "s/{/\&[/g" \
        -e "s/}/]/g" \
        -e "s/_len/_LEN : usize/g" \
        -e "s/${esc_orig}/${var_name}/g" | \
    awk '/pub static/{print "// DO NOT EDIT! Created automatically by gen_font_data.sh\n#[allow(dead_code)]"}1' >"$output_file"
   
}

# Dump a hex representation of a file and convert it to a Rust static array.
make_rust_hex_dump_with_check() {
    local input_file=$1
    local output_file=$2
    local var_name=$3

     if [[ -z "$input_file" || -z "$output_file" || -z "$var_name" ]]; then
        log "ERROR" "make_rust_hex_dump: All parameters are required"
        return 1
    fi

    # Check if the input file exists
    if [[ ! -f "$input_file" ]]; then
        log "ERROR" "Input file \"$input_file\" does not exist."
        exit 1
    fi
    log "INFO" "Converting \"$input_file\" to \"$output_file\"..."
    make_rust_hex_dump "$input_file" "$output_file" "$var_name"

    log "INFO" "Font Conversion complete. \"$output_file\" created."
}

create_rust_variable_name() {
    local font_file="$1"
    local base_name
    base_name=$(basename "$font_file" .ttf)
    echo "${base_name//-/_}" | tr '[:lower:]' '[:upper:]'
}

setup_rust_font_file_and_variable_names() {
    # Create the Rust static variable name for the Rust font data file from the ttf font file name
    # Example: DejaVuSans.ttf -> DEJAVUSANS
    local -i i
    for i in "${!DEJAVU_TTF_FONTS[@]}"; do
        RUST_VARIABLES[i]=$(create_rust_variable_name "${DEJAVU_TTF_FONTS[$i]}")
    done

    # Create the Rust file names from the TTF font file names
    # Example: DejaVuSans.ttf -> font_dejavusans.rs
    for i in "${!DEJAVU_TTF_FONTS[@]}"; do
        name_without_underscore=${RUST_VARIABLES[$i]//_/}
        RUST_FONT_FILES[i]=$(echo "font_${name_without_underscore}.rs" | tr '[:upper:]' '[:lower:]')
    done
}

# If font files are not available locally then download them from GitHub
download_and_install_fonts() {
    local font_dir="$1"
    local -a ttf_font_files=("${@:2}") # Get all arguments after the first one

    # log "NOTICE" "No installed fonts found. Will download DejaVu fonts to \"$font_dir\"."
    mkdir -p "$font_dir" || { log "ERROR" "Failed to create font directory \"$font_dir\"."; exit 1; }

    temp_dir=$(mktemp -d) || { log "ERROR" "Failed to create temporary directory."; exit 1; }
    tmp_archive="${temp_dir}/dejavu-fonts-${DEJAVU_VERSION}.tar.bz2"

    # Download the font package from the DejaVu Fonts repository
    curl -f -S -s -L -o "$tmp_archive" "${DEJAVU_URL}" || { log "ERROR" "Failed to download DejaVu Fonts package from ${DEJAVU_URL}."; exit 1; }
    log "INFO" "DejaVu fonts package downloaded successfully."

    # Safety check: ensure archive contains no absolute paths or '..' components
    if tar -tjf "$tmp_archive" | grep -E '(^/|(^|/)\.\./)' >/dev/null; then
        log "ERROR" "Archive contains unsafe paths (absolute paths or .. entries); aborting."
        exit 1
    fi

    tar -xjf "$tmp_archive" -C "${temp_dir}" || { log "ERROR" "Failed to extract DejaVu Fonts package."; exit 1; }
    log "INFO" "DejaVu fonts package unpacked successfully."

    for ttf_file in "${ttf_font_files[@]}"; do
        if [[ ! -f "${temp_dir}/dejavu-fonts-ttf-${DEJAVU_VERSION}/ttf/${ttf_file}" ]]; then
            log "ERROR" "Required font file \"$ttf_file\" not found in the extracted package."
            exit 1
        fi
        cp -- "${temp_dir}/dejavu-fonts-ttf-${DEJAVU_VERSION}/ttf/${ttf_file}" "${font_dir}/${ttf_file}" || {
            log "ERROR" "Failed to copy font file \"$ttf_file\" to \"$font_dir\"."
            exit 1
        }
        log "INFO" "Font file \"$ttf_file\" copied to \"$font_dir\"."
    done

    # Clean up the temporary files
    rm -rf -- "${temp_dir}" || { log "ERROR" "Failed to remove temporary directory \"$temp_dir\"."; exit 1; }
    log "INFO" "Temporary files cleaned up."

    log "SUCCESS" "Font files \"${ttf_font_files[*]}\" installed in \"$font_dir\"."
}

# Install the font files if they are not already installed
install_or_skip_if_same() {
    local update=1
    for file in "${RUST_FONT_FILES[@]}"; do
        if [[ -f "${file}" && -f "${output_dir}/${file}" ]] && cmp -s -- "${file}" "${output_dir}/${file}"; then
            log "NOTICE" "New font file \"${file}\" is the same as existing \"${output_dir}/${file}\"."
        else
            mv -- "${file}" "${output_dir}/${file}" || {
                log "ERROR" "Failed to move \"${file}\" to \"${output_dir}/${file}\"."
                exit 1
            }
            log "INFO" "Moved \"${file}\" to \"${output_dir}/${file}\"."
            update=0
        fi
    done
    return $update
}

# Create the Rust static data file for each font file
create_rust_static_data_file() {
    # Create the Rust static data file for each font file
    for i in "${!DEJAVU_TTF_FONTS[@]}"; do
        make_rust_hex_dump_with_check "${font_dir}/${DEJAVU_TTF_FONTS[$i]}" "${RUST_FONT_FILES[$i]}" "${RUST_VARIABLES[$i]}"
    done
}

check_if_font_exists() {
    # Check if any of the font files are missing, then we download all of them
    log "INFO" "Checking for missing font files..."
    local need_update=false
    if [[ ! -d "${font_dir}" ]]; then
        download_and_install_fonts "${font_dir}" "${DEJAVU_TTF_FONTS[@]}"
    else
        for font in "${DEJAVU_TTF_FONTS[@]}"; do
            if [[ ! -f "${font_dir}/${font}" ]]; then
                log "NOTICE" "Font file \"${font}\" not found in \"$font_dir\". Will download fonts."
                need_update=true
                break
            fi
        done
        if [[ "${need_update}" = true ]]; then
            download_and_install_fonts "${font_dir}" "${DEJAVU_TTF_FONTS[@]}"
        fi
    fi
}   

# ============================================================
# Utility and Helper Functions
# ============================================================

# Remove temporary Rust data files created during the script execution
cleanup_temporary_files() {
    log "INFO" "Removing temporary Rust data files..."
    for file in "${RUST_FONT_FILES[@]}"; do
        if [[ -f "$file" ]]; then
            rm -- "$file" || { log "ERROR" "Failed to remove temporary file \"$file\"."; exit 1; }
            log "INFO" "Temporary file \"$file\" removed."
        fi
    done
    return 0
}

# Print information about the script's configuration
print_debug_info() {
    log "DEBUG" "Font files: \t\t\t${DEJAVU_TTF_FONTS[*]}"
    log "DEBUG" "Rust font data files: \t${RUST_FONT_FILES[*]}"
    log "DEBUG" "Rust variables: \t\t${RUST_VARIABLES[*]}"
    log "DEBUG" "Output directory: \t\t$output_dir"
    log "DEBUG" "Font directory: \t\t$font_dir"
    log "DEBUG" "Columns in hex dump: \t$columns"
}

# Print the script header information
print_script_header() {
    if [[ "${quiet}" = false ]]; then
        echo -e "${HEADER_GLYPH}gen_font_data.sh, v${SCRIPT_VERSION}${ResetColor} - Generating Rust static font data files."
    fi
}

# ============================================================
# Main Script Execution
# ============================================================

main() {
   
    # Read and verify options given to the script
    read_options "$@"

    # Possibly ignore colors and glyphs (useful in a CI/CD pipeline)
    check_no_color
    check_no_glyphs

    # Verify the options
    verify_options

     # Check for tool dependencies and exit if any are missing
    check_dependencies

    # Set up the Rust font file names and variable names from the TTF font names
    setup_rust_font_file_and_variable_names

    # Some script information
    print_script_header
    print_debug_info

    # Check if all font files exist and download if they don't
    check_if_font_exists

    # Create the Rust static variable files from the font files
    create_rust_static_data_file

    # Finally give the good news to the user
    if install_or_skip_if_same ; then
        log "SUCCESS" "Done. Rust font data files successfully updated under \"$output_dir\"."
    else
        log "SUCCESS" "Done. No updates necessary. Rust font data files are already up to date."
    fi

    # Remove the temporary Rust data files
    cleanup_temporary_files

    log "INFO" "Script completed successfully."
    exit 0
}

main "$@"
