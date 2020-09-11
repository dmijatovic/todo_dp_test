# Todo tokio deadpool data api

This library is specifically developed for CRUD operations on todo_db. It uses tokio and deadpool for achieving this.
The CRUD operations are done on todo_list and todo_item tables of todo_db.

This crate is used in this bench project with various http libraries like actix-web, hyper and so on. The idea is to
test the performance of http libraries with the same CRUD operations and see if there is a difference in the performance.
