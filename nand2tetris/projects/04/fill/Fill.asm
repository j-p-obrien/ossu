// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

// the index is on the screen for any index with:
// SCREEN <= index < SCREEN + 8192 = screen_max_index
@8192
D = A
@SCREEN
D = D + A
@screen_max_index
M = D

// Initialize loop index. Starts at SCREEN
(LOOP_INIT)
@SCREEN
D = A
@index
M = D
// The main loop that sets the screen color
(LOOP)
// If any key is pressed, change color to black
@KBD
D = M
@COLOR_SCREEN
D; JGT
// If no key pressed, put default color into D
D = 0
(RE_ENTER)

// Set @index to D
@index
A = M
M = D

// Compute next index value, save result into itself and D register.
@index
MD = M + 1
// If the next index value is still on-screen, jump to beginning of the
// loop; else, re-initialize loop and start from the beginning.
@screen_max_index
D = M - D
@LOOP
D; JGT
@LOOP_INIT
0; JMP

// Change D to hold black color
(COLOR_SCREEN)
D = -1
@RE_ENTER
0; JMP