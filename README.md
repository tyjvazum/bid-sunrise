# bid-sunrise 🌅
a command-line tool to take top10milliondomains.csv and create 10k-reserved-domains.csv
=====

`bid-sunrise` is a command-line tool to take the top10milliondomains.csv file from DomCop's
[Open PageRank](https://www.domcop.com/openpagerank/what-is-openpagerank) initiative and
create the 10k-reserved-domains.csv file that serves as a list of reserved domains for the sunrise
period of the `bid` protocol for Bitcoin Identifiers/Usernames with `arb`.

It is experimental software, should be considered a work-in-progress, and has
no warranty. All features may not be fully implemented currently. See issues and
[LICENSE](LICENSE) for more details.

Join [the Gitter room](https://app.gitter.im/#/room/#arb-proto:gitter.im) to
chat about the `arb` ecosystem.

`bid` Protocol
------

- Characters can be alphanumeric with hyphens and underscores, lowercase a through z,
  0 through 9, - and _ in any combination.

- Length can be 1 through 16 characters, with 6 characters and shorter reserved
  for a future update, so 7 to 16 characters to start with.

- Usernames must be renewed periodically, likely every 52,500 blocks, which is
  about 1 year, but perhaps a shorter period initially to discourage speculation
  and encourage engagement.

- A [sunrise period](https://en.wikipedia.org/wiki/Sunrise_period) where a list of the top ten
  thousand domains are reserved, with the matching username claimable by publishing some specific
  data at a [well-known](https://en.wikipedia.org/wiki/Well-known_URI) location on the domain prior
  to the end of the sunrise period, which would be some specified block height.

- Claimed domains (e.g., example.tld) can choose between the formats example, exampletld, and
  example_tld, which allows domains like "simplywall.st" to have an appropriate representation.

Usage
------------

- Download and extract the [top10milliondomains.csv](https://www.domcop.com/files/top/top10milliondomains.csv.zip)
- Place the CSV file in the directory that contains the `Cargo.toml` file for `bid-sunrise`
- Run `cargo build`
- Run `./target/debug/bid-sunrise`
- Optionally compare the hash of the output `10k-reserved-domains.csv` to what the command printed
