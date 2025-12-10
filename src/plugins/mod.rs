pub mod manager;

/*
TODO
the user/hooks trigger jobs
every job:
- has a set of parameters
- a plugin which executes the job
- can request/release locks on a number of entries
- has an own directory where it can store temporary files
- yields transactions which get executed atomically
- if locked file gets modified while job is running, job gets aborted
- communication over stdin/stdout
---
communication protocol:

*/
