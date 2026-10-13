# interfaces/

READ-ONLY for sessions. Never edit, add, rename or delete anything here.

This directory is exported by the owner, on his own machine, from the contract repos
(instruction layouts, request formats, golden vectors under `interfaces/golden/`).
Only the owner changes it, and only in commits whose subject starts with `export:`.
CI rejects any other commit that touches this path.

If the code needs something that is missing here, mark it `TODO(interfaces)` and list it in
the PR; do not invent it.
