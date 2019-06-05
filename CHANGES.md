# 0.3.0 (Unreleased)

- Eliminate the needs for `failure` crate directly. Even if the reason to expose error types is back, it is expected to be able to generalized as generics or associated items.
- Move the type parameter of `Component<M>`, that represents a data model of the component, to an associated item `Component::Model`.
- Remove redundant dependency on `tokio` crate.

# 0.2.0 (2019-06-05)

Initial release.

