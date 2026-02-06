# kociemba-http

---
A useful HTTP wrapper for the kociemba crate. 

# Setup
```bash
cargo install kociemba-http
```

# Usage
```bash
kociemba-http
```

## Configuration
The server can be configured using environment variables or a `.env` file:

- `SITE_ADDR`: The address and port the server will bind to. (Default: `0.0.0.0:3000`)

## API
### Solve Cube
`GET /solve/{cube}`

#### Parameters
- `cube`: A 54-character string representing the state of the cube using facelet notation.

**Facelet Notation Order:**
U1, U2, ..., U9, R1, R2, ..., R9, F1, F2, ..., F9, D1, D2, ..., D9, L1, L2, ..., L9, B1, B2, ..., B9

**Faces:**
- `U`: Up
- `R`: Right
- `F`: Front
- `D`: Down
- `L`: Left
- `B`: Back

#### Response
- **Success (200 OK):** Returns a JSON object containing the solution moves and the solve time.
  ```json
  {
    "solution": ["R", "D2", "B2", "R2", "L2", "B3", "U", "F3", "D2", "R", "B2", "R2", "F2", "B2", "R2", "D2", "B"],
    "solve_time": {
      "secs": 0,
      "nanos": 965972
    }
  }
  ```
- **Error (400 Bad Request):** Returns an error message if the cube string is invalid.
- **Error (500 Internal Server Error):** Returns an error message if the solver fails.