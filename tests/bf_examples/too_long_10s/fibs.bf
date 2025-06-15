++++++++++             Set cell[0] = 10 (number of Fibonacci numbers to print)
>+                     Set cell[1] = 1 (F(1))
>+                     Set cell[2] = 1 (F(2))
>>+                    Set cell[4] = 1 (print flag)

<<<[                   Loop while cell[0] > 0
    >>.                Output current Fibonacci number (cell[2])
    <.                 Output previous Fibonacci number (cell[1])
    
    <[->+>+<<]         Copy cell[1] (F(n−2)) to cell[3] and cell[4]
    >>[<<+>>-]         Add F(n−2) (in cell[4]) to F(n−1) (cell[2]) to get F(n)
    <<[-]              Clear temp cell[3]
    <[-]               Clear old F(n−2)
    <[-]               Clear loop control
    
    <                  Move to loop counter
    -                  Decrement counter
]
