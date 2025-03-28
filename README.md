# BambangShop Publisher App

Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project

In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:

1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment

1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable | type | description |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)

- [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
- **STAGE 1: Implement models and repositories**
  - [ ] Commit: `Create Subscriber model struct.`
  - [ ] Commit: `Create Notification model struct.`
  - [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
  - [ ] Commit: `Implement add function in Subscriber repository.`
  - [ ] Commit: `Implement list_all function in Subscriber repository.`
  - [ ] Commit: `Implement delete function in Subscriber repository.`
  - [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
- **STAGE 2: Implement services and controllers**
  - [ ] Commit: `Create Notification service struct skeleton.`
  - [ ] Commit: `Implement subscribe function in Notification service.`
  - [ ] Commit: `Implement subscribe function in Notification controller.`
  - [ ] Commit: `Implement unsubscribe function in Notification service.`
  - [ ] Commit: `Implement unsubscribe function in Notification controller.`
  - [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
- **STAGE 3: Implement notification mechanism**
  - [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
  - [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
  - [ ] Commit: `Implement publish function in Program service and Program controller.`
  - [ ] Commit: `Edit Product service methods to call notify after create/delete.`
  - [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections

This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. **Subscriber as Interface vs. Concrete Class:**
   In the classic Observer pattern described in Head First Design Pattern, Observer is typically defined as an interface. In the BambangShop case, we're using a concrete `Subscriber` struct instead of a trait (Rust's equivalent of an interface). This approach works well here because all subscribers have the same behavior - they receive HTTP notifications via the same mechanism. Using a concrete struct simplifies the implementation.

   However, if we needed to support multiple notification mechanisms (e.g., HTTP, WebSockets, email), then a trait would be more appropriate. A trait would allow different subscriber implementations while maintaining a common interface for the notification process.

2. **Vec vs. DashMap for Unique Identifiers:**
   Using DashMap instead of Vec is necessary in this case because we need efficient lookups, updates, and deletions based on unique identifiers (URL for Subscribers). With a Vec, operations like finding or deleting a specific subscriber would require O(n) time complexity as we'd need to iterate through the entire list. DashMap provides near-constant time O(1) access, making operations much more efficient when dealing with unique identifiers.

   Additionally, DashMap maintains the association between keys (URLs) and values (Subscriber objects), ensuring uniqueness and providing a natural way to organize subscribers by product type.

3. **DashMap vs. Singleton Pattern for Thread-Safety:**
   In our implementation, we're actually using both patterns for different purposes. The `lazy_static!` macro essentially implements the Singleton pattern by ensuring SUBSCRIBERS is initialized only once. Meanwhile, DashMap provides thread-safe concurrent access to the map structure itself.

   We still need DashMap (or an alternative like HashMap with Mutex/RwLock) because Singleton alone doesn't address thread safety of operations on the contained data structure. DashMap simplifies our code by providing thread-safety out of the box, avoiding explicit lock management that would be necessary with regular collections.

#### Reflection Publisher-2

1. **Separating Service and Repository from Model:**
   While the traditional MVC pattern combines data storage and business logic into the Model component, separating these concerns into Service and Repository layers provides several benefits aligned with modern design principles:

   - **Single Responsibility Principle**: Each component has a clearly defined responsibility - Models represent data structures, Repositories handle data access, and Services contain business logic.

   - **Separation of Concerns**: By isolating data access (Repository) from business rules (Service), we can change how data is stored without affecting business operations, and vice versa.

   - **Testability**: Smaller, focused components are easier to test in isolation. We can mock repositories when testing services without dealing with actual data storage.

   - **Maintainability**: When business logic or data access patterns need to change, we can make targeted modifications to the appropriate layer without affecting other parts of the application.

   - **Scalability**: As the application grows, this separation allows different teams to work on different layers without stepping on each other's toes.

2. **Complexity Without Separation:**
   If we merged everything back into the Model layer:

   - Each Model would become a large, monolithic class handling data structure, storage, and business logic.

   - The `Subscriber` model would need to include logic for both storing itself and sending notifications, mixing concerns.

   - The `Notification` model would become bloated with logic for creating notifications and distributing them to subscribers.

   - Cross-model interactions would create tight coupling, making changes risky and testing difficult.

   - As requirements evolve, these large models would become increasingly difficult to maintain and understand, violating the principle that "classes should be open for extension but closed for modification."

3. **Experience with Postman:**
   Postman has been invaluable for testing the BambangShop API:

   - **Request Collections**: Organizing related API calls (product creation, subscriber management) into collections makes testing workflows efficient.

   - **Environment Variables**: Setting up different environments (development, testing) with their own variables helps switch contexts quickly.

   - **Automated Testing**: Writing test scripts that run after each request allows verification of response codes, payload structure, and business rules.

   - **Documentation**: Postman can generate API documentation from collections, making it easier for team members to understand available endpoints.

   - **Request History**: The history feature helps track what has been tried before, which is useful when debugging intermittent issues.

   - **Mock Servers**: For future projects, Postman's ability to create mock endpoints would allow frontend development to proceed even when backend APIs aren't complete.

   - **Collaboration**: Sharing collections with team members ensures everyone tests against the same endpoints with consistent parameters.

   These features will be particularly helpful in our group project for maintaining API quality and consistency across the team.

#### Reflection Publisher-3

1. **Push vs. Pull Model in Observer Pattern:**
   In this BambangShop implementation, we're using the Push model variation of the Observer pattern. The publisher (NotificationService) actively pushes data to subscribers by sending HTTP POST requests with notification payloads whenever a product is created, deleted, or promoted. Subscribers don't need to request updates; they simply wait to receive notifications when events occur.

2. **Advantages and Disadvantages of Using Pull Model Instead:**
   If we used a Pull model instead:

   **Advantages:**

   - Subscribers could control when they receive updates, reducing unnecessary processing if they're busy with other tasks
   - Lower resource usage on the publisher side since it wouldn't need to maintain active connections to all subscribers
   - Subscribers could request only specific information they need at that moment
   - Better handling of slow or unreliable subscriber connections without affecting the publisher

   **Disadvantages:**

   - Increased latency between event occurrence and subscriber awareness
   - More complex client implementation requiring polling mechanisms
   - Higher overall network traffic if subscribers poll frequently
   - Subscribers might miss important updates if their polling interval is too long
   - Need for additional endpoints on the publisher side to handle polling requests

3. **Impact of Not Using Multi-threading in Notification Process:**
   If we removed multi-threading from our notification process:

   - The publisher would process notifications sequentially, blocking the main thread until all HTTP requests complete
   - Performance would degrade significantly as the number of subscribers increases
   - If any subscriber is slow to respond or times out, all subsequent notifications would be delayed
   - The entire application would become less responsive during notification broadcasts
   - The publisher might hit timeout limits when dealing with many subscribers
   - System resources would be underutilized (especially on multi-core systems)

   Multi-threading is crucial in this implementation because it allows the notification process to happen asynchronously without blocking the main application flow, making the system more responsive and scalable.
