#!/bin/bash

# Markenz Diagram Generation Script
# Version: 1.0
# Author: Architecture Team
# Description: Automated diagram generation from source files

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE_DIR="$SCRIPT_DIR/../source"
RENDERED_DIR="$SCRIPT_DIR/../rendered"
LOG_FILE="$SCRIPT_DIR/generation.log"

# Create directories
mkdir -p "$RENDERED_DIR/png"
mkdir -p "$RENDERED_DIR/svg"
mkdir -p "$RENDERED_DIR/pdf"

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Check if required tools are installed
check_dependencies() {
    log "Checking dependencies..."
    
    if ! command -v mmdc &> /dev/null; then
        log "ERROR: Mermaid CLI (mmdc) not found. Install with: npm install -g @mermaid-js/mermaid-cli"
        exit 1
    fi
    
    if ! command -v plantuml &> /dev/null; then
        log "WARNING: PlantUML not found. PlantUML diagrams will be skipped."
    fi
    
    if ! command -v dot &> /dev/null; then
        log "WARNING: Graphviz not found. DOT diagrams will be skipped."
    fi
    
    log "Dependencies check completed."
}

# Generate Mermaid diagrams
generate_mermaid() {
    local file="$1"
    local basename=$(basename "$file" .mmd)
    local category=$(dirname "$file" | sed 's|.*/||')
    
    log "Generating Mermaid diagram: $category/$basename"
    
    # Generate PNG
    if mmdc -i "$file" -o "$RENDERED_DIR/png/${basename}.png" -t neutral -b transparent -w 1200 -H 800 2>> "$LOG_FILE"; then
        log "✓ Generated PNG: ${basename}.png"
    else
        log "✗ Failed to generate PNG: ${basename}.png"
        return 1
    fi
    
    # Generate SVG
    if mmdc -i "$file" -o "$RENDERED_DIR/svg/${basename}.svg" -t neutral -b transparent 2>> "$LOG_FILE"; then
        log "✓ Generated SVG: ${basename}.svg"
    else
        log "✗ Failed to generate SVG: ${basename}.svg"
        return 1
    fi
    
    return 0
}

# Generate PlantUML diagrams
generate_plantuml() {
    local file="$1"
    local basename=$(basename "$file" .puml)
    local category=$(dirname "$file" | sed 's|.*/||')
    
    log "Generating PlantUML diagram: $category/$basename"
    
    if command -v plantuml &> /dev/null; then
        # Generate PNG
        if plantuml -tpng -o "$RENDERED_DIR/png/" "$file" 2>> "$LOG_FILE"; then
            log "✓ Generated PNG: ${basename}.png"
        else
            log "✗ Failed to generate PNG: ${basename}.png"
            return 1
        fi
        
        # Generate SVG
        if plantuml -tsvg -o "$RENDERED_DIR/svg/" "$file" 2>> "$LOG_FILE"; then
            log "✓ Generated SVG: ${basename}.svg"
        else
            log "✗ Failed to generate SVG: ${basename}.svg"
            return 1
        fi
    else
        log "WARNING: Skipping PlantUML diagram $file (PlantUML not installed)"
        return 0
    fi
    
    return 0
}

# Generate Graphviz DOT diagrams
generate_dot() {
    local file="$1"
    local basename=$(basename "$file" .dot)
    local category=$(dirname "$file" | sed 's|.*/||')
    
    log "Generating Graphviz diagram: $category/$basename"
    
    if command -v dot &> /dev/null; then
        # Generate PNG
        if dot -Tpng -o "$RENDERED_DIR/png/${basename}.png" "$file" 2>> "$LOG_FILE"; then
            log "✓ Generated PNG: ${basename}.png"
        else
            log "✗ Failed to generate PNG: ${basename}.png"
            return 1
        fi
        
        # Generate SVG
        if dot -Tsvg -o "$RENDERED_DIR/svg/${basename}.svg" "$file" 2>> "$LOG_FILE"; then
            log "✓ Generated SVG: ${basename}.svg"
        else
            log "✗ Failed to generate SVG: ${basename}.svg"
            return 1
        fi
    else
        log "WARNING: Skipping Graphviz diagram $file (Graphviz not installed)"
        return 0
    fi
    
    return 0
}

