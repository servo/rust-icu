# rust-icu

[Documentation](http://doc.servo.org/icu_sys/)

[![Build Status](https://travis-ci.org/servo/rust-icu.svg?branch=master)](https://travis-ci.org/servo/rust-icu)

This package contains Rust bindings for [ICU](http://site.icu-project.org/).
The `icu-sys` crate contains raw FFI bindings. By default it will use
pkg-config to search for the ICU libraries. If this fails or if the
`ICU_SYS_NO_PKG_CONFIG` environment variable is present, then it will build
ICU from source instead.

```
icu-sys = { git = "https://github.com/servo/rust-icu" }
```

The bindings currently cover only a subset of the libicu-uc library:

* uchar.h
* uscript.h

If you need bindings for features that are not yet included, please file an
issue or submit a pull request!
