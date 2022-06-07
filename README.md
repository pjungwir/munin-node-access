# munin-node-access

This is a quick project to give munin access to my website.
Since that's a public machine, we want munin-node to listen only to requests from our desktop IP.
But that IP changes whenever we reset the cable modem.
This project gets our IP, sanity-checks that it looks like an IP, and then updates `munin-node.conf` on the box.

It might be nice to accept a template filename as a parameter too, but for now we can just embed the template into the binary.
