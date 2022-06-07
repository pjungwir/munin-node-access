# munin-node-access

This is a quick project to give munin access to my website.
Since that's a public machine, we want munin-node to listen only to requests from our desktop IP.
But that IP changes whenever we reset the cable modem.
This project gets our IP, sanity-checks that it looks like an IP, and then updates `munin-node.conf` on the box.

It might be nice to accept a template filename as a parameter too, but for now we can just embed the template into the binary.

# TODO

- Accept a filename argument to provide a custom template. But maybe make it optional, and default to the compiled-in template.
- Add an `-n` or `--dry-run` option that generates the template and prints it to stdout, but doesn't upload it anywhere. That way you can test that you are getting a sane result.
- Provide other ways to discover your ip. Also accept an `--ip` argument if you know it already or want to get it outside this script. But we still need to convert it to a regex.
