---
type: review
id: review-web-1792782456-e01cd4b3-9967-4f35-a08f-21a6c1e8db88
author: gxltjPfmdiXKA78VbPYy3nwR8CSvKWou6mFsIjm4CWs
authorName: s3dfx-cyber
ts: 1792782456
state: changes_requested
sig: mYiAwmbF_CwY7TnOIcsaJxGVDJh3Wkk3x5upZmiClckjYVDqvtaBAEy_Avj_X_ER_ttzZVSeTpUwY5vLbvBwCA
---

Range normalization may change the meaning of ranges ending in 7, The code replaces 7 with 0 and then removes 7 and adds 0 This may work for 1-7 if the parser expands the range before normalization. But if parseField() rejects 7 because the declared maximum is 6, or if it normalizes range endpoints prematurely, the range may fail or produce incorrect values. The screenshot does not show the complete parseField() implementation, so this behavior cannot be confirmed. 
The new tests do not establish the full semantics of Sunday aliases

Does blt-cron explicitly support both 0 and 7 as Sunday?
Does parseField() expand ranges before the new normalization block runs?
Can the tests assert the exact matching dates for 1-7, 5-7, and */7, rather than only successful parsing or a limited match check?
