def generate_brainfuck_sum(n):    
    code = []
    code.append('>' + '+' * n)  # Initialise cell 1 with N
    code.append('[<+> -]')      # Loop: move N times from cell 1 to cell 0 (acts like sum += 1 + 2 + ... + N)

    # This does not yet compute 1 + 2 + ... + N, so we simulate it:
    # Reset cells
    code = ['[-]']         # Clear cell 0
    code.append('>[-]')    # Clear cell 1
    code.append('>[-]')    # Clear cell 2 (temporary)
    
    # Fill cell 1 with N
    code.append('>' + '+' * n)
    code.append('<')       # Move to cell 1
    code.append('[->+>+<<]')   # Copy N to cell 2 and cell 3
    code.append('>>[-<<+>>]')  # Restore cell 1 from cell 3
    code.append('<<')         # Move to cell 1
    
    # Begin loop to compute sum
    code.append('[>+>+<<-]')   # Duplicate counter to cell 2 and 3
    code.append('>>[<<+>>-]')  # Move duplicated value to cell 0 (sum)
    code.append('<[-<+>]')     # Restore counter
    code.append('<')           # End on cell 0

    return ''.join(code)

if __name__ == "__main__":
    import sys
    args = sys.argv[1:]
    if len(args) != 1:
        print("Usage: python gen.py <number>")
        sys.exit(1)
    n = int(args[0])
    bf_code = generate_brainfuck_sum(n)
    print(bf_code)
