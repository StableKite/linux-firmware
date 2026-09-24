# Upstream synchronization

`upstream-main` is an exact tracking branch for `https://gitlab.com/kernel-firmware/linux-firmware.git` `main`. StableKite development happens on `main`.

On each push to StableKite `main`, GitHub Actions fetches GitLab upstream and attempts a normal Git merge into `main`. If upstream changes the deleted arch10.8 binary or another locally modified path and Git cannot merge automatically, the job exits non-zero and prints the conflicted paths. No conflict is auto-resolved.

After a successful merge, the workflow updates `upstream-main`, pushes new upstream tags, and pushes the merged `main`.
