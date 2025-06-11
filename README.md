[view readme](https://github.com/nathan-websculpt/updraft-substreams/tree/main/v1)

## Getting Started
```bash
git clone git@github.com:nathan-websculpt/updraft-substreams.git <dir_name>
cd <dir_name>/v1
substreams build

substreams auth
<FOLLOW DIRECTIONS>
 . ./.substreams.env <AUTH_TOKEN_NO_QUOTES>
substreams gui -e mainnet.eth.streamingfast.io:443 substreams.yaml map_events -s 345473248 -t +1
substreams gui -e mainnet.eth.streamingfast.io:443 substreams.yaml map_events_calls -s 345473248 -t +1


```
