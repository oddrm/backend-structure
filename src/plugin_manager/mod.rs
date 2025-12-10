pub mod manager;

/*
TODO implement plugin manager
the user/hooks trigger jobs
every job:
- has a set of parameters
- a plugin which executes the job
- can request/release locks on a number of entries
- lock required for modification, read but not enumerate
- has an own directory where it can store temporary files
- yields transactions which get executed atomically
- if locked file gets modified while job is running, job gets aborted
- communication over stdin/stdout
---
communication protocol:
- manager -> job initial info:
    - job parameters: Map<String, String>
    - temporary directory: PathBuf
- job -> manager:
    - everything in queries
    - request lock: Vec<EntryID>
        - blocking until lock acquired
    - release lock: Vec<EntryID>
- manager -> job:
    - suspend
    - continue
    - stop
    - (kill)
*/
