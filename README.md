# Call an API with Reqwest

It seems to be that Rust does not really offer a lot with regards to HTTP clients
_as part of the standard library_. That last part is key, don't @ me.

There are a few ubiquitous libraries to be found, such as [Hyper](https://crates.io/crates/hyper),
[curl-rust](https://crates.io/crates/curl) and [Reqwest](https://crates.io/crates/reqwest) keep cropping up.

From the above, Reqwest is the higher level one, which is perfect as I'm not that interested in learning the ins and outs of
these lower level APIs just yet.
