Keepld

Keepld is a lightweight system for organizing projects, objects, relationships and actions without turning organization itself into another project.

Keepld is being built around a simple idea:

«A project should describe what exists, what is related, and what needs to happen next.»

It is not a document editor, a traditional task manager, a file manager, or a productivity dashboard.

Keepld is intended to provide a small operational structure between a simple note and highly configurable project-management software.

---

Status

Current version: "0.1.0-dev"

Keepld is currently in early development.

Version "0.1" is focused on establishing the executable foundation of the system:

- Core domain model
- Projects
- Nodes
- Persistent storage
- CRUD operations
- Internal application architecture
- Command-line interface

The current implementation is intentionally small.

Cloud infrastructure, authentication, remote APIs, collaboration and production deployment are planned for later stages and are not requirements for the initial system.

---

Why Keepld?

Most organizational software falls somewhere between two extremes.

On one side, there are simple notes:

Buy ingredients
Call John
Finish prototype

They are fast, but relationships and execution context disappear easily.

On the other side, there are systems that require users to construct a complete information-management environment before they can actually work.

Keepld explores the space between those extremes.

The goal is not to make users manage more information.

The goal is to make the structure of a project sufficiently clear that the user does not have to continuously reconstruct it in their head.

---

The basic idea

Keepld represents a project as an operational structure rather than as a document.

A simplified project can be represented as:

Project
│
├── Object / Node
├── Object / Node
├── Object / Node
│      │
│      └── relationship
│
└── context

The important part is not the visual representation.

The important part is that the system knows that these things exist and how they relate.

This allows Keepld to eventually support different ways of interacting with the same underlying project:

CLI
 │
 ├── Terminal workflows
 │
 ├── Web interface
 │
 ├── Mobile interface
 │
 └── API

The interface is not the project.

The project exists underneath the interface.

---

Design principles

Small mechanisms

Keepld is deliberately built from small components with clear responsibilities.

The project should not depend on a large framework to define its fundamental behavior.

Each subsystem should do a small number of things and expose a clear interface to the rest of the system.

---

Low cognitive overhead

Keepld should require less structure than the structure it helps create.

Configuration should not become the primary activity.

The system should provide enough structure to make relationships and execution visible without requiring users to construct an elaborate workspace first.

---

Data before presentation

The underlying project model is independent from its presentation.

A project should be usable from the command line before a graphical interface exists.

This also makes the system easier to test and allows different interfaces to operate on the same model.

---

Persistent source of truth

The application state must have a clear and inspectable representation.

Indexes, caches and derived structures should not become the only representation of the user's project.

Persistence is treated as part of the architecture rather than as an implementation detail hidden behind the interface.

---

Explicit architecture

Keepld favors explicit data flow over framework-driven behavior.

The intended direction is:

Interface
    ↓
Core
    ↓
Domain
    ↓
Storage

The exact implementation will evolve, but responsibilities should remain separated.

---

Architecture

The current architecture is being developed in Rust.

At the "v0.1" stage, the project is organized around four major layers:

┌───────────────────────────────┐
│           INTERFACE           │
│              CLI              │
└───────────────┬───────────────┘
                │
┌───────────────▼───────────────┐
│             CORE              │
│     operations / application  │
└───────────────┬───────────────┘
                │
┌───────────────▼───────────────┐
│            DOMAIN             │
│       Project / Node / ...    │
└───────────────┬───────────────┘
                │
┌───────────────▼───────────────┐
│           STORAGE             │
│       persistent state        │
└───────────────────────────────┘

The purpose of this separation is to prevent the user interface, persistence mechanism and domain model from becoming one inseparable program.

---

Domain

The domain contains the concepts that define Keepld itself.

The first implementation centers around:

Project
Node
Profile
Relation
Context

Not all of these concepts are equally developed in "v0.1".

The initial implementation deliberately starts with the smallest useful subset.

---

Project

A "Project" represents an organized body of work.

A project is not simply a text document.

It provides a container for entities and relationships that together describe an operational situation.

The project model will evolve considerably between "v0.1" and later versions.

---

Node

"Node" is the initial generic entity used by "v0.1".

It provides a simple foundation for representing information inside a project while the more complete object model is being developed.

"Node" is intentionally temporary terminology.

Starting with "v0.2", it will be replaced by the more general concept of:

Object

This is a semantic change in the domain model, not merely a rename.

---

The Object Model

The next architectural step is to establish a more general object system.

