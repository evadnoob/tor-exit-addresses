use docopt::*;

static USAGE: &'static str = "

get-tor-exit-addresses is a proxy to the published list of tor exit ip addresses.

Usage:
  get-tor-exit-addressesc --port=<port>
  get-tor-exit-addresses (-h | --help)

Options:
  -h --help                Show this screen.
  --version                Show version.
";


pub fn parse() -> ArgvMap {
    
   let args = Docopt::new(USAGE)
                      .and_then(|d| d.parse())
        .unwrap_or_else(|e| e.exit());

    return args
}
