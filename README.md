[view readme](https://github.com/nathan-websculpt/updraft-substreams/tree/main/v1)

## Getting Started
```bash
git clone git@github.com:nathan-websculpt/updraft-substreams.git <dir_name>
cd <dir_name>/v1
substreams build

substreams auth
<FOLLOW DIRECTIONS>
 . ./.substreams.env <AUTH_TOKEN_NO_QUOTES>


-- VIEW IDEA-CREATED EVENTS --

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_idea_created -s 345976339 -t +1

-- VIEW SOLUTION-CREATED EVENTS --

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_solution_created -s 346190110 -t +1

-- VIEW PROFILE-UPDATED EVENTS --

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_user_updated -s 345846756 -t +100

-- VIEW DYNAMIC IDEAS (FROM IDEA ADDRESSES [IDEA-CREATED EVENTS]) --

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_idea_events --limit-processed-blocks 600000 -s 345949351 -t +2

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_idea_events --limit-processed-blocks 600000 --start-block 345976339 --stop-block 346012422
^^^^
Idea-Created: https://arbiscan.io/tx/0x3e61bf4fb27c37332c9ed510c8256055a96a3ecb768bf33fc86e899cb113e0f6#eventlog
Idea Addr: https://arbiscan.io/address/0x8D68AF66509e4020aF3bbDF08fA70D56Dad92005
Contribution event on that Idea Addr: https://arbiscan.io/address/0x8D68AF66509e4020aF3bbDF08fA70D56Dad92005#events


-- VIEW DYNAMIC SOLUTIONS (FROM SOLUTION ADDRESSES [SOLUTION-CREATED EVENTS]) --

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_solution_events --limit-processed-blocks 800000 --start-block 346190109 --stop-block 346261880


-----------------------
-- VIEW GENERIC DATA --

substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_events -s 345473248 -t +1
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_events_calls -s 345473248 -t +1
substreams gui -e arb-one.streamingfast.io:443 substreams.yaml map_calls -s 345473248 -t +1

```

#### Notes

After adding or editing protos

add file to substreams.yaml under: `protobuf > files`, then run `substreams protogen` (to gen the .rs files), and then run `substreams build`

To avoid buf rate limits: `buf registry login` .... make an account at [buf build](https://buf.build/)
