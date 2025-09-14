# Release

To release a new set of actions follow these steps:

- Choose an appropriate new semantic version based on the latest changes
- Update any references in actions to other local actions to the chosen version
- Raise a PR with the appropriate changes
- Once merged, tag on `main` using `git tag -s v<X.Y.Z>`
- Manually create a GitHub release
