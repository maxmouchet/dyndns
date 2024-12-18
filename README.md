# DynDNS

This is a simple dynamic DNS server that allows you to update A/AAAA/TXT records for a given domain.

Similar to services like [DuckDNS](https://www.duckdns.org/), but you can host it yourself. Currently, it only supports [Porkbun](https://porkbun.com/)'s backend for DNS updates. Future updates may include support for additional DNS providers.

```
Usage: dyndns [OPTIONS] --porkbun-api-key <PORKBUN_API_KEY> --porkbun-secret-key <PORKBUN_SECRET_KEY> --domain <DOMAIN>

Options:
      --porkbun-api-key <PORKBUN_API_KEY>        Porkbun API key
      --porkbun-secret-key <PORKBUN_SECRET_KEY>  Porkbun secret key
      --domain <DOMAIN>                          Domain
  -v, --verbose...                               Increase logging verbosity
  -q, --quiet...                                 Decrease logging verbosity
  -h, --help                                     Print help
  -V, --version                                  Print version
```