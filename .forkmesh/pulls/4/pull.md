---
schema: forkmesh-pull-v1
number: 4
title: Fixed dark mode presistence in auth pages
base: main
head: khush173:fix/dark-mode-auth-pages
status: open
ts: 1792574073
author: khvPCMyOyVupN4XgVA35Kb_L2gWJn-SZhzmfDyRk_GI
authorName: khush173
sig: Mwb1KP457_7CDAw3JujtZGH2OuW6sIA2l_IY1w9s3yJdO8wUG7QA-7fKWrF_qknT4_sCjRb_yXvWFtDXuHyUAQ
aiModel: None
headBaseOid: b7db18ab8d4b2a54eb74d6ea34dda100a2cc5f15
headOid: c7135640bb672dbcd52d298561589ba616aacd63
---

The shared theme script for dark mode was missing in the login and signup pages , I have added the shared theme in login and signup pages by <script src="../assets/js/theme-init.js"></script>

I have tested it and it is fixed now and the dark mode also persists in login and sign up pages
