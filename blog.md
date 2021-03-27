### pre 2/26
Doing this in rust has some tricky spots. For example, this guide has you modifying variables that get passed into methods in a loop. Rust will complain about this, so I had to return the hitrecord (seen from this GH repo: ) instead of returning true and effectively having an "out" variable like in C#.

### 2/26 Antialiasing
https://rust-by-example-ext.com/rand.html#:~:text=Random%20numbers%20A%20lot%20of%20applications%20require%20random,with%20a%20different%20performance%20and%20security%20trade%20off.

### 3/4 Diffuse Material
Crazy that diffuse lighting is calculated "randomly," and recursively. The program stack overflowed around scanline 176. The firs this method worked, it literally took about 10 minutes to render a single image.

It came out very dark, the solution here was to sqrt the color that resulted from all the samples. Apparently in order to "gamma correct" something, you need to transform each value before storing it. Transform it by raising it to the power of 1/gamma. So sqrt worked fine enough.

Reducing the sample count from 100 to 50 and the scanline count from 50 to 25 reduced render time in half (as expected). Came out VERY grainy.

Something is off. Nevermind it's good but it took forever!


### 3/26
Have a clean commit history
Label commits by chapter (consider release labels)
Save examples or final renders as you go