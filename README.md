# Thread Id

Ultrafast, unique, per-thread ids.

Fast as possible thread ids. This is for cases when you need an ultra-fast,
unique thread id and nothing else. Ids are not promised continuous, nor do they
prevent reuse. Throwing those two conditions out allows for extremely fast
implementations on many platforms.
