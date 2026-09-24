# Reconstruction status

The source in `../mali-csffw/` is the Stage-17 Rust reconstruction of the Arm Mali CSF arch10.8 firmware track. It is source-only and contains no copy of the proprietary firmware image.

## Recovered model

- Cortex-M7 execution model.
- Code window classified at `0x0080_0000..0x0081_5000`.
- Initialized data source at `0x0100_0000`; runtime SRAM begins at `0x0200_0000`.
- Recovered initialized-data and BSS relocation boundaries.
- Cortex-M system-control/fault-register model and decoded CFSR/HFSR state.
- MPU region-0 programs recovered for both observed modes.
- Early reset plan and cache-control behavior.
- Scheduler tick/context-switch transition.
- Stage-17 correction: when the sampled current task equals the last task at the dispatch boundary, the reference firmware reports `Firmware hang detected at PC 0x%08x` and enters the recovered halt/fault path rather than normal scheduling.

## Confidence boundary

The crate is a behavioral/source reconstruction. It is not yet claimed to be a drop-in byte-identical `mali_csffw.bin` image. Hardware-facing semantics that are not proven from evidence remain isolated behind typed boundaries rather than guessed.