The intended direction is:

Object
├── Project
├── Profile
├── Task
├── Model
└── Element

Objects will have properties and relationships rather than being treated merely as entries inside a project.

This allows Keepld to represent relationships such as:

Project
   │
   ├── contains → Object
   │
   ├── uses → Model
   │
   └── requires → Task

Task
   │
   ├── assigned_to → Profile
   ├── depends_on → Task
   └── belongs_to → Project

This model is scheduled to become a central part of "v0.2".

---

Tasks

Tasks are intentionally not part of the fundamental "v0.1" implementation.

They will be introduced after the Object model has been established.

The intended "v0.3" direction includes:

- Task creation
- Task state
- Ordering and priority
- Assignment
- Dependencies
- Relationships with projects
- Relationships with profiles
- Execution-oriented workflows

The objective is not to reproduce a conventional task manager.

A task should become another object in the operational structure of Keepld.

---

Models and Elements

Keepld is expected to distinguish between concrete objects and reusable structural definitions.

A simplified future model is:

Object
  │
  ├── concrete entities
  │
  └── properties / relationships


Model
  │
  └── reusable structure


Element
  │
  └── component of a structure

The exact semantics are still being established.

The project intentionally avoids freezing concepts before their relationships are understood.

---

Command Line

The CLI is the first interface because it provides a direct way to exercise the underlying system without requiring a graphical environment.

The intended interaction is simple:

keepld project create
keepld project read
keepld project update
keepld project delete

and eventually:

keepld object create
keepld object read
keepld object update
keepld object delete

keepld task create
keepld task run
keepld task update

The exact command syntax is still under development.

The CLI is not considered a disposable prototype. It is the first real client of the Keepld core.

---

Storage

Persistence is designed to remain explicit and inspectable.

The storage layer is separated from the domain so that the application does not depend directly on a particular persistence mechanism.

The current development stage uses local persistent data.

Remote storage and service infrastructure will be considered only after the local application model is sufficiently stable.

---

Technology

Current implementation:

- Rust
- Local persistent storage
- Command-line interface
- Small, explicitly separated components

Future infrastructure may include:

- HTTP/API layer
- Authentication
- Remote persistence
- AWS deployment
- Web interface
- Mobile interface

These are implementation stages, not prerequisites for the domain model.

---

Development philosophy

Keepld is being developed from the inside out.

The order is intentional:

Problem
  ↓
Domain
  ↓
Architecture
  ↓
Persistence
  ↓
Operations
  ↓
CLI
  ↓
User interface
  ↓
Network
  ↓
Production infrastructure

This makes the first versions less visually impressive, but it reduces the risk of building a large interface around an unstable model.

---

Roadmap

v0.1 — Foundation

Goal: obtain a small, executable Keepld.

- [x] Initial Rust project
- [x] Domain foundation
- [x] Project model
- [x] Initial Node model
- [x] Storage architecture
- [ ] Persistent CRUD
- [ ] CLI contract
- [ ] Integration
- [ ] Initial usable release

---

v0.2 — Object Model

Goal: establish the general object architecture.

- [ ] Replace "Node" with "Object"
- [ ] Project as an Object
- [ ] Profile as an Object
- [ ] Object properties
- [ ] Object relationships
- [ ] Model
- [ ] Element
- [ ] Refine persistence around the object model

---

v0.3 — Tasks

Goal: introduce execution.

- [ ] Task Object
- [ ] Task state
- [ ] Ordering / priority
- [ ] Assignment
- [ ] Dependencies
- [ ] Project ↔ Task relationships
- [ ] Profile ↔ Task relationships
- [ ] Basic execution workflow

---

Later

The project may eventually expand toward:

          Keepld
             │
     ┌───────┼────────┐
     │       │        │
    CLI     Web     Mobile
             │
            API
             │
       Remote services
             │
            AWS

Possible future capabilities include collaboration, synchronization, richer views, timelines, groups and distributed operation.

These are deliberately outside the initial scope.

---

Development status

Keepld is experimental software.

The architecture is being actively developed and interfaces may change between versions.

The "0.x" series should be considered a period of architectural construction rather than a stable API commitment.

---

Philosophy

Keepld is built around a simple distinction:

Information tells you what exists.

Structure tells you how it relates.

Execution tells you what happens next.

The purpose of Keepld is to connect these three without forcing the user to build a second system merely to organize the first one.

---

License

License information will be added before the first public release.
