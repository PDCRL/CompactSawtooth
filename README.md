# CompactSawtooth
Enhanced Sawtooth node that works with Sawtooth docker images from docker hub.

To build the requirements to run a validator network, run this command:

  ```bash
    $ docker-compose build
  ```

This will build docker images suitable for running a validator, rest api,
settings transaction processor, intkey and xo python transaction processors,
and a client to interact with the network.


To run a full validator node::

```bash
  $ docker-compose up
```

This command starts a validator with the following components attached to it:

  - REST API (available on host port 8008)
  - IntKey transaction processor (Python implementation)
  - Settings transaction processor
  - XO transaction processor (Python implementation)
  - Shell (for running Sawtooth commands)

From another console window, you can access the shell with this command:

```bash
  $ docker-compose exec client bash
```
