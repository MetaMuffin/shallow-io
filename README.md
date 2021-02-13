# some random .io game

### Client-bound packets

- `spawn-request`
- `control`
    - `velocity`
    - `position`
    - `rotation`
    - `shooting`
    - `meta` Optional meta

### Server-bound packets

- `info` Informs the client about global parameters.
    - Insert some important parameters here...

- `player-update` Updates player controlled entity id
    - `id`
- `spawn` Spawns a new entity
    - `id`
    - `type`
    - `velocity`
    - `position`
    - `health`
    - `team`
    - `meta` Optional metadata
- `update`
    - `id`
    - `velocity`
    - `position`
    - `health`
    - `team`
    - `meta` Optional metadata
- `remove` Remove an entity
    - `id`
    - `meta` Optional metadata, for example how the entity was killed and which animation should be used.