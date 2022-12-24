```
-(x + y) + z * (-x * -y) + y
Simplify parentheses and negative signs:
  Remove unneeded parentheses:
    -(x + y) + z * (-x * -y) + y
                   ^^^^^^^^^
    -(x + y) + z * -x * -y + y

  Distribute negative signs into parentheses:
    -(x + y) + z * -x * -y + y
    ^^^^^^^^
    -x - y + z * -x * -y + y

  Cancel negative signs multiplied by each other:
    -x - y + z * -x * -y + y
                 ^^   ^^
    -x - y + z * x * y + y

Cancel terms that are both added and subtracted:

  Cancel -y and +y:
    -x - y + z * x * y + y
         ^               ^
    -x + z * x * y
```

```
(((x * y))) + (x * y)
Simplify parentheses and negative signs:
  Remove unneeded parentheses:
    (((x * y))) + (x * y)
      ^^^^^^^
    ((x * y)) + (x * y)
     ^^^^^^^
    (x * y) + (x * y)
    ^^^^^^^
    x * y + (x * y)
            ^^^^^^^
    x + y + x * y
```
