#!/bin/bash
set -e

# ============================================================================
# ANSI Colors
# ============================================================================
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

# ============================================================================
# Defaults
# ============================================================================
MODE="debug"
LOG_LEVEL="debug"
CARGO_FLAGS=""
PROJECT=""
BIN_NAME=""

# ============================================================================
# Functions
# ============================================================================
print_banner() {
    echo -e "${CYAN}"
    echo "  ╦╔═╔═╗╔╗╔"
    echo "  ╠╩╗║ ║║║║"
    echo "  ╩ ╩╚═╝╝╚╝"
    echo -e "${NC}"
}

print_help() {
    print_banner
    echo -e "${BOLD}USAGE${NC}"
    echo -e "  ./kon.sh ${DIM}[OPTIONS]${NC} ${GREEN}<demo>${NC}"
    echo -e "  ./kon.sh ${DIM}[OPTIONS]${NC} ${GREEN}<category>/<demo>${NC}"
    echo ""
    echo -e "${BOLD}OPTIONS${NC}"
    echo -e "  ${YELLOW}-r, --release${NC}    Build and run in release mode"
    echo -e "  ${YELLOW}-t, --trace${NC}      Set log level to 'trace'"
    echo -e "  ${YELLOW}-q, --quiet${NC}      Set log level to 'error'"
    echo -e "  ${YELLOW}-l, --list${NC}       List all available demos"
    echo -e "  ${YELLOW}-h, --help${NC}       Show this help message"
    echo ""
    echo -e "${BOLD}EXAMPLES${NC}"
    echo -e "  ${DIM}# Run app_demo${NC}"
    echo -e "  ./kon.sh app_demo"
    echo ""
    echo -e "  ${DIM}# Run specific ecs demo${NC}"
    echo -e "  ./kon.sh ecs_demo/query_demo"
    echo ""
    echo -e "  ${DIM}# Run in release mode${NC}"
    echo -e "  ./kon.sh -r ecs_demo/tag_demo"
    echo ""
}

list_demos() {
    print_banner
    echo -e "${BOLD}AVAILABLE DEMOS${NC}"
    echo ""

    for dir in demos/*/; do
        category=$(basename "$dir")

        # Check for Cargo.toml to see if it's a simple demo or category
        if [ -f "$dir/Cargo.toml" ]; then
            # Check for [[bin]] entries
            bins=$(grep -A1 "^\[\[bin\]\]" "$dir/Cargo.toml" 2>/dev/null | grep "name" | sed 's/.*= *"\(.*\)"/\1/' || true)

            if [ -n "$bins" ]; then
                echo -e "  ${MAGENTA}$category/${NC}"
                for bin in $bins; do
                    echo -e "    ${GREEN}•${NC} $category/$bin"
                done
            else
                echo -e "  ${GREEN}•${NC} $category"
            fi
        fi
    done
    echo ""
}

error() {
    echo -e "${RED}${BOLD}Error:${NC} $1" >&2
}

warn() {
    echo -e "${YELLOW}${BOLD}Warning:${NC} $1"
}

# ============================================================================
# Parse Arguments
# ============================================================================
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -r|--release) MODE="release"; CARGO_FLAGS="--release" ;;
        -t|--trace)   LOG_LEVEL="trace" ;;
        -q|--quiet)   LOG_LEVEL="error" ;;
        -l|--list)    list_demos; exit 0 ;;
        -h|--help)    print_help; list_demos; exit 0 ;;
        -*)           error "Unknown option '$1'"; echo ""; print_help; exit 1 ;;
        *)
            if [ -z "$PROJECT" ]; then
                PROJECT="$1"
            else
                error "Multiple targets provided."
                exit 1
            fi
            ;;
    esac
    shift
done

# ============================================================================
# Validate
# ============================================================================
if [ -z "$PROJECT" ]; then
    print_help
    list_demos
    exit 1
fi

# Parse category/demo format
if [[ "$PROJECT" == *"/"* ]]; then
    CATEGORY=$(echo "$PROJECT" | cut -d'/' -f1)
    BIN_NAME=$(echo "$PROJECT" | cut -d'/' -f2)

    if [ ! -d "demos/$CATEGORY" ]; then
        error "Category '$CATEGORY' not found."
        echo ""
        list_demos
        exit 1
    fi
else
    CATEGORY="$PROJECT"
    BIN_NAME=""
fi

# Check if demo exists
if [ ! -d "demos/$CATEGORY" ]; then
    error "Demo '$CATEGORY' not found."
    echo ""
    list_demos
    exit 1
fi

# ============================================================================
# Run
# ============================================================================
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}  Demo:${NC}  $PROJECT"
echo -e "${BOLD}  Mode:${NC}  $MODE"
echo -e "${BOLD}  Log:${NC}   $LOG_LEVEL"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

export RUST_BACKTRACE=1
export RUST_LOG=$LOG_LEVEL

if [ -n "$BIN_NAME" ]; then
    cargo run -p "$CATEGORY" --bin "$BIN_NAME" $CARGO_FLAGS
else
    cargo run -p "$CATEGORY" $CARGO_FLAGS
fi
