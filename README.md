# drone-monitoring-system

A distributed surveillance platform built in **Rust** that simulates an autonomous incident prevention network using **drones, surveillance cameras, and a message broker** based on the **publish–subscribe architecture**.

This project was developed as part of the **Taller de Programación** course at **FIUBA (University of Buenos Aires)**.

The system models how a city could coordinate multiple autonomous agents to detect and respond to public incidents in real time.

## Contributors

This project was developed as a team assignment by the following students:

- **Kevin Alberto Vallejo** — 109975  
- **Mateo Ezequiel Ibañez** — 107983  
- **Gonzalo Calderon** — 107143  
- **Julian Mutchinick** — 99479  
---

<img width="510" height="622" alt="image" src="https://github.com/user-attachments/assets/42139b86-0b35-4e82-9e22-9022ff1fa655" />


# System Overview

The platform simulates a **smart city surveillance network** composed of multiple distributed components communicating through a custom **asynchronous messaging service**.

The architecture follows a **client–server model** with a **publish–subscribe communication pattern** similar to protocols like:

- MQTT
- AMQP

A central **message broker server** coordinates communication between several independent applications.

### Main components

- **Message Broker Server**
    - Handles client connections
    - Routes messages between publishers and subscribers
    - Maintains sessions and message delivery guarantees
- **Monitoring Application**
    - Displays the state of the entire system
    - Allows users to report incidents
    - Visualizes drones, cameras, and active incidents on a map
- **Drone Agent Software**
    - Represents autonomous drones
    - Receives incident notifications
    - Moves toward incidents if conditions allow
    - Reports position and status periodically
- **Camera Control System**
    - Manages surveillance cameras
    - Switches between energy-saving and alert modes
    - Can trigger incidents automatically

---

# Features

### Asynchronous Messaging System

Custom implementation of a message broker supporting:

- **Publish–Subscribe communication**
- **Client sessions**
- **Message persistence**
- **At-least-once delivery (QoS)**
- **Client reconnection with message recovery**

---

### Distributed Autonomous Agents

Drones operate independently and react to incidents based on:

- Maximum operation range
- Battery level
- Incident proximity
- Other drones already responding

---

### Smart Camera Network

Cameras dynamically adapt their behavior:

- Energy-saving mode when idle
- Alert mode when incidents occur nearby
- Ability to trigger incidents through image processing

---

### Incident Resolution Logic

An incident is resolved when:

- At least **two drones reach the incident location**
- They remain there for a configurable amount of time

---

# Architecture

The system is composed of several independent Rust applications communicating through the messaging server.

Each component runs independently and communicates using asynchronous messaging.

---

# Technologies

- **Rust (stable)**
- **TCP networking**
- **Multi-threading**
- **Channels**
- **Custom message broker**
- **Unit and integration testing**

Allowed crates used in the project:

- `rand`
- `chrono`

---

# Running the Project

## Start the Server

From the root directory:

```bash
cargo run --bin 0server <port>
```

Example:

```
cargo run--bin 0server5000
```

Before running the server, ensure that the `resources` folder inside the `jetstream` directory is empty if it exists.

## Monitoring System UI

Navigate to the monitoring application directory and run:

```
cargo run
```

---

## Drone Agent UI

From the drone application directory:

```
cargo run
```

---

## Camera Control System

From the camera system directory:

```
cargo run
```

---

## Client

To run a generic client:

```
cargo run-- <server_ip> <server_port>
```

Example:

```
cargo run--127.0.0.15000
```

---

# Testing

The project includes both **unit tests** and **integration tests**.

Mocks are used for some components (such as TCP streams and message clients) in order to isolate functionality during testing.

To run tests:

```
cargo test
```

Tests can also be run from individual application directories.
