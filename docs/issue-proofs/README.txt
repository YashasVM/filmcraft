FilmCraft reproduction evidence for #4587, #4844, and #4891

The native screenshots and CLI results were captured on Linux from the original
commit 7215e46c8ff42d40bd4aa73f9fb688baf5524b18 and the reviewed local ten-fix
checkout. The three publication candidates are isolated onto upstream commit
c979757bb92c0b87228ba02aa59fc6dab2e2f82a. Every pre-existing file touched by
these candidates is byte-identical between those two upstream bases; see
base-compatibility.json. Fresh per-candidate test logs are under checks/.

The before/after keyframe captures follow the same two-key selection and Clear
action. State JSON independently records how many keys remain. The header pair
uses 960 x 720; the additional wide capture is 1440 x 900. The fixed code was
already present for these captures; these are actual native windows, not mockups.
Native capture used software OpenGL. The environment has no physical audio
device, so its initialization warning in a window is unrelated to the fixes.

PNG files here were recompressed losslessly. screenshot-integrity.json records
the identical decompressed pixel streams and source sizes. Original capture
files are retained in the local proof archive delivered with the initial report.
Only the software's own UI and procedural demo media are shown. Attribution
sidecars and the repository attribution index accompany the images.

The one Sol reviewer approved the combined scoped changes with score 9.3/10.
The independent PR candidates retain the reviewed production changes; the
sequence-name test file contains only its own issue's regression. The Sol ledger
records one full review round, no findings, and no usage data exposed.

Two broader proposals overlap these candidates: #3753 includes per-key Clear
commands within an easing editor; #669 combines header spacing with native font
selection. The candidate PR descriptions disclose these overlaps. Seven other
local fixes with equivalent open proposals are not being submitted separately.

No PR head is represented as having a green full workspace suite. See
ci-limitations.txt for the exact scope and remaining baseline/tooling failures.
