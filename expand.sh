#!/bin/bash

set -e
mkdir -p temp

cargo expand --all-features --test mod validation::allowlist >> temp/allowlist.rs || true
cargo expand --all-features --test mod validation::blocklist >> temp/blocklist.rs || true
cargo expand --all-features --test mod validation::contains >> temp/contains.rs || true
cargo expand --all-features --test mod validation::email >> temp/email.rs || true
cargo expand --all-features --test mod validation::ip >> temp/ip.rs || true
cargo expand --all-features --test mod validation::ipv4 >> temp/ivp4.rs || true
cargo expand --all-features --test mod validation::ipv6 >> temp/ipv6.rs || true
cargo expand --all-features --test mod validation::length >> temp/length.rs || true
cargo expand --all-features --test mod validation::range >> temp/range.rs || true
cargo expand --all-features --test mod validation::prefix >> temp/prefix.rs || true
cargo expand --all-features --test mod validation::suffix >> temp/suffix.rs || true
cargo expand --all-features --test mod validation::url >> temp/url.rs || true
cargo expand --all-features --test mod validation::pattern >> temp/pattern.rs || true
cargo expand --all-features --test mod validation::option >> temp/option.rs || true
cargo expand --all-features --test mod validation::time >> temp/time.rs || true
cargo expand --all-features --test mod validation::naive_time >> temp/naive_time.rs || true
cargo expand --all-features --test mod validation::naive_date >> temp/naive_date.rs || true
cargo expand --all-features --test mod validation::after_now >> temp/after_now.rs || true
cargo expand --all-features --test mod validation::before_now >> temp/before_now.rs || true
cargo expand --all-features --test mod validation::now >> temp/now.rs || true
cargo expand --all-features --test mod validation::today >> temp/today.rs || true
cargo expand --all-features --test mod validation::after_today >> temp/after_today.rs || true
cargo expand --all-features --test mod validation::before_today >> temp/before_today.rs || true
cargo expand --all-features --test mod validation::inline >> temp/inline.rs || true
cargo expand --all-features --test mod validation::custom >> temp/custom.rs || true
cargo expand --all-features --test mod validation::async_custom >> temp/async_custom.rs || true
cargo expand --all-features --test mod validation::async_custom_with_context >> temp/async_custom_with_context.rs || true
cargo expand --all-features --test mod validation::custom_with_context >> temp/custom_with_context.rs || true

cargo check --all-features --test mod
