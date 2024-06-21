```ascii
| ........................ process ......................... |
| .......... parse .. | ... run ... | ... stringify .........|

           +--------+                 +----------+
Input ->-  | Parser | ->-   AST   ->- | Compiler | ->- Output
           +--------+        |        +----------+
                             X
                             |
                      +--------------+
                      | Transformers |
                      +--------------+
```
