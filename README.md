[view readme](https://github.com/nathan-websculpt/updraft-substreams/tree/main/v1)

## Getting Started
```bash
git clone git@github.com:nathan-websculpt/updraft-substreams.git <dir_name>
cd <dir_name>/v1
substreams build

substreams auth
<FOLLOW DIRECTIONS>
 . ./.substreams.env <AUTH_TOKEN_NO_QUOTES>


substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_events -s 345473248 -t +1
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_events_calls -s 345473248 -t +1
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_calls -s 345473248 -t +1

-- VIEW IDEA-CREATED EVENTS --
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_idea_created -s 345976339 -t +1

-- VIEW SOLUTION-CREATED EVENTS --
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_solution_created -s 346190110 -t +1

-- VIEW PROFILE-UPDATED EVENTS --
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_user_updated -s 345846756 -t +100

-- VIEW DYNAMIC IDEAS (FROM IDEA ADDRESSES [IDEA-CREATED EVENTS]) --
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_idea_events --limit-processed-blocks 600000 -s 345976039 -t +1000
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_idea_events --limit-processed-blocks 600000 -s 345949351 -t +2




```

#### Notes

After adding or editing protos

add file to substreams.yaml under: `protobuf > files`, then run `substreams protogen` (to gen the .rs files), and then run `substreams build`

To avoid buf rate limits: `buf registry login` .... make an account at [buf build](https://buf.build/)
