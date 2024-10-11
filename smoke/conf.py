import smoke.versions as smoke

products = [
    {
        "name": "smoke",
        "versions": smoke.versions,
    },
]

cache = [
    {
        "type": "registry",
        "ref_prefix": "build-repo.stackable.tech:8083/sandbox/cache",
        "mode": "max",
        "compression": "zstd",
        "ignore-error": "true",
    },
]
