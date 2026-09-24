# Panthor integration

Upstream Linux Panthor requests `mali_csffw.bin` for Gen10 Mali GPUs. StableKite `main` deliberately removes only the arch10.8 proprietary image and carries the source reconstruction next to its former location.

Current goal: converge the Rust reconstruction toward a buildable firmware image with the memory layout and entry semantics expected by Mali-G610/Panthor on RK3588. Until that build boundary is completed and hardware-tested, the source tree should be treated as a reconstruction project, not as a production replacement binary.

Other Mali CSF firmware variants (`arch10.10`, `arch10.12`, `arch11.8`, `arch12.8`, `arch13.8`) remain exactly as supplied by upstream.
