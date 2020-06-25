# Fastly CDN setup

Used for all static assets.

Make sure to set interconnect location that's closest to storage (e.g. Amsterdam Fastly for Belgium Google)

Only needs simple redirect for each origin as a request condition, e.g. `req.http.host == "docs.jicloud.org"` for the docs bucket origin

See Fastly documentation for more details