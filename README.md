# munin-node-access

This is a quick project to give munin access to my website.
Since that's a public machine, we want munin-node to listen only to requests from our desktop IP.
But that IP changes whenever we reset the cable modem.
This project builds a command you can run on the remote machine to update its `munin-node.conf` with a new IP.


# Usage

`munin-node-access <ip>` - updates your `/etc/munin/munin-node.conf` with the given IP.

This script is extra simple so that you can give someone passwordless sudo access to it.

A previous version of this script ran locally and would ssh into the remote machine for you,
but that was less secure since it required root accesss.

It might be nice to accept a template filename as a parameter too, but for now we can just embed the template into the binary.

# TODO

- Accept a filename argument to provide a custom template. But maybe make it optional, and default to the compiled-in template.
- Add an `-n` or `--dry-run` option that generates the template and prints it to stdout, but doesn't upload it anywhere. That way you can test that you are getting a sane result.
- Add a second script that runs locally and obtains your IP then calls the remote script.
- Provide other ways for that second script to discover your ip.
