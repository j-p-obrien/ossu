// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// Put your code here.

// Note that R0*R1 = R0*(R1 - 1) + R0
// We compute sum = sum + R0; R1 - 1 recursively until R1 = 0
// Then we loop forever

// R2 holds our output. Set equal to 0 at initialization
@R2
M = 0

// Get number of iterations (R1) and put it in variable i
@R1
D = M
@i
M = D

// If i != 0 then continue to the loop, otherwise we end.
(END)
@END
D ; JEQ

// Set D = R0
@R0
D = M

// Compute R2 = R2 + D, i.e. R2 = R2 + R0
@R2
M = M + D

// Decrement i by 1, and additionally store in D
@i
DM = M - 1

// Unconditional jump to END, which will check to see if we continue the loop
@END
0; JMP