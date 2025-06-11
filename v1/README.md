# v1 Substreams modules

This package was initialized via `substreams init`, using the `evm-events-calls` template.

## Usage

```bash
substreams build
substreams auth
substreams gui       			  # Get streaming!
```

Optionally, you can publish your Substreams to the [Substreams Registry](https://substreams.dev).

```bash
substreams registry login         # Login to substreams.dev
substreams registry publish       # Publish your Substreams to substreams.dev
```

## Modules

All of these modules produce data filtered by these contracts:
- _updraft_factory_ at **0x08e87242f23904e22da12a392b2facbb56c2959a**
### `map_events_calls`

This module gets you events _and_ calls


### `map_events`

This module gets you only events that matched.



### `map_calls`

This module gets you only calls that matched.


