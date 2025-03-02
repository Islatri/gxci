<p align="center" dir="auto">
    <img style="height:240px;width:240px"  src="https://s2.loli.net/2024/09/08/uDKESYW7ks9eRyf.png" alt="Logo逃走啦~"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">GXCI</h1>

<p align="center">
  <a href="https://crates.io/crates/gxci" target="_blank"><img src="https://img.shields.io/crates/v/gxci"/></a>
  <a href="https://docs.rs/gxci" target="_blank"><img src="https://img.shields.io/docsrs/gxci/0.3.7"/></a>
  <a href="https://github.com/islatri/gxci" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
</p>

<p align="center">
    Rust-based safe interface development for Daheng Industrial Camera GxIAPI
</p>

<hr />

# Now, the document site is available!

1. English: [https://hakochest.github.io/gxci-en/](https://hakochest.github.io/gxci-en/)
2. 中文: [https://hakochest.github.io/gxci-cn/](https://hakochest.github.io/gxci-cn/)

# Quick Start
1. Ensure you have OpenCV Rust Bindings installed, if not, you can see the [crates page's README](https://crates.io/crates/gxci)
2. Ensure your camera version is supported by the GxIAPI SDK,and ensure you have installed the GxIAPI SDK.

# HAL Part
There five main modules in the HAL: base, device, config, event and network. 

But until 0.3, the event and network module are not implemented.

# RAW Part
The RAW part in GXCI is all-safety, with LazyLock-Arc-Mutex-Option. And you can find the unsafe static mut implementation in the precious lib called [gxi_hako](https://crates.io/crates/gxi_hako), which is a deprecated RAW-only version of GXCI.

# Utils Part
Just the builder pattern and facade pattern, they are friendly to debug.

