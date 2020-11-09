# ip-in-subnet

Checking that subnet contains an IP address.

## Usage

To use `ip-in-subnet`, first add this to your `Cargo.toml`:

```toml
[dependencies]
ip-in-subnet = "0.1"
```

## Examples

### Check that subnet contains an interface.

```rust
extern crate ip_in_subnet;

use ip_in_subnet::iface_in_subnet;

let res = iface_in_subnet("192.168.182.1", "192.168.182.0/24").unwrap();
assert!(res);
```

```rust
extern crate ip_in_subnet;

use ip_in_subnet::iface_in_subnet;

let res = iface_in_subnet("192.168.183.1", "192.168.182.0/24").unwrap();
assert!(!res);
```

### Check that any subnet contains an interface.

```rust
extern crate ip_in_subnet;

use ip_in_subnet::iface_in_any_subnet;

let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
let res = iface_in_any_subnet("192.168.182.1", &subnets).unwrap();
assert!(res);
```

```rust
extern crate ip_in_subnet;

use ip_in_subnet::iface_in_any_subnet;

let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
let res = iface_in_any_subnet("192.168.183.1", &subnets).unwrap();
assert!(!res);
```

### Check that all subnets contains an interface.

```rust
extern crate ip_in_subnet;

use ip_in_subnet::iface_in_all_subnets;

let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
assert!(res);
```

```rust
extern crate ip_in_subnet;

use ip_in_subnet::iface_in_all_subnets;

let subnets = vec!["192.168.182.0/24", "192.168.182.2/32"];
let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
assert!(!res);
```

### Check that any subnet contains any interface.

```rust
extern crate ip_in_subnet;

use ip_in_subnet::any_iface_in_any_subnet;

let ifaces = vec!["192.168.182.1", "192.168.182.2"];
let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
assert!(res);
```

```rust
extern crate ip_in_subnet;

use ip_in_subnet::any_iface_in_any_subnet;

let ifaces = vec!["192.168.182.1", "192.168.182.2"];
let subnets = vec!["192.168.181.0/24", "192.168.182.3/32"];
let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
assert!(!res);
```
