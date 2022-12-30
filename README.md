# Auth service API

This project is api for:
- users registration and saving user data to database
- users authentication and returning auth token

In the project I use:
- actix-web framework for building APIs 
- sqlx framework for working with database
- tokio framework for writing asynchronous methods 
- tracing framework for logging
- ...

Also I testing API methods and methods for working with database. and other services in app.

For run project you need:
 - from script folder run init_db.sh (create database, run migration)
 - run 'cargo run'

You can see swagger doc. For this you need run project and go http://127.0.0.1:8000/swagger/

API methods:
- http://localhost:8000/auth_service/v1/authentication
- http://localhost:8000/auth_service/v1/registration
