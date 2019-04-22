# NetBricks

[NetBricks](http://netbricks.io/) is a Rust based framework for NFV development. Please refer to the
[paper](https://people.eecs.berkeley.edu/~apanda/assets/papers/osdi16.pdf) for information
about the architecture and design. Currently NetBricks requires a relatively modern Linux version.

## Dependencies

Building NetBricks requires the following dependency packages (on Debian):

```
apt-get install libcurl4-gnutls-dev, libgnutls30 libgnutls-openssl-dev,
tcpdump, libclang-dev, libpcap-dev (for dpdk), libnuma-dev (for dpdk)
```

NetBricks also supports using SCTP as a control protocol. SCTP support requires
the use of `libsctp` (this is an optional dependency) which can be installed on
Debian using:

```
apt-get install libsctp-dev
```

Comcast's team has more information about their dev environment setup below:

- Look further at the our [utils README](//github.com/williamofockham/utils/blob/master/README.md) to understand the layout of our sandbox and design of our Docker images.
- If you're building NetBricks locally, take a look at how we set out or [development VM](https://github.com/williamofockham/utils/blob/master/vm-setup.sh).
  around transparent hugepages and the loading of modules.
- Read more about how different PMDs (poll-mode drivers) require varying kernel drivers on the [DPDK site](https://doc.dpdk.org/guides/linux_gsg/linux_drivers.html).

## Tuning

Changing some Linux parameters, including disabling C-State, and P-State; and isolating CPUs can greatly benefit NF
performance. In addition to these boot-time settings, runtime settings (e.g., disabling uncore frequency scaling and
setting the appropriate flags for Linux power management QoS) can greatly improve performance. The
[energy.sh](scripts/tuning/energy.sh) in [scripts/tuning](scripts/tuning) will set these parameter appropriately, and
it is recommended you run this before running the system.
