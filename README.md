# Rust server for smooth email communication

This is a multicrate repository, inspired by clean architecture , which consists of 4 parts:

1. System : all system code that has nothing to do with business logic itself
2. Domain : representation of the application main building blocks (entities, repositories, various dto)
3. Business : so called "usecase". In this folder entities from domain are used (services, actions)
4. Http : connecting business actions with http endpoints (routing, handlers, middleware)  

sea orm and axum web framework were used to build this project.

Description :

Managers use various gmail accounts to communicate with clients. Swithcing between them takes a lot of effort, so this app was created to help them handle emails easier.
Intergration with gmail api made it possibe to create a custom email client, that makes email communication experience seem like a chat.
Now our managers could just open a list of clients, select one they need to contact and write him a message, while required gmail account would be selected under the hood 
