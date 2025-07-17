#!/bin/bash
# Final cleanup - remove all redundant scripts

echo "🧹 Consolidating to single blueprint.sh script..."

# Remove redundant scripts
rm -f scripts/research_workflow.sh
rm -f scripts/mcp_research.sh  
rm -f scripts/full_workflow.sh
rm -f scripts/blueprint_master.sh
rm -f scripts/blueprint_workflow.sh
rm -f scripts/quality_pipeline.sh

# Remove redundant docs
rm -f docs/WORKFLOW_BREAKDOWN.md

# Verify blueprint.sh exists and is executable
if [ ! -f scripts/blueprint.sh ]; then
    echo "❌ ERROR: scripts/blueprint.sh not found!"
    echo "Create blueprint.sh before running cleanup."
    exit 1
fi

chmod +x scripts/blueprint.sh

echo "✅ Cleanup complete! Use scripts/blueprint.sh for everything."
echo ""
echo "🚀 New simplified workflow:"
echo "  ./scripts/blueprint.sh switch           # Complete component"
echo "  ./scripts/blueprint.sh button I         # Phase I only"
echo "  ./scripts/blueprint.sh dialog research  # Research only"
echo ""
echo "💡 VS Code: Ctrl+Shift+P → 'Tasks: Run Task' → 'Generate Component'"
