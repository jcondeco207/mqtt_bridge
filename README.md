# MQTT Bridge - **!STILL IN DEVELOPMENT!**

Bridge between Chirpstack and Apache Kafka (for masters dissertation).


![alt text](<Master Thesis.png>)

The **MQTT Bridge** is a Rust-based microservice built with **Axum** that acts as a connector between the **Chirpstack pub/sub system** and **Apache Kafka**.  

Its purpose is to **consume uplink events from devices managed in Chirpstack, transform the data, and publish it to Kafka topics**. This bridge is designed for scalability, with multiple instances running in parallel—each instance handling only a subset of devices assigned by the `topics_manager` service.

---

## Objectives

- Enable **distributed processing of uplink events** by spawning multiple microservices (bridges).
- Ensure **fine-grained assignment of devices**: each bridge only processes events from the devices assigned to it.
- Provide **basic transformations** to incoming Chirpstack payloads before pushing them into Apache Kafka.

---

## Workflow

### 1. Initialization
On startup, each MQTT Bridge instance:
- Registers itself with:
  - **ID** (provided via Docker ENV var)
  - **Host**
  - **Port**
- Notifies the `topics_manager` that it is ready.

### 2. Topic Assignment
- The `topics_manager` microservice:
  - Creates Kafka topics (one per device).
  - Assigns devices/topics to each bridge instance by writing assignments into the database.
- The MQTT Bridge queries the DB for its assigned topics.

### 3. Subscription & Processing
- The bridge subscribes to **Chirpstack MQTT uplink events** for the assigned devices.
- On receiving an uplink:
  - Extracts **DevEUI**, **deduplicationId**, and **rawData**.
  - Parses and converts metrics into **labeled fields**.
  - Publishes the transformed payload to the corresponding Kafka topic.

---

## Tech Stack

- **Language:** `Rust`
- **Frameworks:** `Axum`
