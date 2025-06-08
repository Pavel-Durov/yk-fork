#!/bin/bash

# run each test in this directory as:
# time ~/temp/bf.bench $filename, if the test takes more than 3 seconds then halt

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if bf.bench exists
BF_BENCH="$HOME/temp/bf.bench"
if [ ! -f "$BF_BENCH" ]; then
    echo -e "${RED}Error: bf.bench not found at $BF_BENCH${NC}"
    echo "Please compile bf.bench and place it at $HOME/temp/bf.bench"
    exit 1
fi

# Make sure bf.bench is executable
chmod +x "$BF_BENCH"

echo "Running Brainf*ck benchmarks with 20-second timeout..."
echo "=================================================="

# Initialize counters
total_tests=0
passed_tests=0
failed_tests=0
timeout_tests=0

# Loop through all .bf files in current directory
for bf_file in /home/pd/yk-fork/tests/bf_examples/*.bf; do
    # Check if file exists (in case no .bf files found)
    if [ ! -f "$bf_file" ]; then
        echo "No .bf files found in directory"
        exit 1
    fi
    
    total_tests=$((total_tests + 1))
    echo -e "\n${YELLOW}Testing: $(basename "$bf_file")${NC}"
    echo "----------------------------------------"
    
    # Run with timeout and capture timing
    start_time=$(date +%s.%N)
    
    # Use timeout to limit execution to 20 seconds
    timeout 10s time "$BF_BENCH" "$bf_file" 2>&1 1>/dev/null
    exit_code=$?
    
    # Calculate duration in milliseconds using bash arithmetic
    end_time=$(date +%s.%N)
    duration_ms=$(awk "BEGIN {printf \"%.1f\", ($end_time - $start_time) * 1000}")
    
    # Check exit status
    if [ $exit_code -eq 124 ]; then
        # Timeout occurred
        echo -e "${RED}✗ TIMEOUT: $(basename "$bf_file") (>20 seconds)${NC}"
        timeout_tests=$((timeout_tests + 1))
    elif [ $exit_code -eq 0 ]; then
        # Success
        printf "${GREEN}✓ PASSED: $(basename "$bf_file") (%.1fms)${NC}\n" "$duration_ms"
        passed_tests=$((passed_tests + 1))
    else
        # Other error
        echo -e "${RED}✗ FAILED: $(basename "$bf_file") (exit code: $exit_code)${NC}"
        failed_tests=$((failed_tests + 1))
    fi
done

# Summary
echo -e "\n=================================================="
echo "SUMMARY:"
echo "Total tests:   $total_tests"
echo -e "Passed:        ${GREEN}$passed_tests${NC}"
echo -e "Failed:        ${RED}$failed_tests${NC}"
echo -e "Timeouts:      ${YELLOW}$timeout_tests${NC}"

if [ $failed_tests -gt 0 ] || [ $timeout_tests -gt 0 ]; then
    exit 1
else
    echo -e "\n${GREEN}All tests completed successfully!${NC}"
    exit 0
fi