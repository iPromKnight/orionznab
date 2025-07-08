# Orionznab: Torznab provider for Orionoid

Orionznab is a Torznab provider designed to work with the Orionoid api.
It allows you to search and index content in a torznab-compatible format, making it easy to integrate with various media management systems.

## Requirements
* You require an Api key from Orionoid, which must be passed on all search requests (add it in prowlarr).

## Configuration via Env Variables

```yaml
# The useragent to use when fetching trailers.
# Optional, Defaults to 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/124.0.0.0'.
ORIONZNAB_USER_AGENT: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/124.0.0.0"
# Sets the internal rate limit for requests through the orionoid client.
# Optional, Defaults to '10/second'
ORIONZNAB_RATE_LIMIT: "10/second"
```
