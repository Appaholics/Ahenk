# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-10-22

### Added

#### Database & Migrations
- SQLite database with automatic schema migrations
- Migration system with version tracking and history
- Support for 12 database tables (Users, Devices, TaskList, Task, Habits, HabitEntry, Blocks, TaskBlock, Pomodoros, BlockedItem, Sounds, FavoriteSound)
- Complete CRUD operations for all database entities
- Transaction support for complex operations

#### User Management
- User registration with Argon2 password hashing
- User authentication with timing-safe password comparison
- Email validation and duplicate prevention
- Device management and authorization
- Multi-device pairing workflows with challenge-response authentication

#### Task Management
- Task list creation and management
- Task CRUD operations with user ownership verification
- Task status tracking (pending, completed, deleted)
- Task assignment to time blocks
- Due date tracking and queries
- Priority and completion time tracking

#### Habit Tracking
- Habit creation with customizable frequency (daily, weekly, monthly)
- Habit completion logging
- Streak calculation algorithm
- Historical habit entry queries
- Optional notes for habit completions

#### Time Blocking & Pomodoro
- Time block scheduling with date/time support
- Pomodoro preset management
- Customizable work/break durations
- Task assignment to specific time blocks

#### Focus & Blocking
- Website and app blocking system
- Active blocklist queries by date/time
- Support for different blocking types (website, app)

#### Audio Management
- Sound library with categories
- User favorite sounds system
- Sound metadata (name, category, file path)

#### P2P Synchronization
- libp2p-based peer-to-peer networking
- CRDT-based conflict resolution
- Operation log (oplog) for change tracking
- Peer discovery and connection management
- Device authorization for sync
- Relay server support
- Bootstrap node connectivity
- Sync message encoding/decoding
- Automatic conflict resolution with vector clocks

#### Cross-Platform Support
- iOS (arm64, simulator)
- Android (arm64-v8a, armeabi-v7a, x86, x86_64)
- macOS (Apple Silicon, Intel)
- Windows (x64, ARM64)
- Linux (x86_64, ARM64)
- WatchOS (device, simulator)
- WearOS (arm64-v8a, armeabi-v7a)

#### FFI & Integration
- C-compatible FFI interface for cross-language integration
- Tauri API support (optional feature)
- Build scripts for all platforms
- Static and dynamic library outputs

#### Testing
- 59 comprehensive tests
- Unit tests for core functionality
- Integration tests for database operations
- Logic tests for business rules
- Migration tests for schema versioning
- Sync tests for P2P functionality

#### Documentation
- Comprehensive README with examples
- Database migration guide
- Cross-compilation documentation
- API documentation with rustdoc
- Implementation summaries

### Security
- Argon2 password hashing with cryptographic salts
- SQL injection prevention via parameterized queries
- Input validation on all operations
- Access control with user ownership verification
- Timing-safe password comparison
- Device authorization with challenge-response authentication

### Performance
- Binary size optimization (opt-level = "z")
- Link-time optimization (LTO)
- Symbol stripping
- Optimized panic handling for smaller binaries

[Unreleased]: https://github.com/kodfikirsanat/focussuite/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kodfikirsanat/focussuite/releases/tag/v0.1.0
