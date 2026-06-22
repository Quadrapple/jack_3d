### 3d demo for the Hack computer

A demo showcasing edge-based 3d rendering with backface culling and a visibility system with rooms and portals.

As a uni project it had to be written in the original Jack language, so it uses a lot of C preprocessor macros
to avoid function calls. It also avoids using array and directly calls Memory.peek and poke.

All calculations are done with fixed-point arithmetic 
and the code abuses the fact that the divide/multiply calls in the VM emulator are to a builtin java function and not simulated on the actual Hack hardware
as it lacks even bitshifts. Without it clipping and culling would probably be intractable.

Best run here https://funkschy.github.io/nand-to-browser/
