Converts forum export data from parity encoded format to serde serialized json

Usage:

Run joystream-node (Acropolis testnet):

```
joystream-node
```

Exported Forum state using the export tool:

```
git clone -b acropolis https://github.com/Joystream/joystream-api-examples.git
cd joystream-api-examples
yarn && yarn build
node lib/export_forum --encoded > forum_data_encoded.json
```

Run the transofrm tool:

```bash
git clone https://github.com/Joystream/transform-encoded-forum-export
cd transform-encoded-forum-export/
cargo build
# output file from export tool
cat forum_data_encoded.json | ./target/debug/transform-encoded-forum-export > forum_data_serialized.json
```
