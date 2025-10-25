## TODO List

### **Camera**:

- `camera`: Moving camera.

### **Control**:

- `control`: Add No-air-jump, Coyote Time and Wall Jump.

### **Collision**:

- `collision`
  - `mod`: CollisionPlugin.
  - `component`: Collision component that attach to player and platform.
  - `calculation`: Collision select and calculate overlap
- `event`: Write an event to send collision graphic information.
- `physics`
  - `collision`: Perform physical analysis on the incoming collision information.

### **Rope**

- `rope`
  - `spawn`: Spawn rope and send event to request Physics component.
  - `visualization`: Rope visualization.
- `physics`
  - `rope`: Rope physics analysis.

### **Map Loading**:

- `mapload`:
  - `mod`: MapLoadingPlugin.
  - `mapdata`: Convert the json file in assets into resources.
  - `background`: Spawn background.
  - `platform`: Send platform spawn event.
  - `coin`: Send coin spawn event.
  - `resoruce`: Save map data as resource.
- `event`: Write platform and coin spawn events.

### **Platform**:

- `platform`: Spawn platform as an entity.
  - `mod`: PlatformPlugin.
  - `spawn`: Platform spawn and send collision component attachment request and physics component attachment request.
- `physics`
  - `platform`: Add moving platform logic here.

### **Coin System**:

- `coin`: Spawn coin as an entity.
  - `mod`: CoinPlugin.
  - `spawn`: Spawn coin.
  - `detect`: Detect coin gathering inside the Coin System. Do not solve it in collision.

### **AI Bot**:

- `ai`:
  - `observer`: Detect environment as state
  - `policy`: Choose policy
  - `controller`: Get action answer from policy, and send it to control module.
- `control`:
  - `scheme`: Add a new logic to convert the action output by the AI ​​module into the ControlScheme of the control module