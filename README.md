# VM-Hours Allocation Service

## Overview

This service implements a VM capacity auction system where:
- Users submit **bids** (username, volume, price) for VM hours through `/buy` endpoints.
- Providers add **supply** through `/sell` endpoints.
- The system automatically **allocates** supply to the highest-priced bids (FIFO within price levels).
- Leftover supply persists and auto-matches future bids.
- It is possible to query the allocation for a specific user through the `/allocation`
endpoint, providing the username as a query parameter.

## Quick Start

### Prerequisites
- Rust 1.78+

### Build and Run

```bash
# Build the project
cargo build

# Run the server (listens on 0.0.0.0:8080)
cargo run

# Run tests
cargo test

# Run specific test suites
cargo test --lib                    # Unit tests
cargo test --test property_test     # Property tests
cargo test --test concurrency_test  # Concurrency tests
```

## API Endpoints

### POST `/buy`
Submit a bid for VM capacity.

**Request:**
```json
{
  "username": "user1",
  "volume": 100,
  "price": 5
}
```

- **Response:** `200 OK`

**Behavior:**
- If leftover supply exists, immediately allocates what's available.
- Remaining volume is queued as a bid.
- Empty username returns `400 Bad Request`.
- Zero volume is accepted (no-op).

### POST `/sell`
Add VM capacity supply to the system.

**Request:**
```json
{
  "volume": 100,
}
```

- **Response:** `200 OK`

**Behavior:**
- Allocates to outstanding bids by price (highest first).
- Within same price level, fills bids in FIFO order.
- Leftover supply is stored for future bids.


### GET `/allocation?username=u1`
Query total allocated VM-hours for a user.

- **Response:** `200 OK with plain text integer body (e.g., "150")`

**Error responses:**
- `400 Bad Request` - Missing username parameter
- `404 Not Found` - Username not found


## Example Usage

```bash
# Start server
cargo run

# In another terminal:

# User1 bids 100 hours at price 3
curl -s -X POST localhost:8080/buy \
  -H 'Content-Type: application/json' \
  -d '{"username":"u1","volume":100,"price":3}'

# User2 bids 150 hours at price 2
curl -s -X POST localhost:8080/buy \
  -H 'Content-Type: application/json' \
  -d '{"username":"u2","volume":150,"price":2}'

# User3 bids 50 hours at price 4 (highest)
curl -s -X POST localhost:8080/buy \
  -H 'Content-Type: application/json' \
  -d '{"username":"u3","volume":50,"price":4}'

# Provider sells 250 hours
curl -s -X POST localhost:8080/sell \
  -H 'Content-Type: application/json' \
  -d '{"volume":250}'

# Check allocations
curl -s 'localhost:8080/allocation?username=u1'  # Returns: 100
curl -s 'localhost:8080/allocation?username=u2'  # Returns: 100 (50 still open)
curl -s 'localhost:8080/allocation?username=u3'  # Returns: 50
```

## Architecture

### Core Components
