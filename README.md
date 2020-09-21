# Intro

This repo is summary of my efforts to find out how to use [diesel.rs](https://diesel.rs) with sqlite backend.

You can find following query types:

* embedding migration in code
* insert statement
* insert or update statement
* select last inserted row using `_ROWID_` alias
* `group by` with `sum` query. `group by` is not fully implemented by [diesel.rs] so there is a trick to it

feel free to submit issues and PR to improve it.

Thanks