# Generate diagram index
generate_index() {
    log "Generating diagram index..."
    
    local index_file="$RENDERED_DIR/index.html"
    
    cat > "$index_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markenz Diagrams</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .diagram-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 20px; }
        .diagram-card { border: 1px solid #ddd; border-radius: 8px; padding: 20px; }
        .diagram-card h3 { margin-top: 0; color: #333; }
        .diagram-card img { max-width: 100%; height: auto; }
        .diagram-meta { font-size: 0.9em; color: #666; margin-top: 10px; }
    </style>
</head>
<body>
    <h1>Markenz System Diagrams</h1>
    <p>Generated on $(date)</p>
    
    <div class="diagram-grid">
EOF

    # Add diagrams to index
    find "$SOURCE_DIR" -name "*.mmd" -o -name "*.puml" -o -name "*.dot" | while read file; do
        local basename=$(basename "$file" | sed 's/\.[^.]*$//')
        local category=$(dirname "$file" | sed 's|.*/||')
        local svg_file="$RENDERED_DIR/svg/${basename}.svg"
        
        if [ -f "$svg_file" ]; then
            cat >> "$index_file" << EOF
        <div class="diagram-card">
            <h3>${category}/${basename}</h3>
            <img src="svg/${basename}.svg" alt="${basename}">
            <div class="diagram-meta">
                Category: ${category}<br>
                Source: <a href="../source/${category}/${basename}.$(basename "$file" | sed 's/.*\.//')">${basename}.$(basename "$file" | sed 's/.*\.//')</a>
            </div>
        </div>
EOF
        fi
    done
    
    cat >> "$index_file" << EOF
    </div>
</body>
</html>
EOF
    
    log "✓ Generated index: $index_file"
}

# Main generation function
generate_all() {
    log "Starting diagram generation..."
    
    local total=0
    local success=0
    local failed=0
    
    # Process all source files
    find "$SOURCE_DIR" -name "*.mmd" -o -name "*.puml" -o -name "*.dot" | while read file; do
        total=$((total + 1))
        
        case "$file" in
            *.mmd)
                if generate_mermaid "$file"; then
                    success=$((success + 1))
                else
                    failed=$((failed + 1))
                fi
                ;;
            *.puml)
                if generate_plantuml "$file"; then
                    success=$((success + 1))
                else
                    failed=$((failed + 1))
                fi
                ;;
            *.dot)
                if generate_dot "$file"; then
                    success=$((success + 1))
                else
                    failed=$((failed + 1))
                fi
                ;;
        esac
    done
    
    # Generate index
    generate_index
    
    log "Generation completed. Total: $total, Success: $success, Failed: $failed"
}

# Generate specific category
generate_category() {
    local category="$1"
    log "Generating diagrams for category: $category"
    
    find "$SOURCE_DIR/$category" -name "*.mmd" -o -name "*.puml" -o -name "*.dot" | while read file; do
        case "$file" in
            *.mmd) generate_mermaid "$file" ;;
            *.puml) generate_plantuml "$file" ;;
            *.dot) generate_dot "$file" ;;
        esac
    done
}

# Generate specific file
generate_file() {
    local file="$1"
    log "Generating diagram: $file"
    
    if [ ! -f "$file" ]; then
        log "ERROR: File not found: $file"
        exit 1
    fi
    
    case "$file" in
        *.mmd) generate_mermaid "$file" ;;
        *.puml) generate_plantuml "$file" ;;
        *.dot) generate_dot "$file" ;;
        *)
            log "ERROR: Unsupported file format: $file"
            exit 1
            ;;
    esac
}

# Show usage
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --all              Generate all diagrams"
    echo "  --category CAT     Generate diagrams for specific category"
    echo "  --file FILE        Generate specific diagram file"
    echo "  --index-only       Generate only the index file"
    echo "  --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --all"
    echo "  $0 --category architecture"
    echo "  $0 --file source/architecture/system_overview_v1.mmd"
}

# Main script logic
main() {
    log "=== Diagram Generation Started ==="
    
    check_dependencies
    
    case "${1:-}" in
        --all)
            generate_all
            ;;
        --category)
            if [ -z "${2:-}" ]; then
                echo "ERROR: Category name required"
                show_usage
                exit 1
            fi
            generate_category "$2"
            ;;
        --file)
            if [ -z "${2:-}" ]; then
                echo "ERROR: File path required"
                show_usage
                exit 1
            fi
            generate_file "$2"
            ;;
        --index-only)
            generate_index
            ;;
        --help)
            show_usage
            ;;
        *)
            echo "ERROR: Invalid option"
            show_usage
            exit 1
            ;;
    esac
    
    log "=== Diagram Generation Completed ==="
}

# Run main function
main "$@"
