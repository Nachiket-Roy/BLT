---
type: comment
id: comment-web-1792729063-a82296c6-ee08-46b0-bbc4-aab4eac03384
author: FTSqIIVcAz6yrEUfHk8wm1rr_FLa2bBVWJKeRKZCwcQ
authorName: navodit
ts: 1792729063
sig: jFCkK6qnmlzRIauh9U-gFEXVbW549qmiXhU20dkfFKzXqlicGQ6qiRlqog4gw1nW3ENPdttGEDVOtM8etUpcAQ
---

Thanks @donnieblt ! Just pushed a fix for both:

1) Removed the trailing spaces from the blank line before @media (max-width: 768px) — confirmed clean via git diff and a trailing-whitespace check locally.
2) Added aria-hidden="true" to the bars icon, and fixed the indentation of #page-title-container to match the surrounding rules.
Squashed into the single commit. Let me know if anything else needs a tweak!

Also — quick question: since this PR is approved and just needs a merge, would it be okay for me to unassign myself from issue #26 to pick up another issue in the meantime? Or should I wait until this one is actually merged?
