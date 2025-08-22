# L-Systems

<img src="imgs/dragon.svg" width="300" height="300" style="background-color:white">

## Summary
A command line tool that outputs images of L-Systems that are specified in a json format.

## Format Example
```json
{
    "iters": 6,
    "alphabet": ["X", "F", "+", "-", "[", "]"],
    "axiom": ["X"],
    "rules": {
        "X": ["F", "+", "[", "[", "X", "]", "-", "X", "]", "-", "F", "[", "-", "F", "X", "]", "+", "X"],
        "F": ["F", "F"]
    },
    "interpretation": {
        "F": [{"forward": 10.0}],
        "-": [{"left": 25.0}],
        "+": [{"right": 25.0}],
        "[": [{"push": null}],
        "]": [{"pop": null}]
    }
}
```

## Build and run
Use cargo to build and run the project:

```bash
cargo build
cargo run -- l_system_json_file_path
```

The output will be in _test.svg_.