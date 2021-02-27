### pre 2/26
Doing this in rust has some tricky spots. For example, this guide has you modifying variables that get passed into methods in a loop. Rust will complain about this, so I had to return the hitrecord (seen from this GH repo: ) instead of returning true and effectively having an "out" variable like in C#.

### 2/26 Antialiasing
